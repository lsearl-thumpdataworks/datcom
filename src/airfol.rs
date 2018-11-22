/// -----
/// File: /src/airfol.rs  
/// Project: datcon  
/// File Created: Thursday, 15th Nov 182018 8:17:46 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Thursday, 15th November 2018 8:17:47 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----
/// 
use case;

use decode;
use coord4;

/// Coordinate for an airfoil.
pub struct AirfoilCoords {
    /// Thickness is is calulcated 'normal' to the mean line.
    /// This means this means that x for the upper surface and lower
    /// surface are not at the same spot.
    pub x_upper: Vec<f32>,
    /// Thickness is is calulcated 'normal' to the mean line.
    /// This means this means that x for the upper surface and lower
    /// surface are not at the same spot.
    pub x_lower: Vec<f32>,
    /// height of upper surface
    pub y_upper: Vec<f32>,
    /// height of lower surface
    pub y_lower: Vec<f32>,
    /// x coodinates for thickness and camber.
    pub x: Vec<f32>,
    /// Thickness of airfoil
    pub thick: Vec<f32>,
    /// Camber of airfoil
    pub camber: Vec<f32>
}

/// Calculate the X coordinates to use for NACA airfoils.
/// X SPACING IS CLOSEST IN THE NEIGHBORHOOD OF THE L.E.
pub fn nasa_x_coords() -> Vec<f32> {
    let mut x: Vec<f32> = Vec::new();
    let mut delta_x = 0.00100f32;
    let mut lastx: f32 = 0.0f32;
    x.push(lastx);
    // no more than 60 x positions
    for _i in 1..61 {
        lastx = lastx + delta_x;
        if lastx >= 1.0f32 {
            x.push(1.0f32);
            break;
        }
        else {
            x.push(lastx);
        }
        
        delta_x = if lastx > 0.79f32 {
                0.02f32
            }
            else if lastx > 0.29f32 {
                0.05f32
            }
            else if lastx >= 0.01f32 {
                0.01f32
            }
            else { delta_x }
    }
    x
}



// #######################

/// Calculate the info for the airfoil.
/// * naca_info: the Naca coard
/// * return 
/// ** Ok - the airfoil coorindates.
pub fn airfol(naca_info: &case::NacaCard ) -> Result<AirfoilCoords, String>{
    // figure out what kind of airfoil we are to use
    let (series, design) = match decode::decode(naca_info) {
        Ok((s,d)) => (s,d),
        Err(e) => return Err(e)
    };

    // Depending on which series we need, get the coordinates for the airfoil.
    match series {
        case::NacaSeries::FourDigit => {
            Ok(coord4::coord4(&design))
        },
        case::NacaSeries::FourDigitModified => unimplemented!(),
        case::NacaSeries::FiveDigit => unimplemented!(),
        case::NacaSeries::FiveDigitModified => unimplemented!(),
        case::NacaSeries::OneSeries => unimplemented!(),
        case::NacaSeries::SixSeries => unimplemented!(),
        case::NacaSeries::Supersonic => unimplemented!()
    }
}

//###############################################################################
// Tests
//###############################################################################
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn naca_x_coords () {
        let x_coords = nasa_x_coords();
        println!("");

        for x in x_coords {
            println!("{}", x)
        }
    }
}



