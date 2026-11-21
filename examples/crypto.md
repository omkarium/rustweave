# Crypto functions in one script

This guide shows every helper exposed by `dw::Crypto`.

The script uses all 5 functions listed for `dw::Crypto`.

## Script

```dataweave
%dw 2.0
import * from dw::Crypto
import * from dw::core::Binaries
output application/json
---
{
  md5Hello: MD5("hello"),
  sha1Hello: SHA1("hello"),
  sha256Hex: toHex(hashWith("hello", "SHA-256")),
  md5Hex: toHex(hashWith("hello", "MD5")),
  hmacHex: HMACWith("key", "hello", "HmacSHA256"),
  hmacBinaryHex: toHex(HMACBinary("key", "hello", "HmacSHA1")),
  numberDigest: MD5(123)
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/crypto.dwl
```

## Expected output

```json
{
  "md5Hello": "5d41402abc4b2a76b9719d911017c592",
  "sha1Hello": "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d",
  "sha256Hex": "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824",
  "md5Hex": "5D41402ABC4B2A76B9719D911017C592",
  "hmacHex": "9307b3b915efb5171ff14d8cb55fbcc798c6c0ef1456d66ded1a6aa723a58b7b",
  "hmacBinaryHex": "B34CEAC4516FF23A143E61D79D0FA7A4FBE5F266",
  "numberDigest": "f95b70fdc3088560732a5ac135644506"
}
```

## Functions shown

`MD5`, `SHA1`, `hashWith`, `HMACWith`, `HMACBinary`
