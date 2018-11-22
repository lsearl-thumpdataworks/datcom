/// -----
/// File: /src/case/mod.rs  
/// Project: datcon  
/// File Created: Tuesday, 13th Nov 182018 5:32:15 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Tuesday, 13th November 2018 5:32:20 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----

/// The tom_edit specs and struct for handling a DATCON 'case' portion of the input file. 

pub mod fltcon;
pub mod optins;
pub mod synths;
pub mod body;
pub mod xxplnf;
pub mod xxschr;
pub mod propwr;
pub mod grndef;
pub mod tvtpan;
pub mod symflp;
pub mod asyflp;
pub mod errors;

use conf_file;
use toml_edit;
use std::fmt::{Display, Formatter};
use std;


/* GROUP IV Names */
// TODO: Use it
const INPUT_NAME_CTRLCARD_FLAGS:  &'static str = "CONTROL";
const INPUT_NAME_CTRLCARD_NACA: &'static str = "NACA";
const INPUT_NAME_CTRLCARD_CASEID: &'static str = "CASEID";
const INPUT_NAME_CTRLCARD_DIM: &'static str = "DIM";

// TODO Use It
//const INPUT_NAME_CTRLCARD_NAMELIST: &'static str = "NAMELIST";
//const INPUT_NAME_CTRLCARD_SAVE: &'static str = "SAVE";
//const INPUT_NAME_CTRLCARD_TRIM: &'static str = "TRIM";
//const INPUT_NAME_CTRLCARD_DAMP: &'static str = "DAMP";
//const INPUT_NAME_CTRLCARD_DUMP: &'static str = "DUMP";
//const INPUT_NAME_CTRLCARD_DERIV: &'static str = "DERIV";
//const INPUT_NAME_CTRLCARD_PART: &'static str = "PART";
//const INPUT_NAME_CTRLCARD_BUILD: &'static str = "BUILD";
//const INPUT_NAME_CTRLCARD_PLOT: &'static str = "PLOT";

// ##########################################

/// Type of flying surfaces this is for (wing, horz tail, vert tail, vent fin)
const INPUT_NAME_NACA_PLANFORM: &'static str = "PLANFORM";
/// The type of NACA series (4digit, 5digit, 6series, 1series, supersonic)
const INPUT_NAME_NACA_SERIES: &'static str = "SERIES";
/// The NACA series designatation
const INPUT_NAME_NACA_DESIGN: &'static str = "DESIGN";

const CASE_INPUT_NACA_TABLE: [conf_file::SpecItem<'static>; 3] = [
    conf_file::SpecItem {
        name: INPUT_NAME_NACA_PLANFORM, required: true, item_type: conf_file::SpecItemType::String,
        table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_NACA_SERIES, required: true, item_type: conf_file::SpecItemType::String,
        table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_NACA_DESIGN, required: true, item_type: conf_file::SpecItemType::String,
        table: None
    }
];

const CASE_INPUT_NACA_ARRAYTABLE: conf_file::SpecItem<'static> =
    conf_file::SpecItem {
        name: INPUT_NAME_CTRLCARD_NACA, required: false, 
        item_type: conf_file::SpecItemType::ArrayOfTables, table: Some(&CASE_INPUT_NACA_TABLE)
    };

// ############################################

pub const INPUT_NAME_CASE: &'static str = "CASE";




