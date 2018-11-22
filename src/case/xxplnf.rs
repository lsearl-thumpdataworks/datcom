use conf_file;

const INPUT_NAME_NAMELIST_WGPLNF: &'static str = "WGPLNF";
const INPUT_NAME_NAMELIST_HTPLNF: &'static str = "HTPLNF";
const INPUT_NAME_NAMELIST_VTPLNF: &'static str = "VTPLNF";
const INPUT_NAME_NAMELIST_VFPLNF: &'static str = "VFPLNF";

// WGPLNF, HTPLNF, VTPLNF and VPLNF variable names.
const INPUT_NAME_VAR_CHRDPT: &'static str = "CHRDPT";
const INPUT_NAME_VAR_SSPNOP: &'static str = "SSPNOP";
const INPUT_NAME_VAR_SSPNE: &'static str = "SSPNE";
const INPUT_NAME_VAR_SSPN: &'static str = "SSPN";
const INPUT_NAME_VAR_CHRDBP: &'static str = "CHRDBP";
const INPUT_NAME_VAR_CHRDR: &'static str = "CHRDR";
const INPUT_NAME_VAR_SAVSI: &'static str = "SAVSI";
const INPUT_NAME_VAR_SAVSO: &'static str = "SAVSO";
const INPUT_NAME_VAR_CHSTAT: &'static str = "CHSTAT";
const INPUT_NAME_VAR_TWISTA: &'static str = "TWISTA";
const INPUT_NAME_VAR_SSPNDD: &'static str = "SSPNDD";
const INPUT_NAME_VAR_DHDADI: &'static str = "DHDADI";
const INPUT_NAME_VAR_DHDADO: &'static str = "DHDADO";
const INPUT_NAME_VAR_TYPE: &'static str = "TYPE";
const INPUT_NAME_VAR_SHB: &'static str = "SHB";
const INPUT_NAME_VAR_SEXT: &'static str = "SEXT";
const INPUT_NAME_VAR_RLPH: &'static str = "RLPH";
const INPUT_NAME_VAR_SVWB: &'static str = "SVWB";
const INPUT_NAME_VAR_SVB: &'static str = "SVB";
const INPUT_NAME_VAR_SVHB: &'static str = "SVHB";

///
/// Wing planform table.
const INPUT_SPEC_WGPLNF_TABLE: [conf_file::SpecItem<'static>; 15] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDPT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDPT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPNOP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPNE, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPN, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDBP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDR, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SAVSI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SAVSO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHSTAT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TWISTA, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPNDD, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DHDADI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DHDADO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TYPE, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    }


];

///
/// Horzontal tail planform table.
const INPUT_SPEC_HTPLNF_TABLE: [conf_file::SpecItem<'static>; 18] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDPT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDPT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPNOP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPNE, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPN, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDBP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDR, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SAVSI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SAVSO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHSTAT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TWISTA, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPNDD, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DHDADI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DHDADO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TYPE, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SHB, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SEXT, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_RLPH, required: false, item_type: conf_file::SpecItemType::Array, table: None
    }
];

///
/// Vertical (tail and fin) planform table.
const INPUT_SPEC_VPLNF_TABLE: [conf_file::SpecItem<'static>; 14] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDPT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDPT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPNOP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPNE, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SSPN, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDBP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHRDR, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SAVSI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SAVSO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CHSTAT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TYPE, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SVWB, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SVB, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SVHB, required: false, item_type: conf_file::SpecItemType::Array, table: None
    }

];



pub const INPUT_SPEC_WGPLNF_ITEM: conf_file::SpecItem<'static> =  conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_WGPLNF, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_WGPLNF_TABLE)
};

pub const INPUT_SPEC_HTPLNF_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_HTPLNF, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_HTPLNF_TABLE)
};

pub const INPUT_SPEC_VTPLNF_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_VTPLNF, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_VPLNF_TABLE)
};

