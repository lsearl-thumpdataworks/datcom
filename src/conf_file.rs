/// 
/// File: conf_file.rs
/// Project: DATCON
/// File Created: Sunday, 4th February 2018 9:59:15 pm
/// Author: Leon S. Searl (lsearl@thumpdataworks.com)
/// -----
/// Last Modified: Sunday, 4th February 2018 9:59:52 pm
/// Modified By: Leon S. Searl (lsearl@thumpdataworks.com>)
/// -----
/// Copyright 2018 - 2018 , Thump Data Works, Inc.
/// All Rights Reserved.
/// -----
/// Description:
/// Structures and methods for parsing a toml_edit::Document file to determine
/// if it passes a syntax check specific to the file.
/// 
/// 

use std;
use toml_edit;
use std::io::Read;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigFileErrType {
    /// (name path to itme, the expected item type)
    WrongItemType(( Vec<String>, SpecItemType)),
    /// (name path to the item, the item name)
    MissingItem(( Vec<String>, String)),
    /// (name path to the item, the item name)
    ExtraItem((Vec<String>, String)),
    /// IO Error when trying to open file.
    IOError(String), // std::io::Error
    /// Error parsing the TOML doc.
    TOMLError(toml_edit::TomlError),
    /// A custom/add on error.
    CustomError(String)
}

impl Display for ConfigFileErrType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &ConfigFileErrType::WrongItemType((ref p, ref t)) => write!(f, "[{}]: Expected item type={}", SpecItem::to_item_path(&p), t),
            &ConfigFileErrType::MissingItem((ref p, ref s)) => write!(f, "[{}]: Missing item key={}", SpecItem::to_item_path(&p), s),
            &ConfigFileErrType::ExtraItem((ref p, ref s)) => write!(f, "[{}]: Extra item key='{}'", SpecItem::to_item_path(&p), s),
            &ConfigFileErrType::IOError(ref e) => write!(f, "File Open Error={}", e),
            &ConfigFileErrType::TOMLError(ref e) => write!(f, "{}", e),
            &ConfigFileErrType::CustomError(ref e) => write!(f, "{}", e)
        }
    }
}

//#########

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigFileErr {
    config_filename: String,
    /// The expected type for the item.
    err_type: ConfigFileErrType
}

impl ConfigFileErr {
    /// The entry in the confif file has the wrong type.
    fn spec_type_err(item_path: &Vec<String>, item_type: &SpecItemType) -> ConfigFileErr {
        ConfigFileErr{config_filename: String::from(""), err_type: ConfigFileErrType::WrongItemType( (item_path.clone(), item_type.clone()) )}
    }

    /// A required entry in the config file is missing.
    fn spec_missing(item_path: &Vec<String>, item_name: &str) -> ConfigFileErr {
        ConfigFileErr{config_filename: String::from(""), err_type: ConfigFileErrType::MissingItem((item_path.clone(), String::from(item_name)))}
    }

    /// An extra, unexpected item was found.
    fn spec_extra(item_path: &Vec<String>, item_name: &str) -> ConfigFileErr {
        ConfigFileErr{config_filename: String::from(""), err_type: ConfigFileErrType::ExtraItem(( item_path.clone(), String::from(item_name)))}
    }

    /// Change the configuration file name.
    fn set_config_filename(& mut self, new_name: &str) -> &ConfigFileErr{
        self.config_filename = String::from(new_name);
        self
    }

    /// There was an error opening the configuration file.
    fn io_error(config_filename: String, err: std::io::Error) -> ConfigFileErr {
        ConfigFileErr{config_filename: config_filename, err_type: ConfigFileErrType::IOError(format!("{}", err))}
    }

    /// A TOML syntax error
    fn toml_error(config_filename: String, err: toml_edit::TomlError) -> ConfigFileErr {
        ConfigFileErr{config_filename: config_filename, err_type: ConfigFileErrType::TOMLError(err)}
    }
    
    pub fn custom_error(config_filename: String, msg: String) -> ConfigFileErr {
        ConfigFileErr{config_filename: config_filename, err_type: ConfigFileErrType::CustomError(msg)}
    }

}

impl Display for ConfigFileErr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Config file='{}': {}", self.config_filename, self.err_type)
    }
}

impl std::error::Error for ConfigFileErr {
    fn description(&self) -> &str {
        match self.err_type {
            ConfigFileErrType::WrongItemType(( _, _)) => "Wrong item type",
            ConfigFileErrType::MissingItem((_, _)) => "Missing item",
            ConfigFileErrType::ExtraItem((_, _)) => "Extra item",
            ConfigFileErrType::IOError(_) => "File read error",
            ConfigFileErrType::TOMLError(_) => "TOML syntax error",
            ConfigFileErrType::CustomError(_) => "Custom Error"
        }
    }
}