/// The top level is an array of 'case' tables.
const  CASE_INPUT_TABLE: [conf_file::SpecItem<'static>; 21] = [
    conf_file::SpecItem {
        name: INPUT_NAME_CTRLCARD_FLAGS, required: false, item_type: conf_file::SpecItemType::Array,
        table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_CTRLCARD_CASEID, required: false, item_type: conf_file::SpecItemType::String,
        table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_CTRLCARD_DIM, required: false, item_type: conf_file::SpecItemType::String,
        table: None
    },
    CASE_INPUT_NACA_ARRAYTABLE,

    fltcon::INPUT_SPEC_FLTCON_ITEM,
    optins::INPUT_SPEC_OPTINS_ITEM,
    synths::INPUT_SPEC_SYNTHS_ITEM,
    body::INPUT_SPEC_BODY_ITEM,
    xxplnf::INPUT_SPEC_WGPLNF_ITEM,
    xxplnf::INPUT_SPEC_HTPLNF_ITEM,
    xxplnf::INPUT_SPEC_VTPLNF_ITEM,
    xxplnf::INPUT_SPEC_VFPLNF_ITEM,
    xxschr::INPUT_SPEC_WGSCHR_ITEM,
    xxschr::INPUT_SPEC_HTSCHR_ITEM,
    xxschr::INPUT_SPEC_VTSCHR_ITEM,
    xxschr::INPUT_SPEC_VTSCHR_ITEM,
    propwr::INPUT_SPEC_PROPWR_ITEM,
    grndef::INPUT_SPEC_GRNDEF_ITEM,
    tvtpan::INPUT_SPEC_TVTPAN_ITEM,
    symflp::INPUT_SPEC_SYMFLP_ITEM,
    asyflp::INPUT_SPEC_ASYFLP_ITEM
];

///
/// This TOML spec is array of tables 'case' item table...
/// This is the top level of the TOML spec.
pub const INPUT_SPEC_CASE_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_CASE, required: true, item_type: conf_file::SpecItemType::ArrayOfTables,
        table: Some(&CASE_INPUT_TABLE)
};

// ###########################################################################################


/** the GroupIV controls that do not have arguments. */
pub enum Flags {
    NextCase,
    Save,
    Trim,
    Damp,
    DumpCase,
    DumpInput,
    DumpIom,
    DumpAll,
    Part,
    Build,
    Plot
}

// ######################################################################

/** Dimentions for use in the application. */
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Dimensions {
    /** Dimentions of Feet. Default*/
    FT,
    /** Dimentions of Inchs. */
    IN, 
    /** Dimentions of meters. */
    M,
    /** Dimentions of centimeters. */
    CM
}

const DIMENSIONS_NAME_FT: &'static str = "FT";
const DIMENSIONS_NAME_IN: &'static str = "IN";
const DIMENSIONS_NAME_M: &'static str = "M";
const DIMENSIONS_NAME_CM: &'static str = "CM";

impl Dimensions {
    /// Create a string containing all the dim strings names.
    pub fn enums_string () -> String {
        format!("{}, {}, {}, {}", DIMENSIONS_NAME_FT, DIMENSIONS_NAME_IN, DIMENSIONS_NAME_M,
                                DIMENSIONS_NAME_CM)
    }

    /// Create the name string for the enum.
    pub fn name (&self) -> String {
        match self {
            &Dimensions::FT => String::from(DIMENSIONS_NAME_FT),
            &Dimensions::IN => String::from(DIMENSIONS_NAME_IN),
            &Dimensions::M => String::from(DIMENSIONS_NAME_M),
            &Dimensions::CM => String::from(DIMENSIONS_NAME_CM)
        }
    }

    ///
    pub fn parse (dim_item: &toml_edit::Item, case_error_id: &String, item_name: &str,
                        errs_sofar: &mut Vec<errors::CaseCheckErr>) -> Option<Dimensions> {
        // Parse the DIM card control
        match dim_item.as_value() {
            None => Some(Dimensions::FT),
            Some(dim_str) =>
                match dim_str.as_str().unwrap() {
                    DIMENSIONS_NAME_FT => Some(Dimensions::FT),
                    DIMENSIONS_NAME_IN => Some(Dimensions::IN),
                    DIMENSIONS_NAME_CM => Some(Dimensions::CM),
                    DIMENSIONS_NAME_M => Some(Dimensions::M),
                    val @ _ => {
                        errs_sofar.push(
                                errors::CaseCheckErr::ValueErr(
                                    case_error_id.clone(),
                                    String::from("None"),
                                    String::from(item_name),
                                    errors::ValueInputErr::BadEnumErr(String::from(val), Dimensions::enums_string())));
                            Some(Dimensions::FT)
                    }
                }
        }

    }

}

// ###################################################
// NACA airfoil information
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Planform {
    /// Wing, canard
    Wing,
    /// Horzontal Tail, aft wing
    HorzTail,
    /// Vettical Tail
    VertTail,
    /// Ventral Fin
    VentFin
}

