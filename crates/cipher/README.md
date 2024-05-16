# Cipher

Functions for authenticated encryption and decryption.

The following cryptographic algorithms were selected: [XChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) for encryption, [HKDF-SHA256](https://en.wikipedia.org/wiki/HKDF) for keys derivation and [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC-SHA256) for authentication.

Help I followed:
- [Is encrypt + HMAC stronger than AEAD?](https://crypto.stackexchange.com/a/100852)
- [Should we MAC-then-encrypt or encrypt-then-MAC?](https://crypto.stackexchange.com/questions/202/should-we-mac-then-encrypt-or-encrypt-then-mac)
