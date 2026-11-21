# Extension crypto helpers in one script

This guide shows the crypto helpers in `dw::extra::Common`. The AES, HKDF, and PBKDF2 examples use
published test vectors.

The script uses all 6 crypto functions listed for `dw::extra::Common`.

## Script

```dataweave
%dw 2.0
import * from dw::extra::Common
import * from dw::core::Binaries
output application/json
var key = fromHex("0000000000000000000000000000000000000000000000000000000000000000")
var iv = fromHex("000000000000000000000000")
---
{
  aesEmpty: toHex(aesGcmEncrypt("", key, iv)),
  aesHello: toHex(aesGcmEncrypt("hello", key, iv)),
  aesRoundTrip: aesGcmDecrypt(aesGcmEncrypt("hello", key, iv), key, iv) as String,
  randomLength: sizeOf(randomBytes(24)),
  hkdf: toHex(hkdfSha256(fromHex("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b"), fromHex("000102030405060708090a0b0c"), fromHex("f0f1f2f3f4f5f6f7f8f9"), 42)),
  pbkdf2: toHex(pbkdf2Sha256("password", "salt", 1, 32)),
  same: constantTimeEquals("abc", "abc"),
  different: constantTimeEquals("abc", "abd")
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/extension-crypto.dwl
```

## Expected output

```json
{
  "aesEmpty": "530F8AFBC74536B9A963B4F1C4CB738B",
  "aesHello": "A6C22C51228B908F7F62FFCEA6A92FABEF39BF4D93",
  "aesRoundTrip": "hello",
  "randomLength": 24,
  "hkdf": "3CB25F25FAACD57A90434F64D0362F2A2D2D0A90CF1A5A4C5DB02D56ECC4C5BF34007208D5B887185865",
  "pbkdf2": "120FB6CFFCF8B32C43E7225256C4F837A86548C92CCC35480805987CB70BE17B",
  "same": true,
  "different": false
}
```

## Functions shown

`aesGcmEncrypt`, `aesGcmDecrypt`, `randomBytes`, `hkdfSha256`, `pbkdf2Sha256`,
`constantTimeEquals`
