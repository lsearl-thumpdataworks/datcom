use conf_file;

const INPUT_NAME_NAMELIST_BODY: &'static str = "BODY";

// BODY namelist variables
const INPUT_NAME_VAR_X: &'static str = "X";
const INPUT_NAME_VAR_S: &'static str = "S";
const INPUT_NAME_VAR_P: &'static str = "P";
const INPUT_NAME_VAR_R: &'static str = "R";
const INPUT_NAME_VAR_ZU: &'static str = "ZU";
const INPUT_NAME_VAR_ZL: &'static str = "ZL";
const INPUT_NAME_VAR_BNOSE: &'static str = "BNOSE";
const INPUT_NAME_VAR_BTAIL: &'static str = "BTAIL";
const INPUT_NAME_VAR_BLN: &'static str = "BLN";
const INPUT_NAME_VAR_BLA: &'static str = "BLA";
const INPUT_NAME_VAR_DS: &'static str = "DS";
const INPUT_NAME_VAR_ITYPE: &'static str = "ITYPE";
const INPUT_NAME_VAR_METHOD: &'static str = "METHOD";

const INPUT_SPEC_BODY_TABLE: [conf_file::SpecItem<'static>; 14] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_X, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_S, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_P, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_R, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ZU, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ZL, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BNOSE, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BTAIL, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BLN, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BLN, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BLA, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DS, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ITYPE, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_METHOD, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
];

pub const INPUT_SPEC_BODY_ITEM: conf_file::SpecItem<'static> =  conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_BODY, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_BODY_TABLE)
};


/** Variables for the Namelist "Body" in GroupII Namelists.
 * Variable /NX is not included. We can determine NX as we read the arrays.
 */
pub struct Body {
    /** Longitudinal distance measuredfrom arbitrary location. Length */
    x: Vec<f32>,
    /** Cross Section Area at Station X(i). Area.
     * If this is specified then 'p' and 'r' can be computed and may be omitted.
     * If omitted=pi*R^2
     */
    s: Option<Vec<f32>>,
    /** Periphery at Station x(i). Length.
     * If this is specified then 's' and 'r' can be computed and may be omitted.
     * If omitted=2*pi*R where R=sqrt(S/pi) or input R, whichever is the largest.
     */
    p :Option<Vec<f32>>,
    /** Planform half width at station x(i). Length. 
     * If this is specified then 's' and 'r' can be computed and may be omitted.
     * Use equivalent radius at transonic and supersonic mach Number Req=sqrt(S/pi).
     * If omitted=sqrt(S/pi)
     */
    r: Option<Vec<f32>>,
    /** Z coordinate at upper body surface at station x(i). Length.
     * Only required for subsonic asymmetric bodies.
    */
    zu: Option<Vec<f32>>,
    /** Z coordinate at lower body surface at station x(i). Length.
     * Positive when above centerline.
     * Only required for subsonic asymmetric bodies.
     */
    zl: Option<Vec<f32>>,
    /** BNOSE = 1 Conical Nose, BNOSE=2 OGIVE Nose.
     * Negative when below centerline.
     * Not required for subsonic speed regime.
     */
    bnose: Option<i32>,
    /** BTAIL=1 Conical Tail, BTAIL=2 OGIVE Tail.
     * Noterequired for subsonic speed regime.
     */
    btail: Option<i32>,
    /** Length of body nose. Length.
     * Omit for lbt = 0.
     * Not required for subsonic speed regime.
     */
    bln: Option<f32>,
    /** Length of cylindrical afterbody segment. Length.
     *  Not required for subsonic speed regime.
     */
    bla: Option<f32>,
    /** Nose Blundness diameter. Zero for sharp nosebodies. Length.
     * Hypersonic speed regime only.
     */
    ds: Option<f32>,
    /** 1= Streight wing, No area Rule.
     * 2= swept wing, no area rule.
     * 3= swept wing, ara rule.
     * Default=2.
     * Used in calculation of transonic drag divergence divergence
     * mach number, DATCOM figure 4.5.3.1-18.
     */
    itype: Option<i32>,
    /** 1= Use existing methods (default)
     * 2= use jorgensen method.
     */
    method: Option<i32>
}
