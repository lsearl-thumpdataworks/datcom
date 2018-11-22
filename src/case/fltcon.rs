/*
 * Filename: .../datcom/src/namelist/fltcon.rs
 * Path: .../datcom/src/namelist
 * Created Date: Tuesday, November 13th 2018, 10:33:24 am
 * Author: Leon S. Searl
 * 
 * Copyright (c) 2018 Thump Data Works, Inc.
 */

// #########################################################################

use conf_file;
use toml_edit;
use case::errors;

/// Name for the FLTCON namelist. This is in namelist GroupI
pub const INPUT_NAME_NAMELIST_FLTCON: &'static str = "FLTCON";

/// Variable names for the FLTCON namelist 
pub const INPUT_NAME_VAR_MACH: &'static str = "MACH";
pub const INPUT_NAME_VAR_VINF: &'static str = "VINF";
pub const INPUT_NAME_VAR_ALSCHD: &'static str = "ALSCHD";
pub const INPUT_NAME_VAR_RNNUB: &'static str = "RNNUB";
pub const INPUT_NAME_VAR_ALT: &'static str = "ALT";
pub const INPUT_NAME_VAR_PINF: &'static str = "PINF";
pub const INPUT_NAME_VAR_TINF: &'static str = "TINF";
pub const INPUT_NAME_VAR_HYPERS: &'static str = "HYPERS";
pub const INPUT_NAME_VAR_STMACH: &'static str = "STMACH";
pub const INPUT_NAME_VAR_TSMACH: &'static str = "TSMACH";
pub const INPUT_NAME_VAR_TR: &'static str = "TR";
/// Weight of the aircraft.
pub const INPUT_NAME_VAR_WT: &'static str = "WT";
pub const INPUT_NAME_VAR_GAMMA: &'static str = "GAMMA";
pub const INPUT_NAME_VAR_LOOP: &'static str = "LOOP";

// FLTCON table
const INPUT_SPEC_FLTCON_TABLE: [conf_file::SpecItem<'static>; 14] = [
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_MACH, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_VINF, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ALSCHD, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_RNNUB, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_ALT, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_PINF, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TINF, required: false, item_type: conf_file::SpecItemType::Array, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_HYPERS, required: false, item_type: conf_file::SpecItemType::Boolean, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_STMACH, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TSMACH, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_TR, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_WT, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_GAMMA, required: false, item_type: conf_file::SpecItemType::Float, table: None
    },
    conf_file::SpecItem {
        name: INPUT_NAME_VAR_LOOP, required: false, item_type: conf_file::SpecItemType::Integer, table: None
    }
];

/// The SPEC item of the FLTCON name list to be included inthe case table.
pub const INPUT_SPEC_FLTCON_ITEM: conf_file::SpecItem<'static> = conf_file::SpecItem {
        name: INPUT_NAME_NAMELIST_FLTCON, required: false, item_type: conf_file::SpecItemType::Table,
        table: Some(&INPUT_SPEC_FLTCON_TABLE)
};

// #################################################################################