//###############################################################

#[derive(Debug, Clone, PartialEq)]
/// Types of items from a configuration file.
pub enum SpecItemType {
    None,
    String,
    Integer,
    Float,
    /// Integer or Float
    Number,
    DateTime,
    Boolean,
    Array,
    InlineTable,
    Table,
    ArrayOfTables
}


impl Display for SpecItemType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            &SpecItemType::None => "None",
            &SpecItemType::String => "String",
            &SpecItemType::Integer => "Integer",
            &SpecItemType::Float => "Float",
            &SpecItemType::Number => "Integer|Float",
            &SpecItemType::DateTime => "DateTime",
            &SpecItemType::Boolean => "Boolean",
            &SpecItemType::Array => "Array",
            &SpecItemType::InlineTable => "InlineTable",
            &SpecItemType::Table => "Table",
            &SpecItemType::ArrayOfTables => "ArrayOfTables"
        })
    }
}

//#################

/// Represents the 'key', weather the item is optional, the type of the
/// item. And for a table type of item, the SpecItems for the tables contents.
pub struct SpecItem<'a> {
    pub name: &'a str,
    pub required: bool,
    pub item_type: SpecItemType,
    pub table: Option<&'a[SpecItem<'a>]>
}


impl<'a> SpecItem<'a> {

    /// Compare the value of the table item specifications agains the item values in the document.
    /// spec_items - The specified of the configuraiton file contents.
    /// doc - The parsed configuration file.
    /// return - None if there are no error.s
    ///          Somme all the error found
    pub fn doc_spec_check (spec_items: &[SpecItem], doc: &toml_edit::Document) -> Option<Vec<ConfigFileErr>> {
        // This vector is used to keep track of the path taken down through the spec.
        // If there is an error then it is converted into string that is the spec pathname of the
        // location of the eror.
        let mut name_path: Vec<String> = Vec::new();
        SpecItem::table_spec_check(spec_items, doc.as_table(), &mut name_path)
    }
 
    /// Convert a vector of names into a toml path.
    /// These must name put together, separated by a '.'.
    /// v - Vector of toml_edit names for a path to an item in a doc.
    fn to_item_path (v: & Vec<String>) -> String {
        let mut s = String::new();
        for ele in v {
            if !s.is_empty() {
                s.push('.');
            }
            s.push_str(&ele);
        };
        s
    }

    /// Compare the value of the table item specifications agains the item values in the document.
    /// spec_items - An array of SpecItem.
    /// table - toml_edit table to check agains the spec_items for the table
    /// name_path - The toml_edit name getting to this point.
    /// Return - 
    ///      None - there were no errors.
    ///      Somm - There were errors.
    fn table_spec_check (spec_items: &'a [SpecItem], table: &toml_edit::Table,
                         name_path: & mut Vec<String>) -> Option<Vec<ConfigFileErr>> {

        let mut errors = Vec::new();

        // Check that the types are all correct.
        for spec in spec_items {
            name_path.push(spec.name.to_owned());
            let mut item_found = false;
            // See if the spec and item match.
            // Do the names match?
            for (name, item) in table.iter() {
            
                if spec.name == name  {
                    // We found a match between item and spec.
                    // See if the type is correct.
                    match spec.item_type {
                        SpecItemType::None => 
                            if !item.is_none() {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            },
                        SpecItemType::String =>
                            if !item.is_str() {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            },
                        SpecItemType::Integer => 
                            if !item.is_integer() {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            },
                        SpecItemType::Float => 
                            if !item.is_float() {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            },
                        SpecItemType::Number =>
                            if !(item.is_float() || item.is_integer()) {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            },
                        SpecItemType::DateTime => 
                            if !item.is_date_time() {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            },
                        SpecItemType::Boolean => 
                            if !item.is_bool() {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            },
                        SpecItemType::Array => 
                            if !item.is_array() {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            },
                        SpecItemType::InlineTable => {
                                if !item.is_inline_table() {
                                    errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                    ()
                                }
                                // TODO: Implement inline_table
                                unimplemented!()
                            },
                        SpecItemType::Table => {
                                if !item.is_table() {
                                    errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                    ()
                                }
                                else {
                                    // Check that the table is correct.
                                    let spec_table = match spec.table {
                                        Some(t) => t,
                                        None => panic!("When a SpecItem.item_type is SpecItemType::Table, the SpecItem.table must not be SpecItemType::None.")
                                    };
                                    let result = SpecItem::table_spec_check(spec_table, item.as_table().unwrap(), name_path );
                                    match result {
                                        None => (),
                                        Some(mut err) => {
                                            errors.append(& mut err);
                                            ()
                                        }
                                    }
                                }
                            },
                        SpecItemType::ArrayOfTables => {
                            if !item.is_array_of_tables() {
                                errors.push(ConfigFileErr::spec_type_err(name_path, &spec.item_type));
                                ()
                            }
                            else {
                                let mut i = 0;
                                for ele in item.as_array_of_tables().unwrap().iter() {
                                    let spec_table = match spec.table {
                                        Some(t) => t,
                                        None => panic!("When a SpecItem.item_type is SpecItemType::Table, the SpecItem.table must not be SpecItemType::None.")
                                    };
                                    
                                    name_path.push(format!("[{}]", i));
                                    let result = SpecItem::table_spec_check(&spec_table, ele, name_path);
                                    name_path.pop();
                                    i = i + 1;                                    
                                    match result {
                                       None => (),
                                       Some(mut err) => {
                                           errors.append(&mut err);
                                           ()
                                       }
                                    }
                                   
                                }
                            }
                        }
                    }
                    
                    item_found = true;
                    break;
                }
            }

            
            // Clean up
            name_path.pop();

            // Make sure that required specs are found in the doc.
            if spec.required && item_found == false {
                errors.push(ConfigFileErr::spec_missing(name_path, spec.name))
            }

        }

        // Make sure that there are no extra items in the doc.
        for (name, _) in table.iter() {
            // For each item.
            let mut spec_found = false;
            for spec in spec_items {
                if name == spec.name {
                    spec_found = true;
                    break;
                }
            }

            if !spec_found {
                errors.push(ConfigFileErr::spec_extra(name_path, name));
            }
        }

        // See if we return an error.
        if errors.len() == 0  {
            None
        }
        else {
            Some(errors)
        }
    }
}

