/// -----
/// File: /src/case/xxschr.rs...
/// Project: datcon...
/// File Created: Tuesday, 13th Nov 182018 4:32:59 pm...
/// Author: Leon S. Searl...
/// 
/// Last Modified: Tuesday, 13th November 2018 4:44:43 pm...
/// Modified By: Leon S. Searl...
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----


use conf_file;

const INPUT_NAME_NAMELIST_WGSCHR: &'static str = "WGSCHR";
const INPUT_NAME_NAMELIST_HTSCHR: &'static str = "HTSCHR";
const INPUT_NAME_NAMELIST_VTSCHR: &'static str = "VTSCHR";
const INPUT_NAME_NAMELIST_VFSCHR: &'static str = "VFSCHR";

// WGSCHR, HTSCHR, VTSCHR and VFSCHR nameilst variable names.
const INPUT_NAME_VAR_TOVC: &'static str = "TOVC";
const INPUT_NAME_VAR_DELTAY: &'static str = "DELTAY";
const INPUT_NAME_VAR_XOVC: &'static str = "XOVC";
const INPUT_NAME_VAR_CLI: &'static str = "CLI";
const INPUT_NAME_VAR_ALPHAI: &'static str = "ALPHAI";
const INPUT_NAME_VAR_CLALPA: &'static str = "CLALPA";
const INPUT_NAME_VAR_CLMAX: &'static str = "CLMAX";
const INPUT_NAME_VAR_CMO: &'static str = "CMO";
const INPUT_NAME_VAR_LERI: &'static str = "LERI";
const INPUT_NAME_VAR_LERO: &'static str = "LERO";
const INPUT_NAME_VAR_CAMBER: &'static str = "CAMBER";
const INPUT_NAME_VAR_TOVCO: &'static str = "TOVCO";
const INPUT_NAME_VAR_XOVCO: &'static str = "XOVCO";
const INPUT_NAME_VAR_CMOT: &'static str = "CMOT";
const INPUT_NAME_VAR_CLMAXL: &'static str = "CLMAXL";
const INPUT_NAME_VAR_CLAMO: &'static str = "CLAMO";
const INPUT_NAME_VAR_TCEFF: &'static str = "TCEFF";
const INPUT_NAME_VAR_KSHARP: &'static str = "KSHARP";
const INPUT_NAME_VAR_SLOPE: &'static str = "SLOPE";
const INPUT_NAME_VAR_ARCL: &'static str = "ARCL";
const INPUT_NAME_VAR_XAC: &'static str = "XAC";
const INPUT_NAME_VAR_DWASH: &'static str = "DWASH";
const INPUT_NAME_VAR_YCM: &'static str = "YCM";
const INPUT_NAME_VAR_CLD: &'static str = "CLD";
const INPUT_NAME_VAR_TYPEIN: &'static str = "TYPEIN";
const INPUT_NAME_VAR_XCORD: &'static str = "XCORD";
const INPUT_NAME_VAR_YUPPER: &'static str = "YUPPER";
const INPUT_NAME_VAR_YLOWER: &'static str = "YLOWER";
const INPUT_NAME_VAR_MEAN: &'static str = "MEAN";
const INPUT_NAME_VAR_THICK: &'static str = "THICK";

///
/// Wing section Characteristics
const INPUT_SPEC_WGSCHR_TABLE: [conf_file::SpecItem<'static>; 30] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TOVC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DELTAY, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XOVC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ALPHAI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLALPA, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLMAX, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CMO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_LERI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_LERO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CAMBER, required: false, item_type: conf_file::SpecItemType::Boolean, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TOVCO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XOVCO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CMOT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLMAXL, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLAMO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TCEFF, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_KSHARP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_SLOPE, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ARCL, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XAC, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DWASH, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YCM, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLD, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TYPEIN, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XCORD, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YUPPER, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YLOWER, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_MEAN, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_THICK, required: false, item_type: conf_file::SpecItemType::Array, table: None
    }


];

