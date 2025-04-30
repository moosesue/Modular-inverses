use crate::gcd_utils; 

pub fn modular_inverse(a:i32, b:i32) -> Result<i32, String> {
    
    //We don't need y from extended_gcd as looking for ax = 1 (mod b)
    let (gcd_inverse, x, _y) = gcd_utils::extended_gcd(a, b, false);
    if gcd_inverse != 1{
        return Err(format!("No modular inverse exists for {} mod {}", a, b));
    }
    //ensures positive return instead of just x % b
    //done automatically in Python but not in Rust
    Ok((x % b + b) % b) 
}