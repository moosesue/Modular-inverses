use num_bigint::BigUint;


pub fn rsa_encrypt(m: u32, e: u32, n: u32)-> BigUint{
    //ciphertext = m**e mod n 
    let base = BigUint::from(m);
    let exponent = BigUint::from(e);
    let modulus = BigUint::from(n);

    base.modpow(&exponent, &modulus)
        
       
}