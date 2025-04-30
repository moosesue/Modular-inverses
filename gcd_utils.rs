pub fn standard_gcd(a: i32, b: i32)->i32{
    if a == 0 {
     b
 }
  else {
     standard_gcd(b % a, a)
     }
}

pub fn extended_gcd(mut a: i32, mut b: i32, positive_x: bool) -> (i32, i32, i32) {
    let original_a = a;
    let original_b = b;
    let mut x0 = 0;
    let mut y0 = 1;
    let mut x1 = 1;
    let mut y1 = 0;

    while b != 0 {
        let quotient = a / b;
        let (new_a, new_b) = (b, a % b);
        a = new_a;
        b = new_b;

        let temp_x0 = x0;
        x0 = x1;
        x1 = temp_x0 - quotient * x1;

        let temp_y0 = y0;
        y0 = y1;
        y1 = temp_y0 - quotient * y1;
    }

    if positive_x && x0 < 0 {
        
        let shift_k = ((x0).abs() + (original_b / a) - 1) / (original_b / a);
        x0 += shift_k * (original_b / a);
        y0 -= shift_k * (original_a / a);
    }

    (a, x0, y0)
}
