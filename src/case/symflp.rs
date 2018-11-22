use conf_file;

const INPUT_NAME_NAMELIST_SYMFLP: &'static str = "SYMFLP";

// SYMFLP namelist variable names.
const INPUT_NAME_VAR_FTYPE: &'static str = "FTYPE";
const INPUT_NAME_VAR_DELTA: &'static str = "DELTA";
const INPUT_NAME_VAR_PHETE: &'static str = "PHETE";
const INPUT_NAME_VAR_PHETEP: &'static str = "PHETEP";
const INPUT_NAME_VAR_CHRDFI: &'static str = "CHRDFI";
const INPUT_NAME_VAR_CHRDFO: &'static str = "CHRDFO";
const INPUT_NAME_VAR_SPANFI: &'static str = "SPANFI";
const INPUT_NAME_VAR_SPANFO: &'static str = "SPANFO";
const INPUT_NAME_VAR_CPRMEI: &'static str = "CPRMEI";
const INPUT_NAME_VAR_CPRMEO: &'static str = "CPRMEO";
const INPUT_NAME_VAR_CAPINB: &'static str = "CAPINB";
const INPUT_NAME_VAR_CAPOUT: &'static str = "CAPOUT";
const INPUT_NAME_VAR_DOBDEF: &'static str = "DOBDEF";
const INPUT_NAME_VAR_DOBCIN: &'static str = "DOBCIN";
const INPUT_NAME_VAR_DOBCOT: &'static str = "DOBCOT";
const INPUT_NAME_VAR_SCLD: &'static str = "SCLD";
const INPUT_NAME_VAR_SCMD: &'static str = "SCMD";
const INPUT_NAME_VAR_CB: &'static str = "CB";
const INPUT_NAME_VAR_TC: &'static str = "TC";
const INPUT_NAME_VAR_NTYPE: &'static str = "NTYPE";
const INPUT_NAME_VAR_JETFLP: &'static str = "JETFLP";
const INPUT_NAME_VAR_CMU: &'static str = "CMU";
const INPUT_NAME_VAR_DELJET: &'static str = "DELJET";
const INPUT_NAME_VAR_EFFJET: &'static str = "EFFJET";


const INPUT_SPEC_SYMFLP_TABLE: [conf_file::SpecItem<'static>; 24] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_FTYPE, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DELTA, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_PHETE, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_PHETEP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDFI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDFO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SPANFI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SPANFO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CPRMEI, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CPRMEO, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CAPINB, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CAPOUT, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DOBDEF, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DOBCIN, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DOBCOT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SCLD, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SCMD, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CB, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_NTYPE, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_JETFLP, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CMU, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DELJET, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_EFFJET, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },

];

pub const INPUT_SPEC_SYMFLP_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_SYMFLP, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_SYMFLP_TABLE)
};


/** Namelist for symetrical flaps. */
pub struct Symflp {
     ftype: i32,
     delta: Vec<f32>,
     phete: Option<f32>,
     phetep: Option<f32>,
     chrdfi: f32,
     chrdfo: f32,
     spanfi: f32,
     spanfo: f32,
     cprmei: Option<Vec<f32>>,
     cprmeo: Option<Vec<f32>>,
    capinb: Option<Vec<f32>>,
    capout: Option<Vec<f32>>,
    dobdef: Option<Vec<f32>>,
    dobcin: Option<f32>,
    dobcot: Option<f32>,
    scld: Option<Vec<f32>>,
    scmd: Option<Vec<f32>>,
    cb: Option<f32>,
    tc: Option<f32>,
    ntype: Option<i32>,
    jetflp: Option<i32>,
    cmu: Option<f32>,
    dejet: Option<Vec<f32>>,
    effjet: Option<Vec<f32>>

}
