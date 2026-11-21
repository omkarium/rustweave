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
