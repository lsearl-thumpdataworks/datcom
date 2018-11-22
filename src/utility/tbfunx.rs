/// -----
/// File: /src/utlilty/tbfunx.rs  
/// Project: datcon  
/// File Created: Wednesday, 14th Nov 182018 3:31:10 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Wednesday, 14th November 2018 3:31:10 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----
/// 

use std::fmt::{Display, Formatter};
use std;
use utility;

// ############################################################
///
#[derive(Debug)]
#[derive(PartialEq)]
pub enum TbfunxErr {
    /// Missmatch between array lengths.
    /// * usize - first array length.
    /// * usize - second array length.
    DataLenMissmatch(usize, usize),
    /// The arrays were empty.
    NoData,
    /// The values in an array were not in increasing order.
    XdataNotAllIncreasing(usize)
}

impl Display for TbfunxErr {
    fn fmt (&self, f: & mut Formatter) -> std::fmt::Result {
        match self {
            &TbfunxErr::DataLenMissmatch(ref a1_len, ref a2_len) =>
                write!(f, "{} and {}", a1_len, a2_len),
            &TbfunxErr::NoData => 
                write!(f, "No Data"),
            &TbfunxErr::XdataNotAllIncreasing(ref index) =>
                write!(f, "X data not all increasing near index {}", index)
        }
    }
}


// #############################################################
#[derive(PartialEq)]
pub enum BeyondExtentBehaviour {
    /// Use the value of y_array[0] if x < x[0] or
    /// use y_array[last] if x > x[last].
    /// Slope is determined by 2nd order curve fit.
    Ylimit,
    /// Use the slope to determine y value by linear extrapolation.
    /// Slope is that two elements closed to the array end. 0,1 or last-1,last.
    Linear,
    /// Use a 2nd order curve fit to determine Y and slope.
    CurveFit
}

fn do_quad(x: f32, x_idx0: usize, x_array: &Vec<f32>, y_array: &Vec<f32>) -> (f32, f32) {
    assert!(x_idx0 + 2 < x_array.len());
    assert!(x_array.len() == y_array.len());
    let mut x_quad: Vec<f32> = Vec::new();
    let mut y_quad: Vec<f32> = Vec::new();

    for i in 0..3 {
        x_quad.push(x_array[i + x_idx0]);
        y_quad.push(y_array[i + x_idx0])
    }
    utility::quad::quad(x, &x_quad, &y_quad)
}


// ##############################################################

