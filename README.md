# Sapphire

Modern **secure** and **private** messenger with end-to-end encryption.

**Secure** means protection from danger. **Private** means protection from observation.

# Implementation details

Built on [Double Ratchet Algorithm](https://en.wikipedia.org/wiki/Double_Ratchet_Algorithm).

The following cryptographic algorithms were selected: [X25519](https://en.wikipedia.org/wiki/Curve25519) for key exchange, [XChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) for encryption, [HKDF-SHA256](https://en.wikipedia.org/wiki/HKDF) for keys derivation and [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC-SHA256) for authentication.

### Why not XChaCha20-Poly1305?

[Is encrypt + HMAC stronger than AEAD?](https://crypto.stackexchange.com/a/100852)

# Lines of code

<table id="scc-table">
	<thead><tr>
		<th>Language</th>
		<th>Files</th>
		<th>Lines</th>
		<th>Blank</th>
		<th>Comment</th>
		<th>Code</th>
		<th>Complexity</th>
		<th>Bytes</th>
	</tr></thead>
	<tbody><tr>
		<th>Rust</th>
		<th>24</th>
		<th>2104</th>
		<th>231</th>
		<th>283</th>
		<th>1590</th>
		<th>56</th>
		<th>52467</th>
	</tr><tr>
		<th>TOML</th>
		<th>4</th>
		<th>81</th>
		<th>16</th>
		<th>1</th>
		<th>64</th>
		<th>0</th>
		<th>1342</th>
	</tr><tr>
		<th>YAML</th>
		<th>3</th>
		<th>50</th>
		<th>9</th>
		<th>0</th>
		<th>41</th>
		<th>0</th>
		<th>956</th>
	</tr><tr>
		<th>Markdown</th>
		<th>2</th>
		<th>33</th>
		<th>16</th>
		<th>0</th>
		<th>17</th>
		<th>0</th>
		<th>1552</th>
	</tr><tr>
		<th>License</th>
		<th>1</th>
		<th>674</th>
		<th>121</th>
		<th>0</th>
		<th>553</th>
		<th>0</th>
		<th>34801</th>
	</tr><tr>
		<th>Makefile</th>
		<th>1</th>
		<th>23</th>
		<th>8</th>
		<th>0</th>
		<th>15</th>
		<th>0</th>
		<th>371</th>
	</tr><tr>
		<th>Shell</th>
		<th>1</th>
		<th>55</th>
		<th>8</th>
		<th>3</th>
		<th>44</th>
		<th>0</th>
		<th>953</th>
	</tr><tr>
		<th>gitignore</th>
		<th>1</th>
		<th>2</th>
		<th>0</th>
		<th>0</th>
		<th>2</th>
		<th>0</th>
		<th>18</th>
	</tr></tbody>
	<tfoot><tr>
		<th>Total</th>
		<th>37</th>
		<th>3022</th>
		<th>409</th>
		<th>287</th>
		<th>2326</th>
		<th>56</th>
    	<th>92460</th>
	</tr></tfoot>
	</table>

# Todo
|Path|Line|Description|
|-|-|-|
|**crates/dr/Cargo.toml**|**1**|**documentation with valid links to library elements.**|
