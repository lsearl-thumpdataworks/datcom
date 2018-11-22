use conf_file;

const INPUT_NAME_NAMELIST_PROPWR: &'static str = "PROPWR";

// Namelist PROPWR variable anems.
const INPUT_NAME_VAR_AIETLP: &'static str = "AIETLP";
const INPUT_NAME_VAR_NENGSP: &'static str = "NENGSP";
const INPUT_NAME_VAR_THSTCP: &'static str = "THSTCP";
const INPUT_NAME_VAR_PHALOC: &'static str = "PHALOC";
const INPUT_NAME_VAR_PHVLOC: &'static str = "PHVLOC";
const INPUT_NAME_VAR_PRPRAD: &'static str = "PRPRAD";
const INPUT_NAME_VAR_ENGFCT: &'static str = "ENGFCT";
const INPUT_NAME_VAR_BWAPR3: &'static str = "BWAPR3";
const INPUT_NAME_VAR_BWAPR6: &'static str = "BWAPR6";
const INPUT_NAME_VAR_BWAPR9: &'static str = "BWAPR9";
const INPUT_NAME_VAR_NOPBPE: &'static str = "NOPBPE";
const INPUT_NAME_VAR_BAPR75: &'static str = "BAPR75";
const INPUT_NAME_VAR_YP: &'static str = "YP";
const INPUT_NAME_VAR_CROT: &'static str = "CROT";

// Group III  NamE Lists
const INPUT_SPEC_PROPWR_TABLE: [conf_file::SpecItem<'static>; 14] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_AIETLP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_NENGSP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_THSTCP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_PHALOC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_PHVLOC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_PRPRAD, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ENGFCT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BWAPR3, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BWAPR6, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BWAPR9, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_NOPBPE, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BAPR75, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CROT, required: false, item_type: conf_file::SpecItemType::Boolean, table: None
    },

];


pub const INPUT_SPEC_PROPWR_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_PROPWR, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_PROPWR_TABLE)
};


/** Namelist for the properller variables. */
pub struct Propwr {
    /** angle of incidence of engine thurst axis, Degrees. */
    aietlp: f32,
    /** Number of engines. 1 or 2. */
    nengsp: i32,
    /** Thurst coefficient. */
    thstcp: f32,
    /** Axial location of propeller HUB. Length */
    phaloc: f32,
    /** Vertical location of propeller hub. Length */
    phvloc: f32,
    /** Propeller Radius */
    prprad: f32,
    /** Emperical normal force factor. */
    engfct: Option<f32>,
    /** Blade wdith at 0.3 propeller radius. */
    bwapr3: Option<f32>,
    /** Blade wdith at 0.6 propeller radius. */
    bwapr6: Option<f32>,
    /** Blade wdith at 0.9 propeller radius. */
    bwapr9: Option<f32>,
    /** Number of propeller blades per engine. */
    nopbpe: i32,
    /** Blade angle at 0.75 propeller radius */
    bapr75: f32,
    /** Lateral location of engine. Length. */
    yp: f32,
    /** true=counter rotating propeller,
     * false non counter rotating propeller.
     */
    crot: i32
}
