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
		<th>32</th>
		<th>2049</th>
		<th>227</th>
		<th>266</th>
		<th>1556</th>
		<th>53</th>
		<th>50035</th>
	</tr><tr>
		<th>TOML</th>
		<th>4</th>
		<th>84</th>
		<th>16</th>
		<th>4</th>
		<th>64</th>
		<th>0</th>
		<th>1466</th>
	</tr><tr>
		<th>Shell</th>
		<th>1</th>
		<th>52</th>
		<th>8</th>
		<th>5</th>
		<th>39</th>
		<th>0</th>
		<th>1008</th>
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
		<th>Makefile</th>
		<th>1</th>
		<th>27</th>
		<th>9</th>
		<th>0</th>
		<th>18</th>
		<th>0</th>
		<th>493</th>
	</tr><tr>
		<th>AWK</th>
		<th>1</th>
		<th>13</th>
		<th>1</th>
		<th>1</th>
		<th>11</th>
		<th>0</th>
		<th>213</th>
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
		<th>45</th>
		<th>2310</th>
		<th>286</th>
		<th>276</th>
		<th>1748</th>
		<th>53</th>
    	<th>55741</th>
	</tr></tfoot>
	</table>

# Todo

|Path|Line|Description|
|-|-|-|
|**crates/dr/src/clue.rs**|**5**|**Currently 64 bytes. Need to find out whether it should be a [`Copy`].**|
|**crates/dr/Cargo.toml**|**1**|**More benches**|
|**crates/dr/Cargo.toml**|**2**|**https://signal.org/docs/specifications/doubleratchet/#security-considerations**|
|**crates/dr/Cargo.toml**|**3**|**Examples**|
|**crates/dr/Cargo.toml**|**4**|**Create documentation with valid links to library elements.**|