const PLANFORM_NAME_WING: &'static str = "WING";
const PLANFORM_NAME_HORZTAIL: &'static str = "HORZTAIL";
const PLANFORM_NAME_VERTTAIL: &'static str = "VERTTAIL";
const PLANFORM_NAME_VENTFIN: &'static str = "VENTFIN";

impl Planform {
    /// Create a string containing all the emum strings names.
    pub fn enums_string () -> String {
        format!("{}, {}, {}, {}", PLANFORM_NAME_WING, PLANFORM_NAME_HORZTAIL,
                        PLANFORM_NAME_VERTTAIL, PLANFORM_NAME_VENTFIN)
    }
    
    /// Create the name string for the enum.
    pub fn name (&self) -> String {
        match self {
            &Planform::Wing => String::from(PLANFORM_NAME_WING),
            &Planform::HorzTail => String::from(PLANFORM_NAME_HORZTAIL),
            &Planform::VertTail => String::from(PLANFORM_NAME_VERTTAIL),
            &Planform::VentFin => String::from(PLANFORM_NAME_VENTFIN)
        }
    }

    /// Extract the Planform from the toml naca table.
    /// * naca_table - a naca table type
    /// * namelist - the table name
    /// case_error_id - the caseid
    /// errs_sofar - list of errors to ad new errors to.
    pub fn parse (naca_table: &toml_edit::Table, case_error_id: &String, namelist: &str, 
                        errs_sofar: &mut Vec<errors::CaseCheckErr>) 
                -> Option<Planform> {
        //let mut errs: Vec<errors::CaseCheckErr>= Vec::new();
        let planform_name = naca_table[INPUT_NAME_NACA_PLANFORM].as_str().unwrap(); // it was required
        match planform_name {
            PLANFORM_NAME_WING => Some(Planform::Wing),
            PLANFORM_NAME_HORZTAIL => Some(Planform::HorzTail),
            PLANFORM_NAME_VERTTAIL => Some(Planform::VertTail),
            PLANFORM_NAME_VENTFIN => Some(Planform::VentFin),
            val @ _ => {
                // Put an error into the error list
                errs_sofar.push( errors::CaseCheckErr::ValueErr(case_error_id.clone(),
                                    String::from(namelist),
                                    String::from(INPUT_NAME_NACA_PLANFORM),
                                    errors::ValueInputErr::BadEnumErr(String::from(val), Planform::enums_string())));
                
                None
            }
        }
    }
}

// #####
#[derive(Debug)]
#[derive(PartialEq)]
pub enum NacaSeries {
    OneSeries,
    FourDigit,
    FourDigitModified, // not an input option, determined later
    FiveDigit,
    FiveDigitModified, // not an input option, determined later
    SixSeries,
    Supersonic
}

const NACA_SERIES_NAME_ONESERIES: &'static str = "1SERIES";
const NACA_SERIES_NAME_FOURDIGIT: &'static str = "4DIGIT";
const NACA_SERIES_NAME_FIVEDIGIT: &'static str = "5DIGIT";
const NACA_SERIES_NAME_SIXSERIES: &'static str = "6SERIES";
const NACA_SERIES_NAME_SUPERSONIC: &'static str = "SUPERSONIC";

impl NacaSeries {
    pub fn enums_string() -> String {
        format!("{}, {}, {}, {}, {}", NACA_SERIES_NAME_ONESERIES, 
                NACA_SERIES_NAME_FOURDIGIT, NACA_SERIES_NAME_FOURDIGIT,
                NACA_SERIES_NAME_SIXSERIES, NACA_SERIES_NAME_SUPERSONIC)
    }

    pub fn name (&self) -> String {
        match self {
            &NacaSeries::OneSeries => String::from("1SERIES"),
            &NacaSeries::FourDigit => String::from("4DIGIT"),
            &NacaSeries::FourDigitModified => String::from("4DIGIT_MODIFIED"),
            &NacaSeries::FiveDigit => String::from("5DIGIT"),
            &NacaSeries::FiveDigitModified => String::from("5DIGIT_MODIFIED"),
            &NacaSeries::SixSeries => String::from("6SERIES"),
            &NacaSeries::Supersonic => String::from("SUPERSONIC")
        }
    }