const INPUT_SPEC_HTSCHR_TABLE: [conf_file::SpecItem<'static>; 27] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TOVC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_DELTAY, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XOVC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ALPHAI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLALPA, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLMAX, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CMO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_LERI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_LERO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CAMBER, required: false, item_type: conf_file::SpecItemType::Boolean, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TOVCO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XOVCO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CMOT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLAMO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TCEFF, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_KSHARP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
   conf_file::SpecItem {
        name: INPUT_NAME_VAR_ARCL, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XAC, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YCM, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLD, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TYPEIN, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XCORD, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YUPPER, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YLOWER, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_MEAN, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_THICK, required: false, item_type: conf_file::SpecItemType::Array, table: None
    }


];

///
/// Vertical (fin and tail) section characteristics
const INPUT_SPEC_VSCHR_TABLE: [conf_file::SpecItem<'static>; 17] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TOVC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XOVC, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CLALPA, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_LERI, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_LERO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_CAMBER, required: false, item_type: conf_file::SpecItemType::Boolean, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TOVCO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XOVCO, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TCEFF, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_KSHARP, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ARCL, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TYPEIN, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_XCORD, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YUPPER, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_YLOWER, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_MEAN, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_THICK, required: false, item_type: conf_file::SpecItemType::Array, table: None
    }


];

pub const INPUT_SPEC_WGSCHR_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_WGSCHR, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_WGSCHR_TABLE)
};

pub const INPUT_SPEC_HTSCHR_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_HTSCHR, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_HTSCHR_TABLE)
};

pub const INPUT_SPEC_VTSCHR_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_VTSCHR, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_VSCHR_TABLE)
};

pub const INPUT_SPEC_VFSCHR_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_VFSCHR, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_VSCHR_TABLE)
};


/** Variables of the various SectionCharacteristics namelists
 *  (WGSCHR, HTSCHR, VTSCHR, VFSCHR)
 */
