def RSA_encrypt(m: int, e: int, n: int)-> int:
    #ciphertext = m**e mod n 
    return pow(m,e,n)