pub const INPUT_SPEC_VFPLNF_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_VFPLNF, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_VPLNF_TABLE)
};


/** For the WGPLNF, HTPLNF, VTPLNF, VFPLNF. Part of the GroupII namelists.  */

/** For the WGPLNF, HTPLNF, VTPLNF, VFPLNF. Part of the GroupII namelists.  */
pub struct Wgplnf {
    /** Tip chord, length */
    chrotp: f32,
    /** Semi-span outboard panel, length.
     * Not Required for straight tapered planform.
     */
    sspnop: Option<f32>,
    /** Semi-span exposed panel, length */
    sspne: f32,
    /** Semi-span theoretical panel from theoretical root chord, length */
    sspn: f32,
    /** Chord at brake point, length.
     * Not Required for straight tapered planform
     */
    chrobp: Option<f32>,
    /** Root Chord, Length */
    chrdr: f32,
    /** Inboard Panel sweep angle, degrees */
    savsi: f32,
    /** Outboard panel sweep angle, degrees.
     * Not Required for straight tapered planform
     */
    savso: Option<f32>,
    /** Reference chord station for inboard and outboard
     * panel sweep angles. fraction of chord 0.0 to 1.0
     */
    chstat: f32,
    /** Twist angle, negative leading edge rotated down
     * (from exposed root to tip), degrees
     * Not used for VTPLNF and VFPLHF (vertical tail) name lists.
     */
    twista: f32,
    /** Semi-span of outboard panel with dihedral, length.
     * Not required for straight tapered planform.
     * Not used for VTPLNF and VFPLHF (vertical tail) name lists.
     */
    sspndo: Option<f32>,
    /** Dihedral angle of inboard panel, degrees.
     * If the Input and outboard panels have the same dihedral then omit
     * the outboard panel dihedral.
     * Not used for VTPLNF and VFPLHF (vertical tail) name lists.
     */
    dhdadi: f32,
    /** Dihedral angle of outboard panel, degrees.
     * If the Input and outboard panels have the same dihedral then omit
     * the outboard panel dihedral.
     * Not used for VTPLNF and VFPLHF (vertical tail) name lists.
     */
    dhdado: Option<f32>,
    /** 1= straight tapered planform
     * 2= double delta planform (AR <= 3)
     * 3= cranked planform (AR > 3).
     */
    fptype: i32
 
}