/// The FLTCON information structure.
pub struct Fltcon {
    /** Values of Freestream mach number.
     * REquired when using the NACA control card.
     * Computed if the following combinations of variables are specified:
     *  (vinf, alt),
     *  (pinf,tinf, pinf)
     */
    pub mach: Option<Vec<f32>>,
    /** Values of freestream speed. Length/Time.
     * Computed if the following conbinations of variables are specified:
     *  (pinf, tinf, mach)
     */
    pub vinf: Option<Vec<f32>>,
    /** Values of angels of attack, tabulated in ascending order. Degrees. */
    pub alschd: Option<Vec<f32>>,
    /** Reynolds number per unit length, (rho * V / mu). 1/Length.
     * Required when using the NACA control card.
     * Each Array element must correspond to the respective
     * mach number/freestream speed input. Use LOOP=1.
     * Computed if the offlowing cominbations of variables are specified.
     *  (mach, alt),
     *  (vinf, alt),
     *  (pinf, tinf, vinf),
     *  (pinf, tinf, mach)
     */
    pub rnnub: Option<Vec<f32>>,
    /** Values of geometric altitudes. Length.
     * Atmosphereic conditions are input as either altitude or pressure and
     * temperature.
     */
    pub alt: Option<Vec<f32>>,
    /** Value sof freeestream astatic pressure. Force/Area.
     * Atmosphereic conditions are input as either altitude or pressure and
     * temperature.
     * Required for Transverse Jet Control.
     * Computed if the following combinations of variagles are specified.
     *  (mach, alt),
     *  (vinf, alt),
     */
    pub pinf: Option<Vec<f32>>,
    /** Values of Freestream temperature.
     * Atmosphereic conditions are input as either altitude or pressure and
     * temperature. Degrees
     * Computed if the following combinations of variables are specified:
     *  (mach, alt),
     *  (vinf, alt)
     * 
     */
    pub tinf: Option<Vec<f32>>,
    /** =true, hypersonic analysis at all mach numbes >= 1.4. */
    pub hypers: Option<bool>,
    /** Upper limit of machnumbers of subsonic analysis (0.6 <= stmach <= 0.99).
     * Default to 0.6 if not input.
     */
    pub stmach: Option<f32>,
    /** Lower limit of mach numbers for supersonic anlysis. (1.01<=tsmach<=1.4).
     * Default to 1.4 if not input.
     */
    pub tsmach: Option<f32>,
    /** Drag due to lift transtion flag. For Regression
     * analysis of wing=body configurations.
     * =0 for no transition. (Default.)
     * =1 for transition strips or full scale flight.
    */ 
    pub tr: Option<i32>,
    /** vehicle weight. Force */
    pub wt: Option<f32>,
    /** Flight path angle. Degrees. */
    pub gamma: Option<f32>,
    /** Program looping control.
     * =1 vary altitude and mach together, Default.
     * =2 vary mach, at fixed altitude.
     * =3 very altitude, at fixed mach.
     */
    pub loop_cntl: Option<i32>
}

impl Fltcon {
    pub fn parse_toml (case_table: &toml_edit::Table, caseid: &String,
                        errs_sofar: &mut Vec<errors::CaseCheckErr>) -> Option<Self> {

        let mut errs: Vec<errors::CaseCheckErr> = Vec::new(); 

        // Work on FLTCON namelist
        let result = match case_table[INPUT_NAME_NAMELIST_FLTCON].as_table() {
            None => None,
            Some(table) => {
                // Make the FLTCON struct.
                let mach: Option<Vec<f32>> = 
                    errors::check_f32_array(&table, caseid, INPUT_NAME_NAMELIST_FLTCON,
                                    INPUT_NAME_VAR_MACH, false, &Some(0.0), &None, &mut errs);
                let vinf: Option<Vec<f32>> = 
                   errors::check_f32_array(&table, caseid, INPUT_NAME_NAMELIST_FLTCON,
                                    INPUT_NAME_VAR_VINF, false, &Some(0.0), &None, &mut errs);
                let alschd: Option<Vec<f32>> = 
                    errors::check_f32_array(&table, caseid, INPUT_NAME_NAMELIST_FLTCON,
                                    INPUT_NAME_VAR_ALSCHD, false, &Some(-90.0), &Some(90.0), &mut errs);
                let rnnub: Option<Vec<f32>> = 
                    errors::check_f32_array(&table, &caseid, INPUT_NAME_NAMELIST_FLTCON,
                                    INPUT_NAME_VAR_RNNUB, false,  &Some(0.0), &None, &mut errs);
                let alt: Option<Vec<f32>> = 
                    errors::check_f32_array(&table, &caseid, INPUT_NAME_NAMELIST_FLTCON,
                                    INPUT_NAME_VAR_ALT, false, &None, &None, &mut errs);
                let pinf : Option<Vec<f32>> = 
                    errors::check_f32_array(&table, caseid, INPUT_NAME_NAMELIST_FLTCON,
                                    INPUT_NAME_VAR_PINF, false, &None, &None, &mut errs);
                let tinf : Option<Vec<f32>> = 
                    errors::check_f32_array(&table, caseid, INPUT_NAME_NAMELIST_FLTCON,
                                    INPUT_NAME_VAR_PINF, false, &None, &None, &mut errs);
                let hypers : Option<bool> = table[INPUT_NAME_VAR_HYPERS].as_bool();
                let stmach: Option<f32> =
                    errors::check_f32(&table, caseid, INPUT_NAME_NAMELIST_FLTCON, 
                                    INPUT_NAME_VAR_STMACH, &Some(0.0), &None, &mut errs);
                let tsmach : Option<f32> =
                    errors::check_f32(&table, &caseid, INPUT_NAME_NAMELIST_FLTCON, 
                                    INPUT_NAME_VAR_TSMACH, &Some(0.0), &None, &mut errs);
                let tr : Option<i32> =
                    errors::check_i32(&table, caseid, INPUT_NAME_NAMELIST_FLTCON, 
                                    INPUT_NAME_VAR_TR, &Some(0), &Some(1), &mut errs);
                let wt : Option<f32> =
                    errors::check_f32(&table, caseid, INPUT_NAME_NAMELIST_FLTCON, 
                                    INPUT_NAME_VAR_WT, &Some(0.0), &None, &mut errs);
                let gamma : Option<f32> =
                    errors::check_f32(&table, caseid, INPUT_NAME_NAMELIST_FLTCON, 
                                    INPUT_NAME_VAR_GAMMA, &Some(-90.0), &Some(90.0), &mut errs);
                let loop_cntl : Option<i32> = 
                    errors::check_i32(&table, caseid, INPUT_NAME_NAMELIST_FLTCON, 
                                    INPUT_NAME_VAR_LOOP, &Some(0), &Some(3), &mut errs);

                Some(Fltcon{mach: mach, vinf: vinf, alschd: alschd, rnnub: rnnub, 
                                alt: alt, pinf: pinf, tinf: tinf, hypers: hypers, stmach: stmach,
                                tsmach: tsmach, tr: tr, wt: wt, gamma: gamma, loop_cntl: loop_cntl})
            }
        };

        errs_sofar.append(&mut errs);
        result
    }
}