//#############################


/// Parse the  configuration file and return the configuration.
/// config_spec - A fillout out configuration file speciification
/// config_path - file path containing the configuration file.
/// Return - 
///     Ok() - The tomel_edit document successfull parsed for syntax and key/table erros.
///     Err
pub fn read(config_spec: &[SpecItem], config_path: &str  ) -> Result<toml_edit::Document, Vec<ConfigFileErr>> {
        // Use this string at the front of any error messages.
        let mut errors: Vec<ConfigFileErr> = Vec::new();

        // open the config file to read
        let mut config_file = match std::fs::OpenOptions::new()
                                .create(false)
                                .read(true)
                                .write(false)
                                .open(config_path) {
            Ok(file) => file,
            Err(e) =>  {
                errors.push(ConfigFileErr::io_error(String::from(config_path), e));
                return Err(errors)
            }
            
        };

        // Read the configuration file into a 'doc'.
        let mut config_str = String::new();
        match config_file.read_to_string(&mut config_str) {
            Ok(_) => (),
            Err(e) => {
                errors.push(ConfigFileErr::io_error(String::from(config_path), e));
                return Err(errors)
            }
        };
        let toml_doc = match config_str.parse::<toml_edit::Document>() {
                Ok(doc) => doc,
                Err(e) => {
                    errors.push(ConfigFileErr::toml_error(String::from(config_path), e));
                    return Err(errors)
                }
        };

        // Now check that the configuration file was set up correctly
        match SpecItem::doc_spec_check(config_spec, &toml_doc) {
            None => Ok(toml_doc),
            Some(mut err) => {
                for e in err.iter_mut() {
                    e.set_config_filename(config_path);
                }

                return Err(err)
            }
        }
        
}


//###############################################################################
// Tests
//###############################################################################
#[cfg(test)]
mod tests {

    use super::*;

    //const TABLE_VERSION_ONLY: SpecItem =  ;
 
    // Empty Config spec.
    
    
    const CONFIG_EMPTY: [SpecItem; 0] = [  ];

    const CONFIG_VERSION_ONLY: [SpecItem; 1] = [ 
        SpecItem {
            name: "version", required: true, item_type: SpecItemType::String, table: None
        }
    ];

    const CONFIG_COMM_DEVICE: [SpecItem; 4] = [ 
        SpecItem {
            name: "Device", required: true, item_type: SpecItemType::String, table: None
        },
        SpecItem {
            name: "CommPlugin", required: true, item_type: SpecItemType::String, table: None
        },
        SpecItem {
            name: "CommConfigFile", required: true, item_type: SpecItemType::String, table: None
        },
        SpecItem {
            name: "MeasConfigFile", required: true, item_type: SpecItemType::String, table: None
        }
    ];