    /// Extract the NACA series from the toml naca table.
    /// * naca_table - a naca table type
    /// * namelist - the table name
    /// case_error_id - the caseid
    /// errs_sofar - list of errors to ad new errors to.
    pub fn parse (naca_table: &toml_edit::Table, case_error_id: &String, namelist: &str, 
                        errs_sofar: &mut Vec<errors::CaseCheckErr>) 
                -> Option<NacaSeries> {
        //let mut errs: Vec<errors::CaseCheckErr>= Vec::new();
        let series_name = naca_table[INPUT_NAME_NACA_SERIES].as_str().unwrap(); // it was required
        match series_name {
            NACA_SERIES_NAME_ONESERIES => Some(NacaSeries::OneSeries),
            NACA_SERIES_NAME_FOURDIGIT => Some(NacaSeries::FourDigit),
            NACA_SERIES_NAME_FIVEDIGIT => Some(NacaSeries::FiveDigit),
            NACA_SERIES_NAME_SIXSERIES => Some(NacaSeries::SixSeries),
            NACA_SERIES_NAME_SUPERSONIC => Some(NacaSeries::Supersonic),
            val @ _ => {
                // Put an error into the error list
                errs_sofar.push( errors::CaseCheckErr::ValueErr(case_error_id.clone(),
                                    String::from(namelist),
                                    String::from(INPUT_NAME_NACA_SERIES),
                                    errors::ValueInputErr::BadEnumErr(String::from(val), NacaSeries::enums_string())));
                
                None
            }
        }
    }
}

pub struct NacaCard {
    pub planform: Planform,
    pub series: NacaSeries,
    pub design: String
}

impl NacaCard {
    pub fn parse (naca_array: &toml_edit::ArrayOfTables, case_error_id: &String,
                             naca_name: &str, errs_sofar: &mut Vec<errors::CaseCheckErr>)
             -> Option<Vec<NacaCard>> {
        let mut errs: Vec<errors::CaseCheckErr> = Vec::new(); 
        let mut cards: Vec<NacaCard> = Vec::new();
        for naca_table in naca_array.iter() {
            let planform_opt = Planform::parse (naca_table,  case_error_id, naca_name, &mut errs);
            let series_opt = NacaSeries::parse(naca_table, case_error_id, naca_name, &mut errs);
            let design = String::from(naca_table[INPUT_NAME_NACA_DESIGN].as_str().unwrap());
            
            if planform_opt.is_some() && series_opt.is_some() {
                cards.push(NacaCard{ planform: planform_opt.unwrap(), 
                                    series: series_opt.unwrap(),
                                    design: design });
            }
        }

        // Check for duplicate planforms
        if cards.len() > 1 {
            for i in 0..cards.len()-1 {
                for j in i+1..cards.len() {
                    if cards[i].planform == cards[j].planform {
                        errs.push(errors::CaseCheckErr::ValueErr(case_error_id.clone(), 
                            String::from(naca_name), 
                            String::from(INPUT_NAME_NACA_PLANFORM), 
                            errors::ValueInputErr::DuplicateDataErr(cards[i].planform.name())))
                    }
                }
            }
        }

        if errs.len() > 0 {
            errs_sofar.append(&mut errs)
        }
        if cards.len() > 0 {
            Some(cards)
        }
        else { None }
    }
}

//#######################

/** The nameslists and variables for GroupI. */
pub struct Case {
    pub flags: Option<Flags>,
    pub dim: Option<Dimensions>,
    pub dump: Option<Vec<String>>,
    pub caseid: Option<String>,
    pub naca: Option<Vec<NacaCard>>,
    pub fltcon: Option<fltcon::Fltcon>,
    pub optins: Option<optins::Optins>,
    pub body: Option<body::Body>,
    pub wgplnf: Option<xxplnf::Wgplnf>,
    pub htplnf: Option<xxplnf::Htplnf>,
    pub vtplnf: Option<xxplnf::Vxplnf>,
    pub vfplnf: Option<xxplnf::Vxplnf>,
    pub wgschr: Option<xxschr::Wgschr>,
    pub htschr: Option<xxschr::Htschr>,
    pub vtschr: Option<xxschr::Vxschr>,
    pub vfschr: Option<xxschr::Vxschr>,
    pub propwr: Option<propwr::Propwr>,
    pub grndef: Option<grndef::Grndef>,
    pub tvtpan: Option<tvtpan::Tvtpan>,
    pub symflp: Option<symflp::Symflp>,
    pub asyflp: Option<asyflp::Asyflp>
}