/// Linear interpolate or curve fit to find a Y value and first deritivative.
/// from array of X and Y values.
/// * low_ext - what to do if 'x' is less than 'x_array[0]'.
/// * hight_ext - what to do if 'x' is more than 'y_array[last]' 
/// * return (y, derivative)
pub fn tbfunx(x: f32, x_array: &Vec<f32>, y_array: &Vec<f32>,
             low_ext: &BeyondExtentBehaviour,
             high_ext: &BeyondExtentBehaviour) -> Result<(f32,f32), TbfunxErr> {
    // Make sure that the array lengths are OK
    // Arrays must have same length.
    if x_array.len() != y_array.len() {
        Err(TbfunxErr::DataLenMissmatch(x_array.len(), y_array.len()))
    }
    // Arrays can not be 0 length.
    else if x_array.len() == 0 {
        Err(TbfunxErr::NoData)
    }
    else if x_array.len() == 1 {
        // If there is only one data point then that is the y and slope=0.0.
        Ok((y_array[0], 0.0f32))
    }
    else if x_array.len() <= 2 {
        // We only have 2 elements. Use either limit or linear.
        let dydx: f32 = (y_array[1] - y_array[0])/(x_array[1] - x_array[0]);
        let y = if x < x_array[0] && low_ext == &BeyondExtentBehaviour::Ylimit {
            y_array[0]
        }
        else if x > x_array[1] && high_ext == &BeyondExtentBehaviour::Ylimit {
            y_array[1]
        }
        else {
            // Use linear.
            y_array[0] + dydx * (x - x_array[0])
        };
        Ok((y, dydx))
    } else {
        // Interpolate for y if x_array[0] <= x <= x_array[last]
        if (x > x_array[0]) && (x < x_array[x_array.len()-1]) {
            // Search for the x_array values that surround 'x'
            let mut idx: usize = 0;
            for i in 0..(x_array.len() - 1) {
                // Make sure that the X values are in increasing order.
                if x_array[i] >= x_array[i + 1] {
                    // we had a decrease in X order. return an error.
                    return Err(TbfunxErr::XdataNotAllIncreasing(i))
                }
                // See if we are bracketing x yet.
                if x >= x_array[i] {
                    idx = i;
                    break;
                }
            }

            // We use 3 X&Y numbers for quadradic estimateion of the  derivative. The middle number
            // is the one just less than x, unless it is index 0, then we
            // use index 1 for the middle number. 
            if idx != 0 { idx = idx - 1 }
 
            let (y, derivative) = do_quad(x, idx, x_array, y_array);
            Ok((y, derivative))
        }
        else {
            // our x is outside the x array so we need to extrapolate

             if x <= x_array[0] {
                // We are working at the start of the arrays.
                
                // Make sure that the first 3 x' values are in order.
                if (x_array[0] >= x_array[1]) || (x_array[1] >= x_array[2]) {
                    return Err(TbfunxErr::XdataNotAllIncreasing(1))
                }
                match low_ext {
                    &BeyondExtentBehaviour::Ylimit => {
                        let (_, derivative) = do_quad(x_array[0], 0, x_array, y_array );
                        Ok((y_array[0], derivative))
                    },
                    &BeyondExtentBehaviour::Linear => {
                        let slope = (y_array[1] - y_array[0]) / (x_array[1]  - x_array[0]);
                        Ok((y_array[0] + slope * (x - x_array[0]), slope))
                    },
                    &BeyondExtentBehaviour::CurveFit => {
                        Ok(do_quad(x, 0, x_array, y_array))
                    }
                }
            }
            else {
                // We are working at the end of the arrays.
                let last = x_array.len()-1;

                // Make sure that the last 3 x' values are in order.
                if (x_array[last-2] >= x_array[last-1]) || (x_array[last -1] >= x_array[last]) {
                    return Err(TbfunxErr::XdataNotAllIncreasing(last-1))
                }
 
                 match low_ext {
                    &BeyondExtentBehaviour::Ylimit => {
                        let (_, derivative) = do_quad(x_array[last], last-2, x_array, y_array );
                        Ok((y_array[last], derivative))
                    },
                    &BeyondExtentBehaviour::Linear => {
                        let slope = (y_array[last] - y_array[last-1]) / (x_array[last]  - x_array[last-1]);
                        Ok((y_array[last-1] + slope * (x - x_array[last-1]), slope))
                    },
                    &BeyondExtentBehaviour::CurveFit => {
                        Ok(do_quad(x, last-2, x_array, y_array))
                    }
                }
            }

        }
    }


    
}

// ############################################

#[cfg(test)]

mod tests {
    use utility::tbfunx;
    #[test]
    fn line_between_lower_2points() {
        let xa: Vec<f32> = vec![2.0f32, 3.0f32, 4.0f32, 5.0f32];
        let ya: Vec<f32> = vec![1.0f32, 2.0f32, 3.0f32, 4.0f32];
        let x = 2.5f32;

        match tbfunx::tbfunx(x, &xa, &ya, &tbfunx::BeyondExtentBehaviour::Ylimit,
                        &tbfunx::BeyondExtentBehaviour::Ylimit) {
            Ok((y, slope)) => {
                assert!(y == 1.5f32);
                assert!(slope == 1.0f32);
            }
            Err(_) => { panic!("Should not have encounterd and error."); }
       }

    
    }

    #[test]
    fn line_between_upper_2points() {
        let xa: Vec<f32> = vec![2.0f32, 3.0f32, 4.0f32, 5.0f32];
        let ya: Vec<f32> = vec![1.0f32, 2.0f32, 3.0f32, 4.0f32];
        let x = 4.5f32;

        match tbfunx::tbfunx(x, &xa, &ya, &tbfunx::BeyondExtentBehaviour::Ylimit,
                        &tbfunx::BeyondExtentBehaviour::Ylimit) {
            Ok((y, slope)) => {
                assert!(y == 3.5f32);
                assert!(slope == 1.0f32);
            }
            Err(_) => { panic!("Should not have encounterd and error."); }
       }
    }

    #[test]
    fn bad_x_order() {
        // The last and next to last X are out of order.
        let xa: Vec<f32> = vec![2.0f32, 3.0f32, 5.0f32, 4.0f32];
        let ya: Vec<f32> = vec![1.0f32, 2.0f32, 3.0f32, 4.0f32];
        let x = 4.5f32;

        match tbfunx::tbfunx(x, &xa, &ya, &tbfunx::BeyondExtentBehaviour::Ylimit,
                        &tbfunx::BeyondExtentBehaviour::Ylimit) {
            Ok((_, _)) => {
                panic!("Should have had an error about X not in order.")
                //assert!(y == 3.5f32);
                //assert!(slope == 1.0f32);
            },
            Err(e) => { assert!(e == tbfunx::TbfunxErr::XdataNotAllIncreasing(2),
                        format!("Did not get expected error. Got=>{}", e)) }
       }

    
    }

}