    const CONFIG_SENSOR_FILTER_APP: [SpecItem; 4] = [
        SpecItem {
            name: "FormatVersion", required: true, item_type: SpecItemType::String, table: None
        },
        SpecItem {
            name: "ServiceDiscoveryName", required: true, item_type: SpecItemType::String, table: None
        },
        SpecItem {
            name: "MeasReportPlugin", required: true, item_type: SpecItemType::String, table: None
        },
        SpecItem {
            name: "CommBusDevice", required: true, item_type: SpecItemType::ArrayOfTables, 
            table: Some(&CONFIG_COMM_DEVICE)
        }

    ];


    #[test]
    // Empty config spec should work with empty config file.
    fn check_configfile_empty() {
        let conf = r#"
        "#;
        let result = conf.parse::<toml_edit::Document>();
        match result {
            Ok(doc) => 
                match SpecItem::doc_spec_check(&CONFIG_EMPTY, &doc) {
                    None => (),
                    Some(e) => panic!(format!("Unexpected spec check error: {:?}", e))
                },
            Err(e) => panic!(format!("Syntax error in conf: {}", e))
        }
    }

    //####################################

    #[test]
    #[should_panic]
    // Empty config file should fail with version config file.
    fn check_configfile_empty_against_version_spec() {
        let conf = r#"
        "#;
        let result = conf.parse::<toml_edit::Document>();
        match result {
            Ok(doc) => 
                match SpecItem::doc_spec_check(&CONFIG_VERSION_ONLY, &doc) {
                    None => (),
                    Some(e) => panic!(format!("{:?}", e))
                },
            Err(e) => panic!(format!("Syntax error in conf: {}", e))
        }
    }

    //####################################

    
     #[test]
    // version config spec should work with version config file.
    fn check_configfile_version_only() {
        let conf = r#"
        # Comment
        version = "0.0.0"
        "#;
        let result = conf.parse::<toml_edit::Document>();
        match result {
            Ok(doc) => 
                match SpecItem::doc_spec_check(&CONFIG_VERSION_ONLY, &doc) {
                    None => (),
                    Some(e) => panic!(format!("Unexpected config spec check error: {:?}", e))
                },
            Err(e) => panic!(format!("Syntax error in conf file: {}", e))
        }
    }

    //####################################

    #[test]
    #[should_panic]
    // Should get error because conf file is not empty..
    fn check_configfile_version_against_empty_spec() {
        let conf = r#"
        # Comment
        version = "0.0.0"
        "#;
        let result = conf.parse::<toml_edit::Document>();
        match result {
            Ok(doc) => 
                match SpecItem::doc_spec_check(&CONFIG_EMPTY, &doc) {
                    None => (),
                    Some(e) => panic!(format!("{:?}", e))
                },
            Err(e) => panic!(format!("Syntax error in conf file: {}", e))
        }
    }

    //####################################

    
     #[test]
     #[should_panic]
    // See if the parser detects a syntax problem.
    fn check_configfile_syntax_problem() {
        let conf = r#"
        # Comment
        version =
        "#;
        let result = conf.parse::<toml_edit::Document>();
        match result {
            Ok(doc) => 
                match SpecItem::doc_spec_check(&CONFIG_VERSION_ONLY, &doc) {
                    None => (),
                    Some(e) => panic!(format!("Unexpected config spec check error: {:?}", e))
                },
            Err(e) => panic!(format!("Syntax error in conf file: {}", e))
        }
    }

    //####################################

    
     #[test]
    // See if the parser detects a syntax problem.
    fn check_sensor_filter_app_config() {
        let conf = r#"
        # Configuration file for hmi_sensor_filter
        FormatVersion = "0.0.0"
        ServiceDiscoveryName = "sensor_filter"
        MeasReportPlugin = "meas_1" # This will be relative

        [[CommBusDevice]]
            Device = "can0"
            CommPlugin = "flexcan_comm_1"
            CommConfigFile = "can0_comm_config"
            MeasConfigFile = "can0_meas_config"

        "#;
        let result = conf.parse::<toml_edit::Document>();
        match result {
            Ok(doc) => 
                match SpecItem::doc_spec_check(&CONFIG_SENSOR_FILTER_APP, &doc) {
                    None => (),
                    Some(e) => panic!(format!("Unexpected config spec check error: {:?}", e))
                },
            Err(e) => panic!(format!("Syntax error in conf file: {}", e))
        }
    }

}
