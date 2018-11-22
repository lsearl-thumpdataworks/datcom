/// -----
/// File: /src/case/errors.rs  
/// Project: datcon  
/// File Created: Tuesday, 13th Nov 182018 6:04:01 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Tuesday, 13th November 2018 6:04:01 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----

/// Contains the enums and structs for handling errors that occured while processing the 
/// input file.

use std::fmt::{Display, Formatter};
use std;
use toml_edit;

#[derive(Debug, PartialEq)]
pub enum ValueInputErr {
    /// Value was too low.
    /// * low_limit - the lowest allowed value.
    /// * value - the value received that was bad.
    RangeLowF32Err(f32, f32),
    /// Value was too high.
    /// * high_limit - the highest allowed value.
    /// * value - the value received that was bad.
    RangeHighF32Err(f32, f32),
        /// Value was too low.
    /// * low_limit - the lowest allowed value.
    /// * value - the value received that was bad.
    RangeLowI32Err(i32, i32),
    /// Value was too high.
    /// * high_limit - the highest allowed value.
    /// * value - the value received that was bad.
    RangeHighI32Err(i32, i32),
    /// The Data was not the expected type error message.
    /// * expected type
    /// * received value.
    BadDataTypeErr(String, String),
    /// The data was not one of the expected enumereted values
    /// * expected values
    /// * received value.
    BadEnumErr(String, String),
    /// We had duplcate data. Teh value of the duplcated data.
    /// * the value 
    DuplicateDataErr(String)
}

impl Display for ValueInputErr {
    fn fmt (&self, f: & mut Formatter) -> std::fmt::Result {
        match self {
            &ValueInputErr::RangeLowF32Err(ref low_limit, ref value) =>
                write!(f, "Value={} was below the low limit={}", value, low_limit),
            &ValueInputErr::RangeHighF32Err(ref high_limit, ref value) =>
                write!(f, "Value={} was above the high limit={}", value, high_limit),
            &ValueInputErr::RangeLowI32Err(ref low_limit, ref value) =>
                write!(f, "Value={} was below the low limit={}", value, low_limit),
            &ValueInputErr::RangeHighI32Err(ref high_limit, ref value) =>
                write!(f, "Value={} was above the high limit={}", value, high_limit),
            &ValueInputErr::BadDataTypeErr(ref expected_type, ref value) =>
                write!(f, "Value={} was not of type={}", value, expected_type),
            &ValueInputErr::BadEnumErr(ref expected_values, ref value) =>
                write!(f, "Value={} was not one of={}", value, expected_values),
            &ValueInputErr::DuplicateDataErr(ref value) =>
                write!(f, "Duplicate Value={}", value)
         }
    }
}

// #####################################################################################
#[derive(Debug, PartialEq)]
pub enum ArrayInputErr {
    /// An array element had a bad value. 
    /// * bad value index
    /// * bad value error
    BadEleValueErr(usize, ValueInputErr),
    /// The array was not the expected length.
    /// * expected length
    /// * ectual length
    BadExpectedLengthErr(usize, usize),
    /// An array was empty but not expected to be.
    BadEmptyErr
}

impl Display for ArrayInputErr {
    fn fmt (&self, f: & mut Formatter) -> std::fmt::Result {
        match self {
            &ArrayInputErr::BadEleValueErr(ref index, ref err) =>
                write!(f, "Array Index={}: {}", index, err),
            &ArrayInputErr::BadExpectedLengthErr(ref expected_len, ref actual_len) =>
                write!(f, "Expected Length={}, actual Length={}", expected_len, actual_len),
            &ArrayInputErr::BadEmptyErr => 
                write!(f, "Array was empty, expected at least 1 value.")
         }
    }
}

// #####################################################################################

#[derive(Debug, PartialEq)]
pub enum CaseCheckErr {
    /// * Case name
    /// * listname
    /// * variable name
    /// * the error
    ValueErr(String, String, String, ValueInputErr),
    /// * Case name
    /// * listname
    /// * variable name
    /// * the error
    ArrayErr(String, String, String, ArrayInputErr)
}

