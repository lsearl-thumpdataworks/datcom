/*
 * Filename: datcom/src/lib.rs
 * Path: datcom/src
 * Created Date: Friday, November 9th 2018, 6:37:47 pm
 * Author: lsearl
 * 
 * Copyright (c) 2018 Thump Data Works, Inc.
 ************************************************************************************ */

extern crate toml_edit;

pub mod utility;

pub mod conf_file;
pub mod case;
pub mod xycoord;
pub mod decode;
pub mod airfol;
pub mod coord4;





//############################################################################################
#[cfg(test)]

mod tests {
   

    #[test] 
    fn empty_test (){

    } 
}
