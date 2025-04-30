use crate::modular_inverse;
use crate::gcd_utils;
use rand::Rng;


pub fn generate_keys(p:i32, q:i32) -> Result<(i32,i32,i32),String>{
    //calculate n
    let n = p * q;

    //calculate Euler's totient
    let euler_totient = (p - 1)*(q - 1);

    let mut rng = rand::thread_rng();

    //public key (n,e)
    
    //generate a suitable value e
    let e;
    loop {
        let candidate_e = rng.gen_range(2..euler_totient);
        if gcd_utils::standard_gcd(candidate_e, euler_totient) == 1 {
            if candidate_e < euler_totient{
                e = candidate_e;
                break;
            }
        }
    }

    //private key is (n,d)
    let d = match modular_inverse::modular_inverse(e,euler_totient){
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    Ok((n,e,d))
     
}

