# Sapphire

Modern **secure** and **private** messenger with end-to-end encryption.

**Secure** means protection from danger. **Private** means protection from observation.

# Implementation details

Built on [Double Ratchet Algorithm](https://en.wikipedia.org/wiki/Double_Ratchet_Algorithm).

The following cryptographic algorithms were selected: [X25519](https://en.wikipedia.org/wiki/Curve25519) for key exchange, [XChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) for encryption, [HKDF-SHA256](https://en.wikipedia.org/wiki/HKDF) for keys derivation and [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC-SHA256) for authentication.

### Why not XChaCha20-Poly1305?

[Is encrypt + HMAC stronger than AEAD?](https://crypto.stackexchange.com/a/100852)

# Todo

|Path|Line|Description|
|-|-|-|
|**crates/dr/src/lib.rs**|**4**|**the library. Make it less bloated, make the code**|
|**crates/dr/src/clue.rs**|**5**|**Currently 64 bytes. Need to find out whether it should be a**|
|**crates/dr/Cargo.toml**|**1**|**More**|
|**crates/dr/Cargo.toml**|**2**|****|
|**crates/dr/Cargo.toml**|**3**|****|
|**crates/dr/Cargo.toml**|**4**|**Create documentation with valid links to library**|