pub struct Wgschr {
    /** Maximum airfoil section thickness, fraction of chord. 
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Can be computed of NACA card supplied.
    */
    tovc: Option<f32>,
    /** Difference between airfoil  ordinates at 6.0% and 0.15% chord, 
     * percent chord.
     * Used for WGSCHR and HTSCHR.
     * For all speed regimes.
     * Can be computed of NACA card supplied.
    */
    deltay: Option<f32>,
    /** Chord location of maximum airfoil thickness, fraction of chord.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
    * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed of NACA card supplied.
     */
    xovc: Option<f32>,
    /** Airfoild section design lift coefficent.
     * Used for namelist WGSCHR and HTSCHR namelists.
     * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed if NACA card supplied.
     */
    cli: Option<f32>,
    /** Angle of attack at section design lift coefficient. Degrees. 
     * Used for namelist WGSCHR and HTSCHR nameslists.
    * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed if NACA card supplied.
    */
    alphai: Option<f32>,
    /** Airfoil section lift curve slope dCl/d_aplha, per degree.
     * Array elements must coorespond to to the mach or vinf array
     * from Namelist FLTCON.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Used for subsonic (Not transonic, supersonice or hypersonic)
     * Can be computed if NACA card supplied.
     */
    clalpa: Option<Vec<f32>>,
    /** Airfoil section maximum lift coefficent.
     * Array elements must coorespond to to the mach or vinf array
     * from Namelist FLTCON.
     * Used for namelist WGSCHR and HTSCHR namelists.
     * Used for subsonic (Not transonic, supersonice or hypersonic)
     * Can be computed of NACA card supplied.
     */
    clmax: Option<Vec<f32>>,
    /** Section zero lift pitching momemnt coefficient.
     * Used for namelist WGSCHR and HTSCHR namelists .
     * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed of NACA card supplied.
     */
    cmo: Option<f32>,
    /** Airfoil leading edge radius fraction of chord. 
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Used by all speed regimes.
     * Can be computed of NACA card supplied.
     */
    leri: f32,
    /** Airfoild leading edge radius for outboard panel fraction of chord.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Not required for straight tapered wing.
     * For all speed regimes.
     */
    lero: Option<f32>,
    /** Cambered airfoil section flag. True=cambered. 
     * Used for namelist WGSCHR and HTSCHR namelists .
     * Used for subsonic  (Not transonic, supersonice or hypersonic)
     * Can be computed if NACA card supplied.
     */
    camber: Option<bool>,
    /** t/c for outboard panel.
     * Not required for straight tapered wing.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Optional for supersonic and hypersonic.
     */
    tovco: Option<f32>,
    /** x/c max for outboard panel.
     * Not required for straight tapered wing.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Optional for transonic,  supersonic and hypersonic.
    */
    xovco: Option<f32>,
    /** Cm0 for outboard panel.
     * Not required for straight tapered wing.
     * Used for namelist WGSCHR and HTSCHR namelists .
     * For all speed regimes.
     * Optional for supersonic and hypersonic.
     */
    cmot: Option<f32>,
    /** Airfoil maximum lift coefficient at mach equal zero.
     * Used for WGSCHR namelist only.
     * Used for subsonic and transonic.
     * Can be computed if NACA card is present.
     */
    clmaxl: Option<f32>,
    /** Airfoil section lift curve slope at mach equal zero. Per degree.
     * Used for WGSCHR and HTSCHR name lists.
     * Used for transonic only.
     * Can be computed if NACA card is present.
     */
    clamo: Option<f32>,
    /** Planform effective thickness ratio, fraction of chord.
     * For all 4 namelists.
     * For transonic, supersonic and hypersonic.
     * Can be computed if NACA card is present. 
     */
    yceff: Option<f32>,
    /** Wave-drag factor for sharp-nosed airfoil section. Not 
     * input for round nosed airfoils.
     * For all 4 namelists.
     * For transonic, supersonic and hypersonic.
     * Can be computed if NACA card is present. 
     */
    ksharp: Option<f32>,
    /** Airfoil surface slope at 0,20,40,60,80 and 100% chord. Degrees.
     * Positive when the tangent intersects the chord plane forward of the reference
     * chord point.
     * For WGSCHR namelist only.
     * For transonic, supersonic and hypersonic.
     * Can be computed if NACA card is present. 
     */
    slope: Option<Vec<f32>>,
    /** Aspec ration classification (see table 9) 
     * For all 4 namelists.
     * For all speed regmimes. Optional.
    */
    arcl: Option<f32>,
    /** Section aerodynamic center. Fraction of chord
     * (See vol II for default.)
     * Array elements must coorespond to mach or vinf array from
     * FLTCON namelist.
     * For WGSCHR and HTSCHR nameliests.
     * Optionla for subsonic and supersonic.
     * Can be computed if NACA card is used.
     */
    xac: Option<Vec<f32>>,
    /** subsonic downwash method flag:
     * 1=use datcom method 1
     * 2=use datcom method 2
     * 3=use datcom method 3
     * supersonic. Use datcom method 2
     * if DWASH =1 or 2. (See figure 9).
     * Only use for configurations with horzontal tail.
     * For WGSCHR namelist onley.
     * For subsonic and supersonice. Optional.
     * \
    */
    dwash: Option<f32>,
    /**
     * Airfoil maximum camber. Fraction of chord.
     * For WGSCHR and HTSCHR namelists.
     * For all speed regimes.
     * Can be computed if NACA card is used.
     */
    ycm: Option<f32>,
    /**
     * Conical camber design lift coefficient for M=1.0 design.
     * See NACA RM A55G19 (Defaults to 0.0)
     * For WGSCHR and HTSCHR namelists.
     */
    cld: Option<f32>,
    /** Type of airfoil section coordinates input fro airfoil
     * section module:
     * 1= upper and lower surface
     * 2= mean line and thickness distribution (mean and thick)
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     */
    typein: Option<i32>,
    /** The x coordintates for the yupper,ylower,mean and thick arrays
     * First Chord must be at 0.0. Last coord must be at 1.0.
     * Only used if typein=1 or 2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     */
    xcord: Option<Vec<f32>>,
    /**
     * Ordinates of upper surface.
    * Only used if typein=1.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    yupper: Option<Vec<f32>>, 
    /**
     * Ordinates of lower surface.
    * Only used if typein=1.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    ylower: Option<Vec<f32>>,
    /**
     * Ordinates of meain line.
     * Only used if typein= 2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     * 
     */
    mean: Option<Vec<f32>>,
    /**
     * Ordinates of thickness.
    * Only used if typein=2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    thick: Option<Vec<f32>>   
}

pub struct Htschr {
    /** Maximum airfoil section thickness, fraction of chord. 
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Can be computed of NACA card supplied.
    */
    tovc: Option<f32>,
    /** Difference between airfoil  ordinates at 6.0% and 0.15% chord, 
     * percent chord.
     * Used for WGSCHR and HTSCHR.
     * For all speed regimes.
     * Can be computed of NACA card supplied.
    */
    deltay: Option<f32>,
    /** Chord location of maximum airfoil thickness, fraction of chord.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
    * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed of NACA card supplied.
     */
    xovc: Option<f32>,
    /** Airfoild section design lift coefficent.
     * Used for namelist WGSCHR and HTSCHR namelists.
     * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed if NACA card supplied.
     */
    cli: Option<f32>,
    /** Angle of attack at section design lift coefficient. Degrees. 
     * Used for namelist WGSCHR and HTSCHR nameslists.
    * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed if NACA card supplied.
    */
    alphai: Option<f32>,
    /** Airfoil section lift curve slope dCl/d_aplha, per degree.
     * Array elements must coorespond to to the mach or vinf array
     * from Namelist FLTCON.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Used for subsonic (Not transonic, supersonice or hypersonic)
     * Can be computed if NACA card supplied.
     */
    clalpa: Option<Vec<f32>>,
    /** Airfoil section maximum lift coefficent.
     * Array elements must coorespond to to the mach or vinf array
     * from Namelist FLTCON.
     * Used for namelist WGSCHR and HTSCHR namelists.
     * Used for subsonic (Not transonic, supersonice or hypersonic)
     * Can be computed of NACA card supplied.
     */
    clmax: Option<Vec<f32>>,
    /** Section zero lift pitching momemnt coefficient.
     * Used for namelist WGSCHR and HTSCHR namelists .
     * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed of NACA card supplied.
     */
    cmo: Option<f32>,
    /** Airfoil leading edge radius fraction of chord. 
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Used by all speed regimes.
     * Can be computed of NACA card supplied.
     */
    leri: f32,
    /** Airfoild leading edge radius for outboard panel fraction of chord.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Not required for straight tapered wing.
     * For all speed regimes.
     */
    lero: Option<f32>,
    /** Cambered airfoil section flag. True=cambered. 
     * Used for namelist WGSCHR and HTSCHR namelists .
     * Used for subsonic  (Not transonic, supersonice or hypersonic)
     * Can be computed if NACA card supplied.
     */
    camber: Option<bool>,
    /** t/c for outboard panel.
     * Not required for straight tapered wing.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Optional for supersonic and hypersonic.
     */
    tovco: Option<f32>,
    /** x/c max for outboard panel.
     * Not required for straight tapered wing.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Optional for transonic,  supersonic and hypersonic.
    */
    xovco: Option<f32>,
    /** Cm0 for outboard panel.
     * Not required for straight tapered wing.
     * Used for namelist WGSCHR and HTSCHR namelists .
     * For all speed regimes.
     * Optional for supersonic and hypersonic.
     */
    cmot: Option<f32>,
    /** Airfoil section lift curve slope at mach equal zero. Per degree.
     * Used for HGSCHR and HTSCHR name lists.
     * Used for transonic only.
     * Can be computed if NACA card is present.
     */
    clamo: Option<f32>,
    /** Planform effective thickness ratio, fraction of chord.
     * For all 4 namelists.
     * For transonic, supersonic and hypersonic.
     * Can be computed if NACA card is present. 
     */
    yceff: Option<f32>,
    /** Wave-drag factor for sharp-nosed airfoil section. Not 
     * input for round nosed airfoils.
     * For all 4 namelists.
     * For transonic, supersonic and hypersonic.
     * Can be computed if NACA card is present. 
     */
    ksharp: Option<f32>,
   /** Aspec ration classification (see table 9) 
     * For all 4 namelists.
     * For all speed regmimes. Optional.
    */
    arcl: Option<f32>,
    /** Section aerodynamic center. Fraction of chord
     * (See vol II for default.)
     * Array elements must coorespond to mach or vinf array from
     * FLTCON namelist.
     * For WGSCHR and HTSCHR nameliests.
     * Optionla for subsonic and supersonic.
     * Can be computed if NACA card is used.
     */
    xac: Option<Vec<f32>>,
    /**
     * Airfoil maximum camber. Fraction of chord.
     * For WGSCHR and HTSCHR namelists.
     * For all speed regimes.
     * Can be computed if NACA card is used.
     */
    ycm: Option<f32>,
    /**
     * Conical camber design lift coefficient for M=1.0 design.
     * See NACA RM A55G19 (Defaults to 0.0)
     * For WGSCHR and HTSCHR namelists.
     */
    cld: Option<f32>,
    /** Type of airfoil section coordinates input fro airfoil
     * section module:
     * 1= upper and lower surface
     * 2= mean line and thickness distribution (mean and thick)
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     */
    typein: Option<i32>,
    /** The x coordintates for the yupper,ylower,mean and thick arrays
     * First Chord must be at 0.0. Last coord must be at 1.0.
     * Only used if typein=1 or 2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     */
    xcord: Option<Vec<f32>>,
    /**
     * Ordinates of upper surface.
    * Only used if typein=1.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    yupper: Option<Vec<f32>>, 
    /**
     * Ordinates of lower surface.
    * Only used if typein=1.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    ylower: Option<Vec<f32>>,
    /**
     * Ordinates of meain line.
     * Only used if typein= 2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     * 
     */
    mean: Option<Vec<f32>>,
    /**
     * Ordinates of thickness.
    * Only used if typein=2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    thick: Option<Vec<f32>>   
}

pub struct Vxschr {
    /** Maximum airfoil section thickness, fraction of chord. 
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Can be computed of NACA card supplied.
    */
    tovc: Option<f32>,
    /** Chord location of maximum airfoil thickness, fraction of chord.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
    * Used for subsonic and transonic (Not supersonice or hypersonic)
     * Can be computed of NACA card supplied.
     */
    xovc: Option<f32>,
    /** Airfoil section lift curve slope dCl/d_aplha, per degree.
     * Array elements must coorespond to to the mach or vinf array
     * from Namelist FLTCON.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Used for subsonic (Not transonic, supersonice or hypersonic)
     * Can be computed if NACA card supplied.
     */
    clalpa: Option<Vec<f32>>,
    /** Airfoil leading edge radius fraction of chord. 
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Used by all speed regimes.
     * Can be computed of NACA card supplied.
     */
    leri: f32,
    /** Airfoild leading edge radius for outboard panel fraction of chord.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * Not required for straight tapered wing.
     * For all speed regimes.
     */
    lero: Option<f32>,
    /** t/c for outboard panel.
     * Not required for straight tapered wing.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Optional for supersonic and hypersonic.
     */
    tovco: Option<f32>,
    /** x/c max for outboard panel.
     * Not required for straight tapered wing.
     * Used for WGSCHR, HTSCHR, VTSCHR and VFSCHR namelists
     * For all speed regimes.
     * Optional for transonic,  supersonic and hypersonic.
    */
    xovco: Option<f32>,
    /** Planform effective thickness ratio, fraction of chord.
     * For all 4 namelists.
     * For transonic, supersonic and hypersonic.
     * Can be computed if NACA card is present. 
     */
    yceff: Option<f32>,
    /** Wave-drag factor for sharp-nosed airfoil section. Not 
     * input for round nosed airfoils.
     * For all 4 namelists.
     * For transonic, supersonic and hypersonic.
     * Can be computed if NACA card is present. 
     */
    ksharp: Option<f32>,
    /** Aspec ration classification (see table 9) 
     * For all 4 namelists.
     * For all speed regmimes. Optional.
    */
    arcl: Option<f32>,
    /** Type of airfoil section coordinates input fro airfoil
     * section module:
     * 1= upper and lower surface
     * 2= mean line and thickness distribution (mean and thick)
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     */
    typein: Option<i32>,
    /** The x coordintates for the yupper,ylower,mean and thick arrays
     * First Chord must be at 0.0. Last coord must be at 1.0.
     * Only used if typein=1 or 2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     */
    xcord: Option<Vec<f32>>,
    /**
     * Ordinates of upper surface.
    * Only used if typein=1.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    yupper: Option<Vec<f32>>, 
    /**
     * Ordinates of lower surface.
    * Only used if typein=1.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    ylower: Option<Vec<f32>>,
    /**
     * Ordinates of meain line.
     * Only used if typein= 2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
     * 
     */
    mean: Option<Vec<f32>>,
    /**
     * Ordinates of thickness.
    * Only used if typein=2.
     * All special characteristcs namelists.
     * All Speed regimes (optional)
      * 
     */
    thick: Option<Vec<f32>>   
}
