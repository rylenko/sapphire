# Sapphire

Modern **secure** and **private** messenger with end-to-end encryption.

**Secure** means protection from danger. **Private** means protection from observation.

# Implementation details

Built on [Double Ratchet Algorithm](https://en.wikipedia.org/wiki/Double_Ratchet_Algorithm).

The following cryptographic algorithms were selected: [X25519](https://en.wikipedia.org/wiki/Curve25519) for key exchange, [XChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) for encryption, [HKDF-SHA256](https://en.wikipedia.org/wiki/HKDF) for keys derivation and [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC-SHA256) for authentication.

### Why not XChaCha20-Poly1305?

[Is encrypt + HMAC stronger than AEAD?](https://crypto.stackexchange.com/a/100852)

# Lines of code

<html lang="en"><head><meta charset="utf-8" /><title>scc html output</title><style>table { border-collapse: collapse; }td, th { border: 1px solid #999; padding: 0.5rem; text-align: left;}</style></head><body><table id="scc-table">
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
		<th>JSON</th>
		<th>182</th>
		<th>182</th>
		<th>0</th>
		<th>0</th>
		<th>182</th>
		<th>0</th>
		<th>94934</th>
	</tr><tr>
		<th>D</th>
		<th>169</th>
		<th>2720</th>
		<th>423</th>
		<th>0</th>
		<th>2297</th>
		<th>0</th>
		<th>648540</th>
	</tr><tr>
		<th>Rust</th>
		<th>27</th>
		<th>25941</th>
		<th>3547</th>
		<th>654</th>
		<th>21740</th>
		<th>0</th>
		<th>938624</th>
	</tr><tr>
		<th>LLVM IR</th>
		<th>10</th>
		<th>438</th>
		<th>72</th>
		<th>58</th>
		<th>308</th>
		<th>0</th>
		<th>17331</th>
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
		<th>20</th>
		<th>7</th>
		<th>0</th>
		<th>13</th>
		<th>0</th>
		<th>301</th>
	</tr><tr>
		<th>Shell</th>
		<th>1</th>
		<th>54</th>
		<th>8</th>
		<th>3</th>
		<th>43</th>
		<th>0</th>
		<th>938</th>
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
		<th>401</th>
		<th>30195</th>
		<th>4219</th>
		<th>716</th>
		<th>25260</th>
		<th>0</th>
    	<th>1739337</th>
	</tr></tfoot>
	</table></body></html>

# Todo
|Path|Line|Description|
|-|-|-|
|crates/dr/Cargo.toml|1|   documentation with valid links to library elements.|