#[cfg(test)]

mod tests {
    use toml_edit;
    use case::fltcon;
    use case;

    #[test]
    fn empty_toml() {
        // Need to construct the TOML
        let empty_toml = toml_edit::Table::new();
        let caseid = String::from("0");
        let mut errs: Vec<case::errors::CaseCheckErr>  = Vec::new();
        let a_fltcon_opt = fltcon::Fltcon::parse_toml(&empty_toml, &caseid, &mut errs);


        assert!(errs.len() == 0);
        assert!(a_fltcon_opt.is_none());
        
    }

    
    
    #[test]
    fn fltcon_mach_len2() {
        // Need to construct the TOML
        let mut a_doc_case = toml_edit::Table::default();
        a_doc_case[fltcon::INPUT_NAME_NAMELIST_FLTCON] = toml_edit::table();
        let mut a_doc_mach = toml_edit::Array::default(); // empty array
        let item1:f32 = 9.0;
        let item2:f32 = 10.0;
        a_doc_mach.push(item1 as f64);
        a_doc_mach.push(item2 as f64);
        a_doc_case[fltcon::INPUT_NAME_NAMELIST_FLTCON][fltcon::INPUT_NAME_VAR_MACH] =
            toml_edit::value(a_doc_mach);
        let caseid = String::from("0");
        let mut errs: Vec<case::errors::CaseCheckErr>  = Vec::new();
        let a_fltcon_opt = fltcon::Fltcon::parse_toml(&a_doc_case, &caseid, &mut errs);


        assert!(errs.len() == 0);
        assert!(a_fltcon_opt.is_some());

        let a_fltcon = a_fltcon_opt.unwrap();
        assert!(a_fltcon.mach.is_some());
        let a_mach = a_fltcon.mach.unwrap();
        assert!(a_mach.len() == 2);
        assert!(a_mach[0] == item1);
        assert!(a_mach[1] == item2);

        assert!(a_fltcon.vinf.is_none());
        assert!(a_fltcon.alschd.is_none());
        assert!(a_fltcon.rnnub.is_none());
        assert!(a_fltcon.alt.is_none());
        assert!(a_fltcon.pinf.is_none());
        assert!(a_fltcon.tinf.is_none());
        assert!(a_fltcon.hypers.is_none());
        assert!(a_fltcon.tsmach.is_none());
        assert!(a_fltcon.tr.is_none());
        assert!(a_fltcon.wt.is_none());
        assert!(a_fltcon.gamma.is_none());
        assert!(a_fltcon.loop_cntl.is_none());
    }
}
