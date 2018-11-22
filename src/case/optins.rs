/*
 * Filename: .../datcom/src/case/optins.rs
 * Path: .../datcom/src/case
 * Created Date: Tuesday, November 13th 2018, 11:08:24 am
 * Author: Leon S. Searl
 * 
 * Copyright (c) 2018 Tump Data Works, Inc.
 */

use conf_file;

// ###################################################

pub const INPUT_NAME_NAMELIST_OPTINS: &'static str = "OPTINS";


// OPTINS name list variable names.
/// Roughness factor
pub const INPUT_NAME_VAR_ROUGFC: &'static str = "FOUGFC";
/// Reference Area, Wing area to use of not input.
pub const INPUT_NAME_VAR_SREF: &'static str = "SREF";
/// Longitudinal Ref Length value. Wing MAC to use if not input.
pub const INPUT_NAME_VAR_CBARR: &'static str = "CBARR";
/// Lateral ref length, Wing space to use if not input
pub const INPUT_NAME_VAR_BLREF: &'static str = "BLREF";


// OPTINS table
const INPUT_SPEC_OPTINS_TABLE: [conf_file::SpecItem<'static>; 4] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ROUGFC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SREF, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CBARR, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_BLREF, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },

];

/// ITEM SPEC to be included in the case table.
pub const INPUT_SPEC_OPTINS_ITEM: conf_file::SpecItem<'static> = 
    conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_OPTINS, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_OPTINS_TABLE)
};

// ##################################################################################


/** Variables for the Namelist OPTINS from the GroupI NAmelists. */
pub struct Optins {
    /** Surface roughness factor, equivalent sand roughness.
     * Default to 0.16x10-3 inches. Length.
     *  */
    pub rougfc: Option<f32>,
    /** Reference ARea, Value of theoretical wing area used by program
     * if not input. Area.
     */
    pub sref: Option<f32>,
    /** Longitudinal reference length value of theoretical
     * wing mean aerodynamic chord used by program if not input. Length.
     */
    pub cbarr: Option<f32>,
    /** Lateral reference length value of wing span 
     * used by program if not input. Length.
     */
    pub blref: Option<f32>
}