impl Display for CaseCheckErr {
    fn fmt (&self, f: & mut Formatter) -> std::fmt::Result {
        match self {
            &CaseCheckErr::ValueErr(ref case, ref listname, ref variable, ref err) =>
                write!(f, "Case={}, Listname={}, Variable={}, {}", 
                    case, listname, variable, err),
            &CaseCheckErr::ArrayErr(ref case, ref listname, ref variable, ref err) =>
                write!(f, "Case={}, Listname={}, Variable={}, {}", 
                    case, listname, variable, err),
        }
    }
}


/// Convert the TOML array to an vector of f32 numbers.
/// * ary - the TOML array
/// * length_match - if not None then make sure that the input array length matches the value.
/// * lower_limit - if not None then compare the value of each element to make sure that the value
/// is >= the limit
/// * upper_limit - if not None then compare the value of each element to make sure that the value
/// is <= the limit.
/// * return - Ok -> the resulting array, Err -> a vector of errors that were found.
fn toml_to_float_array(ary: &toml_edit::Array, length_match: &Option<usize>, lower_limit: &Option<f32>,
                        upper_limit: &Option<f32>) ->
    Result<Vec<f32>, Vec<ArrayInputErr>> {

    // Do a sanity check on the array values.
    let mut errs: Vec<ArrayInputErr> = Vec::new();

    if length_match.is_some() && length_match.unwrap() != ary.len() {
        errs.push(ArrayInputErr::BadExpectedLengthErr(length_match.unwrap(), ary.len()))
    }

    let mut i: usize = 0;
    let mut new_ary: Vec<f32> = Vec::new();

    for val in ary.iter() {
        if val.is_integer() || val.is_float() {
            match val.as_float() {
                None => panic!("Could not convert float but I don't know why."),
                Some(f64val) => {
                    let mut new_val = f64val as f32;
                    if lower_limit.is_some() && new_val < lower_limit.unwrap()  {
                        errs.push(ArrayInputErr::BadEleValueErr(i, ValueInputErr::RangeLowF32Err(lower_limit.unwrap() , new_val)));
                        new_val = lower_limit.unwrap();
                    } else if upper_limit.is_some() && new_val > upper_limit.unwrap() {
                        errs.push(ArrayInputErr::BadEleValueErr(i, ValueInputErr::RangeHighF32Err(upper_limit.unwrap() , new_val)));
                        new_val = upper_limit.unwrap();
                    }
                    new_ary.push(new_val);
                }
            }
        }
        else {
            errs.push(ArrayInputErr::BadEleValueErr(i, ValueInputErr::BadDataTypeErr(String::from("f32"), 
                                                    String::from(val.as_str().unwrap()))));
            new_ary.push(0.0); // Don't know what to pick for this.
        }

        i = i + 1;
    }
    if errs.len() == 0 {
        Ok(new_ary)
    }
    else {
        Err(errs)
    }
}

