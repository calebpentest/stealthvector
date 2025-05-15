import base64

def xor_decrypt(data: bytes, key=0xAA) -> str:
    return ''.join(chr(b ^ key) for b in data)

b64 = "5unFxN7Yxcag6aDm6cXE3tjFxqD8oOjLycHZ2svJz6Doy8nB2drLyc+g+aD+oO+g66DmoP6g4qD8oO+g6aD+oOWg+KDmz8zeoOjLycHZ2svJz6DvxN7P2KA="

missing_padding = len(b64) % 4
if missing_padding:
    b64 += '=' * (4 - missing_padding)

cipher_bytes = base64.b64decode(b64)
plain_text = xor_decrypt(cipher_bytes)
print(plain_text)
