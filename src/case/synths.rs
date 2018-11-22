use conf_file;

const INPUT_NAME_NAMELIST_SYNTHS: &'static str = "SYNTHS";

// SYNTHS name list variable names.
/// Longitudinal Loc of CG
const INPUT_NAME_VAR_XCG: &'static str = "XCG";
const INPUT_NAME_VAR_ZCG: &'static str = "ZCG";
const INPUT_NAME_VAR_XW: &'static str = "XW";
const INPUT_NAME_VAR_ZW: &'static str = "ZW";
const INPUT_NAME_VAR_ALIW: &'static str = "ALIW";
const INPUT_NAME_VAR_XH: &'static str = "XH";
const INPUT_NAME_VAR_ZH: &'static str = "ZH";
const INPUT_NAME_VAR_ALIH: &'static str = "ALIH";
const INPUT_NAME_VAR_XV: &'static str = "XV";
const INPUT_NAME_VAR_XVF: &'static str = "XVF";
const INPUT_NAME_VAR_ZV: &'static str = "ZV";
const INPUT_NAME_VAR_ZVF: &'static str = "ZVF";
const INPUT_NAME_VAR_SCALE: &'static str = "SCALE";
const INPUT_NAME_VAR_VERTUP: &'static str = "VERTUP";
const INPUT_NAME_VAR_HINAX: &'static str = "HINAX";

/// The table of variables for the synths namelist.
const INPUT_SPEC_SYNTHS_TABLE: [conf_file::SpecItem<'static>; 15] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XCG, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ZCG, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XW, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ZW, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ALIW, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XH, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ZH, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ALIH, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XV, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XVF, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ZV, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ZVF, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SCALE, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_VERTUP, required: false, item_type: conf_file::SpecItemType::Boolean, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_HINAX, required: false, item_type: conf_file::SpecItemType::Float, table: None
    }
];

/// This spec item is to be used in a higher level table.
pub const INPUT_SPEC_SYNTHS_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_SYNTHS, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_SYNTHS_TABLE)
};

// ##############################################################

/** Variables for he Namelist SYNTHS from the GroupII Namelists.. */
pub struct NameListSynths {
    /** Longitudinal location of CG, (moment Ref. Center). Length */
    xcg: f32,
    /** Vertical location of CG relative to reference plane. Length. */
    zcg: f32,
    /** Longintudinal location of Theoretical Wing Apex. Length. */
    xw: f32,
    /** Vertical location of Theoretical sing Apex relative to reference
     * Plane. Length.
     */
    zw: f32,
    /** Wing Root chord incidence angle measured from reference plane. Degrees */
    aliw: f32,
    /** Longitudinal location of the theoretical horzizonal tail apex. Length.
     * If HINAX is input, Xh and Zh are evaluated at zero incidence (Iw=0).
     */
    xh: Option<f32>,
    /** Vertical location of theroetical horizontal tail apex
     * relative to reference plane. Length.
     * If HINAX is input, Xh and Zh are evaluated at zero incidence (Iw=0).
     */
    zh: Option<f32>,
    /** Horizontal tail root chord incidence angle measured from refrence plane. Degree. */
    alih: f32,
    /** Longitudinal location of theoretical vertical tail apex. Length. */
    xv: f32,
    /** Longitudinal location of theoretcial ventral fin apex. Length. */
    xvf: f32,
    /** Vertical location of theoretical vertical tail apex. Length. */
    zv: f32,
    /** Vertical location of the theorectical ventral tail apex. Length. */
    zvf: f32,
    /** Vehicale scale factor (Multiplier to input dimensions). */
    scale: f32,
    /** Vertup=true, vertical panel above ref plane (default).
     * Vertup=false, vertical panel below ref plane.
     */
    vertup: Option<bool>,
    /** Longitudinal location of horizontal tail hinge axis. Length.
     * Required only for all-movable horizontal tail trim option.
     */
    hinax: Option<f32>
}
