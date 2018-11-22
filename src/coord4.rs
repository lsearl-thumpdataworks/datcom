/// -----
/// File: /src/coord4.rs  
/// Project: datcon  
/// File Created: Friday, 16th Nov 182018 11:04:46 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Friday, 16th November 2018 11:04:47 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----
/// 
/// CALCULATE NACA FOUR DIGIT AIRFOIL COORDINATES
/// 

use airfol;

/// Calculate the NACA four digit airfoil coordinates.
pub fn coord4 (design: &Vec<char>) -> airfol::AirfoilCoords {
    let digs: Vec<u32> = design.into_iter().map(|c| c.to_digit(10).unwrap()).collect();

    // First digit is the maximum camber as percentage of chord.
    let camber_max = (digs[0] as f32) * 0.01f32; // ZM
    // second digit is the position of the maximum camber along the coord in increments fo 10%
    let camber_pos = (digs[1] as f32) * 0.1f32; //ZP
    // The 3rd and 4th digits are the maximum thickeness of the chord as a percentage of chord.
    let thickness_max = ((digs[2] * 10 + digs[3]) as f32) * 0.01f32;

    let x_coord = airfol::nasa_x_coords();

    let mut thickness: Vec<f32> = Vec::with_capacity(x_coord.len()); // thinkness of the airfoil at chord positions
    let mut camber: Vec<f32> = Vec::with_capacity(x_coord.len()); // camber of airfoil at chord positions.

    let mut y_upper: Vec<f32> = Vec::with_capacity(x_coord.len()); 
    let mut y_lower: Vec<f32> = Vec::with_capacity(x_coord.len()); 
    let mut x_upper: Vec<f32> = Vec::with_capacity(x_coord.len()); 
    let mut x_lower: Vec<f32> = Vec::with_capacity(x_coord.len()); 

    thickness.push(0.0f32);
    camber.push(0.0f32);
    y_upper.push(0.0f32);
    y_lower.push(0.0f32);
    x_upper.push(0.0f32);
    x_lower.push(0.0f32);
   
    for i in 1..x_coord.len()-1 {
        let x = x_coord[i];
        let alpha: f32;
        let mut thick: f32;
        let mut cam: f32;
        // Note: The last coefficient is change from 0.1015 to 0.1036 since the original
        // coefficient has the las (at x-1.0) t thickness be too far from 0.0.
        thick = 0.5f32 * thickness_max * ((0.2969f32 * x.sqrt()) - (0.126f32 * x) - (0.3516f32 * x.powi(2)) +
                     (0.2843f32 * x.powi(3)) - (0.1036f32 * x.powi(4)) ); 
        
        // The camber uses a different equation when x < max camber position
        if x < camber_pos {
            cam = (2.0f32 * camber_pos * x - x.powi(2)) * camber_max / camber_pos.powi(2);
            alpha = 2.0f32 * camber_max / (camber_pos.powi(2)) * (camber_pos - x).atan();
        } 
        else if x == camber_pos {
            cam = camber_max;
            alpha = 0.0;
        }
        else {
            cam = (camber_max /((1.0f32 - camber_pos).powi(2))) *
                             ((1.0f32 -2.0f32 * camber_pos) + 2.0f32 * camber_pos * x - x.powi(2));
            alpha = ((2.0f32 * camber_max / ((1.0f32 - camber_pos).powi(2))) * (camber_pos - x)).atan();

        }
         
        x_upper.push(x - (thick * alpha.sin()));
        x_lower.push(x + (thick * alpha.sin()));

        y_lower.push(x + (thick * alpha.cos()));
        y_upper.push(x - (thick * alpha.cos()));

        if cam < 1.0e-5f32 {
            cam = 0.0f32;
        }
        camber.push(cam);
        thickness.push(thick);
         
    }

    thickness.push(0.0f32);
    camber.push(0.0f32);
    x_upper.push(0.0f32);
    x_lower.push(0.0f32);
    y_upper.push(1.0f32);
    y_lower.push(1.0f32);

    airfol::AirfoilCoords{x_upper: x_upper, x_lower: x_lower, y_upper: y_upper, y_lower: y_lower,
                            x: x_coord, thick: thickness, camber: camber}
}

//###############################################################################
// Tests
//###############################################################################
#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn naca_0015 () {
        let design = vec!['0', '0', '1', '5'];
        let coords = coord4 (&design);

        println!("");
        format!("{:9}, {:9}, {:9}", "x", "thickness", "camber");
        for i in 0..coords.x.len() {
            format!("{:9}, {:9}, {:9}", coords.x[i], coords.thick[i], coords.camber[i]);
        }

        println!("");

        format!("{:9}, {:9}, {:9}, {:9}", "x_lower", "x_upper", "y_lower", "y_upper");
        for i in 0..coords.x.len() {
            format!("{:9}, {:9}, {:9}, {:9}", coords.x_lower[i], coords.y_lower[i], coords.x_upper[i],
                            coords.y_upper[i]);
        }
    }
}
