import gcd_utils

def modular_inverse(a: int, b: int) -> int:
    #We don't need y from extended_gcd as looking for ax = 1 (mod b)
    gcd_inverse, x, _ = gcd_utils.extended_gcd(a, b, False)
    if gcd_inverse != 1:
        raise ValueError(f"No modular inverse for {a} mod {b}")
    return x % b
