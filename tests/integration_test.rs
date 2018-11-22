/// -----
/// File: /tests/integration_test.rs  
/// Project: datcon  
/// File Created: Friday, 16th Nov 182018 12:43:02 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Friday, 16th November 2018 12:43:04 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----

extern crate tempfile;
extern crate toml_edit;
extern crate datcom;


use datcom::case;
use std::io::Write;


#[test]
// 
fn read_without_error()  {
    let toml = b"
[[CASE]]
DIM = \"FT\"
[[CASE.NACA]]
PLANFORM = \"WING\"
SERIES = \"4DIGIT\"
DESIGN = \"4415\"
";
    // Write out the toml file.
    let dir = match tempfile::tempdir() {
        Ok(d) => d,
        Err(e) => panic!("{}", e)
    };
    let filepath = dir.path().join("case.toml");
    {
        let mut file = std::fs::File::create(&filepath).unwrap();
        file.write(toml).unwrap();
    }
        
    match case::read_config(&filepath) {
        Ok(_) => (),
        Err(errs) => {
            for e in errs {
                println!("ERROR: {}", e)
            }
            panic!("Should not have had errors.")
        }
    }

    dir.close().unwrap();
}
