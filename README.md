# Modular Inverses

Bézout’s identity, $a \cdot x + b \cdot y = \gcd(a, b)$, is the basis for finding modular inverses.
A modular inverse (or modular multiplicative inverse) is a number $x$ such that:
$a \cdot x\equiv 1\pmod{b}$.

A modular inverse only exists if $\gcd(a, b) = 1$.
When this happens, $a$ and $b$ are said to be coprime (they share no common factors except 1).
If $\gcd(a, b) = 1$, then the $x$ found from the Extended GCD algorithm is the modular inverse of $a$ modulo $b$.

### Example:

Consider finding the modular inverse of 28 modulo 29:
Since 29 is prime, $\gcd(28, 29) = 1$.
Using the Extended GCD algorithm, we find that $x = 1$.

Therefore:

$28 \equiv 1 \pmod{29}$. 
So, 1 is the modular inverse of 28 modulo 29.

In Python we can write this as:
```python
def modular_inverse(a: int, b: int) -> int:
    #We don't need y from extended_gcd as looking for ax = 1 (mod b)
    gcd_inverse, x, _ = gcd_utils.extended_gcd(a, b, False)
    if gcd_inverse != 1:
        raise ValueError(f"No modular inverse for {a} mod {b}")
    return x % b
```
And in Rust:
```rust
fn modular_inverse(a:i32, b:i32) -> Result<i32, String> {
    
    //We don't need y from extended_gcd as looking for ax = 1 (mod b)
    let (gcd_inverse, x, _y) = extended_gcd(a, b, false);
    if gcd_inverse != 1{
        return Err(format!("No modular inverse exists for {} mod {}", a, b));
    }
    //ensures positive return instead of just x % b
    //done automatically in Python but not in Rust
    Ok((x % b + b) % b) 
}
``` 
## Application in RSA (Rivest-Shamir-Adleman) cryptosystems.

The RSA algorithm uses modular inverses to generate key pairs using two prime numbers. 

### Example:

If $p = 13$ and $q = 29$, then $n = 13 \times 29 = 377$. 

Euler's totient function, $\varphi(n)$, counts how many numbers there are coprime to $n$. As we need an encryption key, $e$, and decryption key, $d$, that satisfy $e \cdot d \equiv 1\pmod{\varphi(n)}$, we need to calculate $d$ and to do this we need to find $\varphi(n)$.

$$
\varphi(n)= (p-1)(q-1) = 12 \times 28 = 336
$$

Choose an integer $e$ between 1 and 336 which must be coprime with 336. Let’s choose $e$ = 11, as 11 is prime. 

The public key is (377,11).

Calculate the inverse of $11 \pmod{336}$. The inverse is 275.

The private key is (377,275).

Allowing anyone to encrypt a message but keeping the decryption key secret is an example of asymmetric cryptography and is based on the fact that given only the encryption key and product of two large prime numbers, it is difficult to find the decryption key. The security of the algorithm depends on the size of the two primes chosen. 

### Example:

Say we want to encrypt the number 5 using our public key (377,11). 

The ciphertext, $c$, is found by using $5^{11} \pmod{377}$. We can split this into indices where the result is bigger than 377 and evaluate it at each point to make things easier (and the numbers smaller!).

$5^2 = 25$ (not bigger than 377 so $25 \pmod{377}$ is still 25)
$5^4 = 625 = 248 \pmod{377}$
$5^8 = 248^2 = 61504 \pmod{377} = 320$ 
$5^{11} = 5^{8} \times 5^{2} \times 5$ (remember indices rules where indexes are added when multiplying).

So $5^{11} \pmod{377} = (320 \times 248 \times 5) \pmod{377} = 38 \pmod{377}$.

So $c = 38$.

To decrypt and find the plaintext $m$, we would then use $38^{275} \pmod{377}$.  
(Recall that 275 is the modular inverse of 11 mod 336.)

We can implement a demo of using RSA to encrypt/decrypt a single character in Python:

```python
def generate_keys(p:int, q:int) -> tuple[int,int,int]:
    #calculate n
    n = p * q

    #calculate Euler's totient
    euler_totient = (p - 1)*(q - 1)

    #public key (n,e)

    #generate a suitable value e
    for test_e in range(2,euler_totient):
        gcd_euler = gcd_utils.standard_gcd(test_e, euler_totient)
        if gcd_euler == 1:
            e = test_e
            break

    #private key is (n,d)
    d = mod.modular_inverse(e,euler_totient)

    return n,e,d
def rsa_encrypt(m: int, e: int, n: int)-> int:
    #ciphertext = m**e mod n 
    return pow(m,e,n)

def rsa_decrypt(c: int, d: int,n: int)->int:
    #plaintext = c**d mod n
    return pow(c,d,n)
```
Or in Rust:
```rust
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

pub fn rsa_encrypt(m: u32, e: u32, n: u32)-> BigUint{
    //ciphertext = m**e mod n 
    let base = BigUint::from(m);
    let exponent = BigUint::from(e);
    let modulus = BigUint::from(n);

    base.modpow(&exponent, &modulus)
        
       
}

pub fn rsa_decrypt(c: BigUint, d: u32,n: u32)->BigUint{
    //plaintext = c**d mod n
    let exponent = BigUint::from(d);
    let modulus = BigUint::from(n);

    c.modpow(&exponent, &modulus)
    
}
```


