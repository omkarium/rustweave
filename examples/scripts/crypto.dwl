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
