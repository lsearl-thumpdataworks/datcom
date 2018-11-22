/// -----
/// File: /src/decode.rs  
/// Project: datcon  
/// File Created: Thursday, 15th Nov 182018 8:21:20 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Thursday, 15th November 2018 8:21:20 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----

use case;

/// Decode a string that constains a NACA airfoil identifier.
/// Basically pull the special characters out of the Design value
/// naca_info - the structure from reading the input file.
/// * return
/// ** the 'design' without special characters.
/// ** Since the series may be a modifed version, determined by looking a the 'design' 
///    this value is returned also which may be different than what was passed into the routine.
pub fn decode(naca_info: &case::NacaCard ) -> Result<( case::NacaSeries, Vec<char> ), String> {

    let mut design: Vec<char> = naca_info.design.chars().collect();
    match naca_info.series {
        
        case::NacaSeries::FourDigit => {
            let mut series = case::NacaSeries::FourDigit;
            // Remove spaces and periods.
            design.retain(|&v| v != '.'); // get rid of periods.
            design.retain(|&v| v != ' '); // get rid of spaces.
            if design.contains(&'-') {
                // This is a modified version of the design
                // remove the dash
                design.retain(|&v| v != '-'); // get rid of the dash
                series = case::NacaSeries::FourDigitModified;
            }
            
            if design.iter().find(|&&v| !v.is_digit(10)).is_some() {
                return Err(format!("There unexpected charaters in the NACA 4 Digit ID (only use digits and maybe '-'"))
            }
            match series {
                case::NacaSeries::FourDigit => {
                    if design.len() != 4 {
                        return Err(format!("There are extra characters in the 4 digit NACA series."))
                    }
                },
                case::NacaSeries::FourDigitModified => {
                    if design.len() != 6 {
                        return Err(format!("There are extra characters in the 4 digit modified NACA series."))
                    }
                },
                _ => panic!("Programming error.")
            }
            Ok((series, design))
            
        },

        // TODO: Implement
        case::NacaSeries::FourDigitModified => unimplemented!(),

        // TODO: Implement
        case::NacaSeries::FiveDigit => unimplemented!(),

        // TODO: Implement
        case::NacaSeries::FiveDigitModified => unimplemented!(),

        // TODO: Implement
        case::NacaSeries::OneSeries => unimplemented!(),

        // TODO: Implement
        case::NacaSeries::SixSeries => unimplemented!(),

        // TODO: Implement
        case::NacaSeries::Supersonic => unimplemented!()
    }
}
