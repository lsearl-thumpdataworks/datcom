
use conf_file;

const INPUT_NAME_NAMELIST_ASYFLP: &'static str = "ASYFLP";

// ASYFLP namelist variable names.
const INPUT_NAME_VAR_STYPE: &'static str = "STYPE";
const INPUT_NAME_VAR_SPANFI: &'static str = "SPANFI";
const INPUT_NAME_VAR_SPANFO: &'static str = "SPANFO";
const INPUT_NAME_VAR_PHETE: &'static str = "PHETE";
const INPUT_NAME_VAR_DELTAL: &'static str = "DELTAL";
const INPUT_NAME_VAR_DELTAR: &'static str = "DELTAR";
const INPUT_NAME_VAR_CHRDFI: &'static str = "CHRDFI";
const INPUT_NAME_VAR_CHRDFO: &'static str = "CHRDFO";
const INPUT_NAME_VAR_DELTAD: &'static str = "DELTAD";
const INPUT_NAME_VAR_DELTAS: &'static str = "DELTAS";
const INPUT_NAME_VAR_XSOC: &'static str = "XSOC";
const INPUT_NAME_VAR_XSPRME: &'static str = "XSPRME";
const INPUT_NAME_VAR_HSOC: &'static str = "HSOC";


const INPUT_SPEC_ASYFLP_TABLE: [conf_file::SpecItem<'static>; 13] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_STYPE, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SPANFI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SPANFO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_PHETE, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DELTAL, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DELTAR, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDFI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDFO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DELTAD, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DELTAS, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XSOC, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XSPRME, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_HSOC, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },

];

pub const INPUT_SPEC_ASYFLP_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_ASYFLP, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_ASYFLP_TABLE)
};


/** Namelist for asymetrical flaps. */
pub struct Asyflp {
     stype: i32,
     delta: Vec<f32>,
     phete: Option<f32>,
     chrdfi: Option<f32>,
     chrdfo: Option<f32>,
     spanfi: f32,
     spanfo: Option<f32>,
    deltal: Option<Vec<f32>>,
    deltar: Option<Vec<f32>>,
    deltad: Option<Vec<f32>>,
    deltas: Option<Vec<f32>>,
    xsoc: Option<Vec<f32>>,
    xsprme: Option<f32>,
    hsoc: Option<f32>
}