impl Case {
    ///
    /// Write human readable CASE info.
    pub fn write_human(a_case: & Case, f: & mut Formatter) {
        match write!(f, "CASE:") {
            Ok(_) => (),
            Err(_) => ()
        }
        match &a_case.caseid {
            &None => (),
            &Some(ref id) => {
                match write!(f, "  CASEID={}", id) {
                    Ok(_) => (),
                    Err(_) => ()
                }
            }
        }
    }

    /// 
    /// Find and error check the namelists (and their variables) and elements of the a 'case' table.
    /// * case_table - a toml table of our 'case' spec type.
    /// * errs_sofar - the errors so far. New errors will be appended to this vector.
    /// * return - the case struct. If there were erros it is possible this not all of this
    /// case will be filled out and some values may have been substituded if they were out of
    /// range.
    pub fn check_case (case_table: &toml_edit::Table, default_case_id: &String, 
                        errs_sofar: &mut Vec<errors::CaseCheckErr>) -> Self {
        // Array to accumulate errors into.
        let mut errs: Vec<errors::CaseCheckErr> = Vec::new();

        // Look for the CASEID.
        let caseid = match case_table[INPUT_NAME_CTRLCARD_CASEID].as_value() {
            None => None,
            Some(value) => Some(String::from(value.as_str().unwrap()))
        };

        let case_error_id = match &caseid {
            &None => default_case_id.clone(),
            &Some(ref id) => id.clone() 
        };

        let dim = Dimensions::parse(&case_table[INPUT_NAME_CTRLCARD_DIM], &case_error_id,
                                        &INPUT_NAME_CTRLCARD_CASEID, &mut errs);

        // Parse the NACA control card.
        let naca_opt = match case_table[INPUT_NAME_CTRLCARD_NACA].as_array_of_tables() {
            None => None,
            Some(array) => NacaCard::parse(array, &case_error_id, INPUT_NAME_CTRLCARD_NACA, &mut errs)
        };


        errs_sofar.append(&mut errs);

        Case {flags: None, dim: dim, dump: None, caseid: caseid, naca: naca_opt,
                    fltcon: None, optins: None, body: None, wgplnf: None,
                    htplnf: None, vtplnf: None, vfplnf: None, wgschr: None,
                    htschr: None, vtschr: None, vfschr: None, propwr: None,
                    grndef: None, tvtpan: None, symflp: None, asyflp: None}
    }

    /// Top level function for parseing the configuration/input file tomp_doc.
    /// Use the conf_file::rea
    pub fn get_input_data(doc: &toml_edit::Document) -> Result<Vec<Case>, Vec<errors::CaseCheckErr>> {
        // Get the array of case tables.
        let mut all_cases: Vec<Case> = Vec::new();
        let mut errs: Vec<errors::CaseCheckErr> = Vec::new();
        match doc[INPUT_NAME_CASE].as_array_of_tables() {
            Some(tables) => {
                let mut caseid = 0;
                for a_table in tables.iter() {
                    let a_case = Case::check_case (&a_table, &format!("UnkCase{}", caseid), &mut errs);
                    all_cases.push(a_case);
                    caseid = caseid + 1;
                };
            },
            None => ()
        }
        if errs.len() == 0 {
            Ok(all_cases)
        }
        else {
            Err(errs)
        }
    }


}

// ##############################################################################
#[derive(Debug)]
#[derive(PartialEq)]
pub enum FileInputErr {
    ConfigFileErr(conf_file::ConfigFileErr),
    CaseInputErr(errors::CaseCheckErr)
}

