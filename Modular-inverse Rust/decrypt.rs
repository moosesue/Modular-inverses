use num_bigint::BigUint;


pub fn rsa_decrypt(c: BigUint, d: u32,n: u32)->BigUint{
    //plaintext = c**d mod n
    let exponent = BigUint::from(d);
    let modulus = BigUint::from(n);

    c.modpow(&exponent, &modulus)
    
}