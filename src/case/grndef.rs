use conf_file;

const INPUT_NAME_NAMELIST_GRNDEF: &'static str = "GRNDEF";

// Namelist GRNDEF variable names.
const INPUT_NAME_VAR_GRDHT: &'static str = "GRDHT";


const INPUT_SPEC_GRNDEF_TABLE: [conf_file::SpecItem<'static>; 1] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_GRDHT, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },

];

pub const INPUT_SPEC_GRNDEF_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_GRNDEF, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_GRNDEF_TABLE)
};


/** The variables for the naem list GRNDEF. */
pub struct Grndef {
    /** Values of Ground hieghts. Ground Heights equal
     * altitude of ref plane relative to ground.
     */
    grdht: Vec<f32>
}