/** For the WGPLNF, HTPLNF, VTPLNF, VFPLNF. Part of the GroupII namelists.  */
pub struct Htplnf {
    /** Tip chord, length */
    chrotp: f32,
    /** Semi-span outboard panel, length.
     * Not Required for straight tapered planform.
     */
    sspnop: Option<f32>,
    /** Semi-span exposed panel, length */
    sspne: f32,
    /** Semi-span theoretical panel from theoretical root chord, length */
    sspn: f32,
    /** Chord at brake point, length.
     * Not Required for straight tapered planform
     */
    chrobp: Option<f32>,
    /** Root Chord, Length */
    chrdr: f32,
    /** Inboard Panel sweep angle, degrees */
    savsi: f32,
    /** Outboard panel sweep angle, degrees.
     * Not Required for straight tapered planform
     */
    savso: Option<f32>,
    /** Reference chord station for inboard and outboard
     * panel sweep angles. fraction of chord 0.0 to 1.0
     */
    chstat: f32,
    /** Twist angle, negative leading edge rotated down
     * (from exposed root to tip), degrees
     * Not used for VTPLNF and VFPLHF (vertical tail) name lists.
     */
    twista: f32,
    /** Semi-span of outboard panel with dihedral, length.
     * Not required for straight tapered planform.
     * Not used for VTPLNF and VFPLHF (vertical tail) name lists.
     */
    sspndo: Option<f32>,
    /** Dihedral angle of inboard panel, degrees.
     * If the Input and outboard panels have the same dihedral then omit
     * the outboard panel dihedral.
     * Not used for VTPLNF and VFPLHF (vertical tail) name lists.
     */
    dhdadi: f32,
    /** Dihedral angle of outboard panel, degrees.
     * If the Input and outboard panels have the same dihedral then omit
     * the outboard panel dihedral.
     * Not used for VTPLNF and VFPLHF (vertical tail) name lists.
     */
    dhdado: Option<f32>,
    /** 1= straight tapered planform
     * 2= double delta planform (AR <= 3)
     * 3= cranked planform (AR > 3).
     */
    fptype: i32,
    /** Portion of fuselage side area that lies between mach
     * lines originating from leading and trailing edges of 
     * horizontal tail exposed root chord. Area.
     * Only required for supersonic and hypersonic speed regines. Number of 
     * array elements must match number of elements in MACH array.
     * Not used for WGPLNF, VTPLNF and VFPLHF (wing and vertical tail) name lists.
    */
    shb: Option<Vec<f32>>,
    /** Portion of extendd fuselage side area that lies between 
     * mach lines originating from leading and trailing edges of
     * horizontal tail exposed root chord. Area.
     * Only required for supersonic and hypersonic speed regines. Number of 
     * array elements must match number of elements in MACH array.
     * Not used for WGPLNF, VTPLNF and VFPLHF (wing and vertical tail) name lists.
     */
    sext: Option<Vec<f32>>,
    /** Longitudinal distance between CG and Centroid of SHB, positive
     * aft of CG. Length
     * Only required for supersonic and hypersonic speed regines. Number of 
     * array elements must match number of elements in MACH array.
     * Not used for WGPLNF, VTPLNF and VFPLHF (wing and vertical tail) name lists.
     */
    rlph: Option<Vec<f32>>,
 
}

pub struct Vxplnf {
    /** Tip chord, length */
    chrotp: f32,
    /** Semi-span outboard panel, length.
     * Not Required for straight tapered planform.
     */
    sspnop: Option<f32>,
    /** Semi-span exposed panel, length */
    sspne: f32,
    /** Semi-span theoretical panel from theoretical root chord, length */
    sspn: f32,
    /** Chord at brake point, length.
     * Not Required for straight tapered planform
     */
    chrobp: Option<f32>,
    /** Root Chord, Length */
    chrdr: f32,
    /** Inboard Panel sweep angle, degrees */
    savsi: f32,
    /** Outboard panel sweep angle, degrees.
     * Not Required for straight tapered planform
     */
    savso: Option<f32>,
    /** Reference chord station for inboard and outboard
     * panel sweep angles. fraction of chord 0.0 to 1.0
     */
    chstat: f32,
    /** 1= straight tapered planform
     * 2= double delta planform (AR <= 3)
     * 3= cranked planform (AR > 3).
     */
    fptype: i32,
    /** Portion of exposed vertical panel area that lies
     * between mach lines eminating from leading and trailing 
     * edges of wing exposed root chord.
     * Only required for supersonic and hypersonic speed regines. Number of 
     * array elements must match number of elements in MACH array. Area.
     * Not used for WGPLNF and HTPLNF name lists.
     */
    svwb: Option<Vec<f32>>,
    /** Area of exposed vertical panel not influensed by wing or
     * horizontal tail. Area.
     * Only required for supersonic and hypersonic speed regines. Number of 
     * array elements must match number of elements in MACH array. Area.
     * Not used for WGPLNF and HTPLNF name lists.
     */
    svb: Option<Vec<f32>>,
    /** Portion of exposed vertical panel area that lies between
     * mach lines eminating from leading and trailing edges 
     * of horizonal tail exposed root chord. ARea.
     * Only required for supersonic and hypersonic speed regines. Number of 
     * array elements must match number of elements in MACH array. Area.
     * Not used for WGPLNF and HTPLNF name lists.
     */
    svhb: Option<Vec<f32>>
}