impl Display for FileInputErr {
    fn fmt (&self, f: & mut Formatter) -> std::fmt::Result {
        match self {
            &FileInputErr::ConfigFileErr(ref conf_file_err) =>
                write!(f, "{}", conf_file_err),
            &FileInputErr::CaseInputErr(ref case_check_err) =>
                write!(f, "{}", case_check_err),
        }
    }
}

/// Parese the input file and convert it to structures.
/// * return
/// ** Ok - the cases from the input file.
/// ** Err - fatal errors either reading file or converting to the structures.
pub fn read_config(filepath: &std::path::Path) -> Result<Vec<Case>,Vec<FileInputErr>>  {
    
    let item = [INPUT_SPEC_CASE_ITEM];

    // Read the file to get the toml_doc.
    let filestr = filepath.to_str().unwrap();
    let toml_doc = match conf_file::read(&item, filestr) {
        Ok(doc) => doc,
        Err(e_v) => {
            let mut errs: Vec<FileInputErr> = Vec::new();
            for err in e_v {
                errs.push(FileInputErr::ConfigFileErr(err));
            } 
            return Err(errs);
        }
    };
    
    let cases = match Case::get_input_data(&toml_doc) {
        Ok(c) => c,
        Err(e_v) => {
            let mut errs: Vec<FileInputErr> = Vec::new();
            for err in e_v {
                errs.push(FileInputErr::CaseInputErr(err));
            } 
            return Err(errs);
        }
    };
    Ok(cases)
} 



// ##################################################################################

#[cfg(test)]

mod tests {
    
    use toml_edit;
    use case;
    use std;

    #[test]
    fn empty_toml() {
        // Need to construct the TOML
        let empty_toml = toml_edit::Table::new();
        let caseid = String::from("0");
        let mut errs: Vec<case::errors::CaseCheckErr>  = Vec::new();
        let a_case = case::Case::check_case(&empty_toml, &caseid, &mut errs);

        assert!(errs.len() == 0);
        assert!(a_case.caseid.is_none());
        assert!( ! a_case.dim.is_none());
        assert!(a_case.dump.is_none());
        assert!(a_case.flags.is_none());
        assert!(a_case.naca.is_none());
        assert!(a_case.fltcon.is_none());
        assert!(a_case.optins.is_none());
        assert!(a_case.body.is_none());
        assert!(a_case.wgplnf.is_none());
        assert!(a_case.htplnf.is_none());
        assert!(a_case.vtplnf.is_none());
        assert!(a_case.vfplnf.is_none());
        assert!(a_case.wgschr.is_none());
        assert!(a_case.htschr.is_none());
        assert!(a_case.vtschr.is_none());
        assert!(a_case.vfschr.is_none());
        assert!(a_case.propwr.is_none());
        assert!(a_case.grndef.is_none());
        assert!(a_case.tvtpan.is_none());
        assert!(a_case.symflp.is_none());
        assert!(a_case.asyflp.is_none());
     }

    #[test]
    fn has_caseid() {
        // Need to construct the TOML
        let mut case_toml = toml_edit::Table::new();
        // Put the dimension into the doc.
        let id = "testid";
        case_toml[case::INPUT_NAME_CTRLCARD_CASEID] = toml_edit::value(id);
        
        let caseid = String::from("0");
        let mut errs: Vec<case::errors::CaseCheckErr>  = Vec::new();
        let a_case = case::Case::check_case(&case_toml, &caseid, &mut errs);

        assert!(errs.len() == 0);
        assert!(a_case.caseid.is_some());
        assert!(a_case.caseid.unwrap() == id );
     }
    
    #[test]
    // Make sure that the default dimention FT works.
    fn has_default_dim() {
        // Need to construct the TOML
        let case_toml = toml_edit::Table::new();
        
        let caseid = String::from("0");
        let mut errs: Vec<case::errors::CaseCheckErr>  = Vec::new();
        let a_case = case::Case::check_case(&case_toml, &caseid, &mut errs);

        assert!(errs.len() == 0);
        assert!(a_case.dim.is_some());
        assert!(a_case.dim.unwrap() == case::Dimensions::FT);
     }

