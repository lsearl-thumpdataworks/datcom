/// -----
/// File: /src/airfoilSectMod.rs  
/// Project: datcon  
/// File Created: Wednesday, 14th Nov 182018 12:31:13 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Wednesday, 14th November 2018 12:31:15 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----

use utility::tbfunx;


// ##################################################

#[derive(Debug)]
#[derive(PartialEq)]
pub enum XycoordErr {
    /// An airfoil Y coordinate had an upper value less than the lower value.
    InvertedYcoord(usize),
    /// Either the first or last element of the specified camber was not 0.0.
    CamberFirstLastNotZero,
    /// Either the first or last element of the specified thickness was not 0.0.
    ThicknessFirstLastNotZero,
    /// Missing either thickness or chamber array (need both.)
    PartOfThicknessChamber,
    /// Miss either Y upper or Y lower. Need both.
    PartOfYUpperLower,
    /// The array lengths (either thickness&camber or Yupper&Ylower) do not match X length.
    NotXlenMatch,
    /// At least 3 elements are needed in the arrays.
    ArraysTooShort,
    /// Cant have yupper&ylower & mean&camber all specified
    TooMuchSpecified,
    /// Either the first element of x is not 0 or the last element of ex is not 1.0.
    XFirstFirstNotZeroLastNotOne,
    /// Some element of the X array was not in the correct order.
    InvertedXcoord(usize),
    /// We had an error calling Tbfunx at element I when dealing with the camber array.
    TbfunxCamberErr(usize, tbfunx::TbfunxErr)
}

/// Implements the functions of the Airfile Section Module of DATCOM
/// 

///
/// Type of flying surface we are dealing with.
/// 
//pub enum FlyingSurfaceType {
 //   UNKNOWN,
 //   /// Forward horzinotal flying surface (wing, canard)
 //   /// =1 in fortran DATCOM
 //   WING,
 //   /// Aft horizontal flying surface (wing, tail)
 //   /// =2 in fortan DATCOM
 //   HORZ_TAIL,
 //   /// Vertical tail.
 //   /// =3 in fortran DATCOM
 //   VERT_TAIL,
 //   /// Ventral fin.
  //  /// =4 in fortran DATCOM
  //  VENT_FIN
//}

