def RSA_decrypt(c: int, d: int,n: int)->int:
    #plaintext = c**d mod n
    return pow(c,d,n)
