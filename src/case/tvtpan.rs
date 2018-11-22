use conf_file;

const INPUT_NAME_NAMELIST_TVTPAN: &'static str = "TVTPAN";

// TVTPAN namelist variable names.
const INPUT_NAME_VAR_BVP: &'static str = "BVP";
const INPUT_NAME_VAR_BV: &'static str = "BV";
const INPUT_NAME_VAR_BDV: &'static str = "BDV";
const INPUT_NAME_VAR_BH: &'static str = "BH";
const INPUT_NAME_VAR_SV: &'static str = "SV";
const INPUT_NAME_VAR_VPHITE: &'static str = "VPHITE";
const INPUT_NAME_VAR_VLP: &'static str = "VLP";
const INPUT_NAME_VAR_ZP: &'static str = "ZP";


const INPUT_SPEC_TVTPAN_TABLE: [conf_file::SpecItem<'static>; 8] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BVP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BV, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BDV, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BH, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SV, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_VPHITE, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_VLP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ZP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    }

];

 pub const INPUT_SPEC_TVTPAN_ITEM: conf_file::SpecItem<'static> =   conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_TVTPAN, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_TVTPAN_TABLE)
    };


/** Variables for the NameList twin vertical surfaces. */
pub struct Tvtpan {
    /** Vertical panel span above lifting surface. Length. */
    bvp: f32,
    /** Vertical panel span. Length. */
    bv: f32,
    /** Fuselage depth at quarter chord-point of vertical
     * panel mean aerodynamic chord. Length.
     */
    bdv: f32,
    /** Distance between vertical panels. Length */
    bh: f32,
    /** Planform area of one vertical panel. Area */
    sv: f32,
    /** Total traling edge angle of vertical panel airfoil
     * section. Degrees.
     */
    vphite: f32,
    /** Distance parallel to long axis between the CG and the
     * quarter chord point of the MAC of the panel. Positive
     * if aft of the CG. Length.
     */
    vlp: f32,
    /** Distance in the Z direction between the CG and the MAC
     * of the panel. Positive for panel above CG. Length.
     */
    zp: f32
}