    #[test]
    // Test the setting the IN dimention works.
    fn has_dim_eq_in() {
        // Need to construct the TOML
        let mut case_toml = toml_edit::Table::new();
        // Put the dimension into the doc.
        case_toml[case::INPUT_NAME_CTRLCARD_DIM] = toml_edit::value(case::Dimensions::IN.name());
        
        let caseid = String::from("0");
        let mut errs: Vec<case::errors::CaseCheckErr>  = Vec::new();
        let a_case = case::Case::check_case(&case_toml, &caseid, &mut errs);

        assert!(errs.len() == 0);
        assert!(a_case.dim.is_some());
        assert!(a_case.dim.unwrap() == case::Dimensions::IN);
     }

     #[test]
     // TEst the naca card
     fn simple_nacacard() {
        let mut case_toml = toml_edit::Table::new();

        let mut naca1 = toml_edit::Table::new();        
        naca1[case::INPUT_NAME_NACA_PLANFORM] = toml_edit::value(case::Planform::Wing.name());
        naca1[case::INPUT_NAME_NACA_SERIES] = toml_edit::value(case::NacaSeries::FourDigit.name());
        naca1[case::INPUT_NAME_NACA_DESIGN] = toml_edit::value("4015");

        let mut naca2 = toml_edit::Table::new();        
        naca2[case::INPUT_NAME_NACA_PLANFORM] = toml_edit::value(case::Planform::HorzTail.name());
        naca2[case::INPUT_NAME_NACA_SERIES] = toml_edit::value(case::NacaSeries::FiveDigit.name());
        naca2[case::INPUT_NAME_NACA_DESIGN] = toml_edit::value("23000");

        let mut naca3 = toml_edit::Table::new();        
        naca3[case::INPUT_NAME_NACA_PLANFORM] = toml_edit::value(case::Planform::VertTail.name());
        naca3[case::INPUT_NAME_NACA_SERIES] = toml_edit::value(case::NacaSeries::SixSeries.name());
        naca3[case::INPUT_NAME_NACA_DESIGN] = toml_edit::value("63000");

        let mut naca4 = toml_edit::Table::new();        
        naca4[case::INPUT_NAME_NACA_PLANFORM] = toml_edit::value(case::Planform::VentFin.name());
        naca4[case::INPUT_NAME_NACA_SERIES] = toml_edit::value(case::NacaSeries::OneSeries.name());
        naca4[case::INPUT_NAME_NACA_DESIGN] = toml_edit::value("16000");

        let mut naca_array = toml_edit::ArrayOfTables::new();
        naca_array.append(naca1);
        naca_array.append(naca2);
        naca_array.append(naca3);
        naca_array.append(naca4);

        case_toml[case::INPUT_NAME_CTRLCARD_NACA] = toml_edit::Item::ArrayOfTables(naca_array);

        //---

        let caseid = String::from("0");
        let mut errs: Vec<case::errors::CaseCheckErr>  = Vec::new();
        let a_case = case::Case::check_case(&case_toml, &caseid, &mut errs);

        assert!(errs.len() == 0, format!("{:?}", errs));
        assert!(a_case.naca.is_some());
        let naca_vec = a_case.naca.unwrap();
        assert!(naca_vec.len() == 4);
        let naca1_struct = &naca_vec[0];
        let naca2_struct = &naca_vec[1];
        let naca3_struct = &naca_vec[2];
        let naca4_struct = &naca_vec[3];
        assert!(naca1_struct.planform == case::Planform::Wing);
        assert!(naca1_struct.series == case::NacaSeries::FourDigit);
        assert!(naca1_struct.design == "4015");
        assert!(naca2_struct.planform == case::Planform::HorzTail);
        assert!(naca2_struct.series == case::NacaSeries::FiveDigit);
        assert!(naca2_struct.design == "23000");
        assert!(naca3_struct.planform == case::Planform::VertTail);
        assert!(naca3_struct.series == case::NacaSeries::SixSeries);
        assert!(naca3_struct.design == "63000");
        assert!(naca4_struct.planform == case::Planform::VentFin);
        assert!(naca4_struct.series == case::NacaSeries::OneSeries);
        assert!(naca4_struct.design == "16000");
    }
}
