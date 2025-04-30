import rsa_key_generation as rsa
import decrypt 
import encrypt 

def main():
	#input p and q.
	p = int(input("Enter the first prime: "))
	q = int(input("Enter the second prime: "))

	#get public and private keys
	n,e,d = rsa.generate_keys(p,q)

	print(f"The public key is: ({n},{e})")
	print(f"The private key is: ({n},{d})")

	#test RSA with an example
	m = input("Please input the test letter to encrypt: ")
	if ord(m) < n: 
    		ciphertext = encrypt.rsa_encrypt(ord(m),e,n)
    		print("Here is the encrypted letter:", ciphertext)
    		plaintext = chr(decrypt.rsa_decrypt(ciphertext,d,n))
    		print("Here is the decryption check: ", plaintext)


if __name__ == "__main__":
    main()