// #####################################################################################
///
/// Make sure that the i32 from the toml_edit passes the range test.
/// 
pub fn check_i32 (table: &toml_edit::Table, case: &String, namelist: &str, variable: &str, 
                    lower_limit: &Option<i32>,
                    upper_limit: &Option<i32>, errs: &mut Vec<CaseCheckErr>) -> Option<i32>  {
    if table[variable].is_integer()  {
        let mut new_val = table[variable].as_integer().unwrap() as i32;
        if lower_limit.is_some() && new_val < lower_limit.unwrap() {
            errs.push(CaseCheckErr::ValueErr(String::from(case.clone()),
                                            String::from(namelist),
                                            String::from(variable),
                                            ValueInputErr::RangeLowI32Err(lower_limit.unwrap() ,
                                                                             new_val)));
            new_val = lower_limit.unwrap();
        }
        if upper_limit.is_some() && new_val > upper_limit.unwrap() {
            errs.push(CaseCheckErr::ValueErr(String::from(case.clone()),
                                            String::from(namelist),
                                            String::from(variable),
                                            ValueInputErr::RangeHighI32Err(upper_limit.unwrap() ,
                                                                             new_val)));
            new_val = lower_limit.unwrap();
        }
        Some(new_val)
    }
    else {
        None
    }
}
// #####################################################################################
/// Parse a f32 from a table namelist.
/// If a value is not a number then an error is added to errs and 'None' is returned.
/// * table - doc table that contains namelist variables.
/// * case - the name of the case (for error reporting only)
/// * namelist - the namelist of the table for error reporting.
/// * varaible - the variable to find in the namelist
/// * lower_limit - value must be >= lower_limit, otherwise error. None. returned.
/// * upper_limit - value must be <= upper_limit, otherwise error. None. returned.
/// * errs: If any errors are added then None is the return value.
/// * return
/// ** Some - the good value
/// ** None - There was no value found or there was an error with the value.
pub fn check_f32 (table: &toml_edit::Table, case: &String, namelist: &str, variable: &str, 
                    lower_limit: &Option<f32>,
                    upper_limit: &Option<f32>, errs: &mut Vec<CaseCheckErr>) -> Option<f32>  {
    if table[variable].is_integer() || table[variable].is_float() {
        let mut new_val = table[variable].as_float().unwrap() as f32;
        if lower_limit.is_some() && new_val < lower_limit.unwrap() {
            errs.push(CaseCheckErr::ValueErr(String::from(case.clone()),
                                            String::from(namelist),
                                            String::from(variable),
                                            ValueInputErr::RangeLowF32Err(lower_limit.unwrap() ,
                                                                             new_val)));
            new_val = lower_limit.unwrap();
        }
        if upper_limit.is_some() && new_val > upper_limit.unwrap() {
            errs.push(CaseCheckErr::ValueErr(String::from(case.clone()),
                                            String::from(namelist),
                                            String::from(variable),
                                            ValueInputErr::RangeHighF32Err(upper_limit.unwrap() ,
                                                                             new_val)));
            new_val = lower_limit.unwrap();
        }
        Some(new_val)
    }
    else {
        None
    }
}
// #####################################################################################

// #####################################################################################
/// Parse a f32 array from a table namelist.
/// If an array value is not a number then an error is added to errs and None is returned.
/// * table - doc table that contains namelist variables
/// * case - the name of the case (for error reporting only)
/// * namelist - the name of the namelist. for error reporting.
/// * varaible - the variable to find in the namelist
/// * allow_empty - if true then the array may be empty (0 elements), otherwise and error.
/// * lower_limit - all array values must be >= lower_limit, otherwise error. None. returned.
/// * upper_limit - all array values must be <= upper_limit, otherwise error. None. returned.
/// * errs: If any errors area added then None is the return value.
/// * return
/// ** Some - the good array
/// ** None - There was not array found or there was an error with the array.
pub fn check_f32_array (table: &toml_edit::Table, case: &String, namelist: &str, variable: &str, 
                    allow_empty: bool,
                    lower_limit: &Option<f32>,  upper_limit: &Option<f32>, errs: &mut Vec<CaseCheckErr>) -> Option<Vec<f32>>  {
    if table[variable].is_array() {
        let a_array = table[variable].as_array().unwrap();
        if a_array.len() == 0 {
            if !allow_empty {
                errs.push(CaseCheckErr::ArrayErr(case.clone(), String::from(namelist), String::from(variable),
                    ArrayInputErr::BadEmptyErr))
            };
            let a_vec: Vec<f32> = Vec::new();
            Some(a_vec)
        } else {
            match toml_to_float_array(a_array, &None, lower_limit, upper_limit) {
                Ok(ary) => 
                    Some(ary),
                Err(e) => {
                    for ary_err in e {
                        errs.push(CaseCheckErr::ArrayErr(String::from(case.clone()),
                                                        String::from(namelist),
                                                        String::from(variable), ary_err));
                    }
                    None
                }
            }
        }
    }
    else {
        None
    }

}

