import modular_inverse as mod
import random
import gcd_utils

def generate_keys(p:int, q:int) -> tuple[int,int,int]:
    #calculate n
    n = p * q

    #calculate Euler's totient
    euler_totient = (p - 1)*(q - 1)

    #public key (n,e)

    #generate a suitable value e
    while True:
        e = random.randint(2,euler_totient - 1)
        if gcd_utils.standard_gcd(e,euler_totient)==1:
            break

    #private key is (n,d)
    d = mod.modular_inverse(e,euler_totient)

    return n,e,d