/// Calculate airfoil X,Y coordinates or Mean,Camber distrubutions.
/// The old IVAL (first argument) has been replaced with some logic.
///
/// * x - x coordinates for airfoil.
/// * return (x_low_absc, x_upper_absc, y_low_ord, y_up_ord, thickness, chamber)
/// ** x_low_absc - x lower abscissa, normal to the meanline
/// ** y_up_absc - x upper abscissa, normal to the meanline
/// ** y_low_ord - y lower ordinate, normal to the meanline
/// ** y_up_ord - y upper ordinate, normal to the meanline
/// ** thickenss - 1/2 or the user thickness
/// ** camber - camber value.
pub fn xychord (x: &Vec<f32>, y_lower: &Option<Vec<f32>>, y_upper: &Option<Vec<f32>>,
                thickness: &Option<Vec<f32>>, camber: &Option<Vec<f32>>) 
            -> Result<(Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>), XycoordErr> {

    // Do some sanity checking on the arrays.
    // Make sure the programmer specified pairs (uppper and lower) or (thickness and chamber).
    if !(y_lower.is_some() && y_upper.is_some()) {
        return Err(XycoordErr::PartOfYUpperLower)
    }
    if !(thickness.is_some() && camber.is_some())  {
        return Err(XycoordErr::PartOfThicknessChamber)
    }
    // Make sure the programmer did not specify all 4.
    if y_lower.is_some() && y_upper.is_some() && thickness.is_some() && camber.is_some() {
        return Err(XycoordErr::TooMuchSpecified)
    }
    // Make sure that arrays are the same length
    if x.len() < 3 {
        return Err(XycoordErr::ArraysTooShort)
    }
 
    if y_lower.is_some() {
        if (x.len() != y_lower.as_ref().unwrap().len()) || (x.len() != y_upper.as_ref().unwrap().len()) {
            return Err(XycoordErr::NotXlenMatch);
        }
    }
    else {
        if x.len() == thickness.as_ref().unwrap().len() || x.len() == camber.as_ref().unwrap().len() {
            return Err(XycoordErr::NotXlenMatch);
        }
    }

    let last = x.len() - 1; // index to last element of the arrays.

    let mut tmp_thick: Vec<f32>; // temporary airfoil thickness array.
    let mut tmp_camber: Vec<f32>; // temporary airfoil mean camber array.

    // verify that the X coordinates are good.
    if x[0] != 0.0f32 || x[last] != 1.0f32 {
        return Err(XycoordErr::XFirstFirstNotZeroLastNotOne);
    }

    // Make sure the X array elements are in sorted order.
    for i in 0..last {
        if x[i] >= x[i+1]{
            return Err(XycoordErr::InvertedXcoord(i));
        }
    }

    if y_lower.is_some() {
        // we need to calculate the mean and camber from the upper and lower airfoil coordinates.
        let upper = y_upper.as_ref().unwrap();
        let lower = y_lower.as_ref().unwrap();
        tmp_thick = Vec::with_capacity(lower.len());
        tmp_camber = Vec::with_capacity(lower.len());
        for i in 0..lower.len() {
            // top coord can not be belower lower coord.
            if upper[i] >= lower[i] {
                return Err(XycoordErr::InvertedYcoord(i));
            }
            // TODO: figure out why camber and thickess are *1/2.
            tmp_thick.push(0.5f32 * (upper[i] - lower[i]));
            tmp_camber.push(0.5f32 * (upper[i] + lower[i]));

            // Incase there was a round off error, zeor out the first and last values.
            tmp_thick[0] = 0.0f32;
            tmp_thick[last] = 0.0f32;

            tmp_camber[0] = 0.0f32;
            tmp_camber[last] = 0.0f32;
        }
    }
    else {
        tmp_thick = thickness.as_ref().unwrap().clone();
        if (tmp_thick[0] != 0.0f32) || (tmp_thick[last] != 0.0f32) {
            return Err(XycoordErr::ThicknessFirstLastNotZero);
        }
        tmp_camber = camber.as_ref().unwrap().clone();
        if (tmp_camber[0] != 0.0f32) || (tmp_camber[last] != 0.0f32) {
            return Err(XycoordErr::CamberFirstLastNotZero);
        }
    }

    // Upper abscissa: normal to mean line
    let mut x_up_absc: Vec<f32> = Vec::with_capacity(x.len());
    // Lower abscisa
    let mut x_low_absc: Vec<f32> = Vec::with_capacity(x.len());
    // Upper Ordinate: normal to mean line
    let mut y_up_ord: Vec<f32> = Vec::with_capacity(x.len());
    // lower Ordinate: normal to mean line
    let mut y_low_ord: Vec<f32> = Vec::with_capacity(x.len());
    
    for i in 0..x.len() {
        // Get the slope of the camber at each x
        let mut slope =
            match tbfunx::tbfunx(x[i], x, &tmp_camber, &tbfunx::BeyondExtentBehaviour::Ylimit,
                                    &tbfunx::BeyondExtentBehaviour::Ylimit) {
                Ok((_, dydx)) => dydx,
                Err(e) => return Err(XycoordErr::TbfunxCamberErr(i, e))
        };

        let theta = slope.atan();
        let sin_angle = theta.sin();
        let cos_angle = theta.cos();

        x_up_absc.push(x[i] - tmp_thick[i] * sin_angle);
        x_low_absc.push(x[i] + tmp_thick[i] * sin_angle);
        y_up_ord.push(tmp_camber[i] + tmp_thick[i] * cos_angle);
        y_low_ord.push(tmp_camber[i] - tmp_thick[i] * cos_angle);
 
    }

    // Incase there were sum roundoff errors fix the first and last values
    x_low_absc[0] = 0.0f32;
    x_low_absc[last] = 1.0f32;
    x_up_absc[0] = 0.0f32;
    x_up_absc[last] = 1.0f32;

    y_up_ord[0] = 0.0f32;
    y_up_ord[last] = 0.0f32;
    y_low_ord[0] = 0.0f32;
    y_low_ord[last] = 0.0f32;

    Ok((x_low_absc, x_up_absc, y_low_ord, y_up_ord, tmp_thick, tmp_camber))
    
}
