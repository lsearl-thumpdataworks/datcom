/// -----
/// File: /src/utility/quad.rs  
/// Project: datcon  
/// File Created: Wednesday, 14th Nov 182018 10:58:34 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Wednesday, 14th November 2018 10:58:36 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----

/// Interpolate or detemine derivative using 3 x and y points on a line and an arbitray
/// x position.


/// Calculate the quaradic coeffient calculation.
fn quad_coeff (a11:f32, a12:f32, a13:f32, a21:f32, a22:f32, a23:f32, a31:f32, a32:f32, a33:f32) 
            -> f32 {
    a11 * (a22 * a33 - a23 * a32) - a21 * (a12 * a33 - a13 * a32) + a31 * (a12 * a23 - a13 * a22)
}

/// Peform either quadratic interpolation or quadratic derivative at X. 
/// * x - position for interpolcation or derivative
/// * x_array - the 3 elements of x
/// * y_array - the 3 elements of y
/// * result - interpolated y value at x, derivative at x
pub fn quad(x: f32, x_array: &Vec<f32>, y_array: &Vec<f32>) -> (f32, f32) {
    assert!(x_array.len() == 3);
    assert!(y_array.len() == 3);
    // Ensure the x array is sorted.
    assert!(x_array[0] < x_array[1]);
    assert!(x_array[1] < x_array[2]);
    // Get the squar of the x values.
    let mut x_sqr: Vec<f32> = Vec::new();
    for i in 0..3 {
        x_sqr.push(x_array[i] * x_array[i])
    }
    let denominator = quad_coeff(x_sqr[0], x_array[0], 1.0f32,
                                x_sqr[1], x_array[1], 1.0f32,
                                x_sqr[2], x_array[2], 1.0f32);
    let a_coeff = quad_coeff(y_array[0], x_array[0], 1.0f32,
                            y_array[1], x_array[1], 1.0f32,
                            y_array[2], x_array[2], 1.0f32) / denominator; 
    let b_coeff = quad_coeff(x_sqr[0], y_array[0], 1.0f32,
                            x_sqr[1], y_array[1], 1.0f32,
                            x_sqr[2], y_array[2], 1.0f32) / denominator; 
    let c_coeff = quad_coeff(x_sqr[0], x_array[0], y_array[0],
                            x_sqr[1], x_array[1], y_array[1],
                            x_sqr[2], x_array[2], y_array[2]) / denominator; 
    println!("quad: d={}", denominator);
    println!("quad: a={}", a_coeff);
    println!("quad: b={}", b_coeff);
    println!("quad: c={}", c_coeff);
    (x * (a_coeff * x + b_coeff) + c_coeff,
        2.0f32 * a_coeff * x + b_coeff)
    
}

// ###########################################################################################

#[cfg(test)]

mod tests {
    use utility;
    #[test]
    fn line_between_lower_2points() {
        let xa: Vec<f32> = vec![2.0f32, 3.0f32, 4.0f32];
        let ya: Vec<f32> = vec![1.0f32, 2.0f32, 3.0f32];
        let x = 2.5f32;

        let (y, slope) = utility::quad::quad(x, &xa, &ya);
        //println!("y={}", y);
        //println!("slope={}", slope);
        assert!(y == 1.5f32);
        assert!(slope == 1.0f32);
    }
    #[test]
    fn line_between_upper_2points() {
        let xa: Vec<f32> = vec![2.0f32, 3.0f32, 4.0f32];
        let ya: Vec<f32> = vec![1.0f32, 2.0f32, 3.0f32];
        let x = 3.5f32;

        let (y, slope) = utility::quad::quad(x, &xa, &ya);
        //println!("y={}", y);
        //println!("slope={}", slope);
        assert!(y == 2.5f32);
        assert!(slope == 1.0f32);
    }
    #[test]
    fn parab_above_upper_last_point() {
        let xa: Vec<f32> = vec![0.0f32, 1.0f32, 2.0f32];
        let ya: Vec<f32> = vec![3.0f32, 1.0f32, 3.0f32];
        let x = 4.0f32;

        let (y, slope) = utility::quad::quad(x, &xa, &ya);
        //println!("y={}", y);
        //println!("slope={}", slope);
        assert!(y == 19.0f32);
        assert!(slope == 12.0f32);
    }

}
