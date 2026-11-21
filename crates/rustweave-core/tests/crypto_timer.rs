// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the crypto, timer, and extension crypto helpers.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(script: &str) -> Result<String, rustweave_core::DwError> {
    rustweave_core::run(script, &[])
}

fn script(body: &str, imports: &str) -> String {
    format!("%dw 2.0\n{imports}\noutput application/json\n---\n{body}")
}

#[test]
fn crypto_digest_helpers() {
    let imports = "import * from dw::Crypto";
    assert_eq!(
        run(&script("MD5(\"hello\")", imports)),
        "\"5d41402abc4b2a76b9719d911017c592\""
    );
    assert_eq!(
        run(&script("SHA1(\"\")", imports)),
        "\"da39a3ee5e6b4b0d3255bfef95601890afd80709\""
    );
    assert_eq!(
        run(&script(
            "toHex(hashWith(\"hello\", \"SHA-256\"))",
            &format!("{imports}\nimport * from dw::core::Binaries")
        )),
        "\"2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824\""
    );
    assert_eq!(
        run(&script(
            "toHex(hashWith(\"hello\", \"SHA-512\"))",
            &format!("{imports}\nimport * from dw::core::Binaries")
        )),
        "\"9B71D224BD62F3785D96D46AD3EA3D73319BFBC2890CAADAE2DFF72519673CA72323C3D99BA5C11D7C7ACC6E14B8C5DA0C4663475C2E5C3ADEF46F73BCDEC043\""
    );
}

#[test]
fn crypto_hmac_helpers() {
    let imports = "import * from dw::Crypto";
    assert_eq!(
        run(&script(
            "HMACWith(\"key\", \"hello\", \"HmacSHA256\")",
            imports
        )),
        "\"9307b3b915efb5171ff14d8cb55fbcc798c6c0ef1456d66ded1a6aa723a58b7b\""
    );
    assert_eq!(
        run(&script(
            "toHex(HMACBinary(\"key\", \"hello\", \"HmacSHA512\"))",
            &format!("{imports}\nimport * from dw::core::Binaries")
        )),
        "\"FF06AB36757777815C008D32C8E14A705B4E7BF310351A06A23B612DC4C7433E7757D20525A5593B71020EA2EE162D2311B247E9855862B270122419652C0C92\""
    );
    assert_eq!(
        run(&script("HMACWith(1, 2, \"HmacSHA256\")", imports)),
        "\"6c34e84f57d4a9940c962b152acfbd28fb6f78f2090ca1dce041c36a07cd57ee\""
    );
}

#[test]
fn crypto_rejects_unknown_algorithms_and_bad_content() {
    let imports = "import * from dw::Crypto";
    assert!(run_result(&script("hashWith(\"hello\", \"NOPE\")", imports)).is_err());
    assert!(run_result(&script(
        "HMACWith(\"k\", \"hello\", \"HmacSHA256\")",
        imports
    ))
    .is_ok());
    assert!(run_result(&script("MD5(true)", imports)).is_err());
    assert!(run_result(&script("SHA1(null)", imports)).is_err());
}

#[test]
fn timer_to_milliseconds_and_duration() {
    let imports = "import * from dw::util::Timer";
    assert_eq!(
        run(&script("toMilliseconds(|2020-01-01T00:00:00Z|)", imports)),
        "1577836800000"
    );
    assert_eq!(
        run(&script("toMilliseconds(|1999-12-31T23:59:59Z|)", imports)),
        "946684799000"
    );
    assert_eq!(
        run(&script("keysOf(duration(() -> 2 + 2))", imports)),
        "[\n  \"time\",\n  \"result\"\n]"
    );
    assert_eq!(run(&script("duration(() -> 2 + 2).result", imports)), "4");
    assert_eq!(
        run(&script("keysOf(time(() -> \"x\"))", imports)),
        "[\n  \"start\",\n  \"result\",\n  \"end\"\n]"
    );
    assert_eq!(run(&script("currentMilliseconds() > 0", imports)), "true");
    assert!(run_result(&script("duration({})", imports)).is_err());
    assert!(run_result(&script("toMilliseconds(\"2020-01-01\")", imports)).is_err());
}

#[test]
fn aes_gcm_matches_known_vectors() {
    let imports = "import * from dw::extra::Common\nimport * from dw::core::Binaries";
    let key = "fromHex(\"0000000000000000000000000000000000000000000000000000000000000000\")";
    let iv = "fromHex(\"000000000000000000000000\")";
    assert_eq!(
        run(&script(
            &format!("toHex(aesGcmEncrypt(\"\", {key}, {iv}))"),
            imports
        )),
        "\"530F8AFBC74536B9A963B4F1C4CB738B\""
    );
    assert_eq!(
        run(&script(
            &format!(
                "toHex(aesGcmEncrypt(fromHex(\"00000000000000000000000000000000\"), {key}, {iv}))"
            ),
            imports
        )),
        "\"CEA7403D4D606B6E074EC5D3BAF39D18D0D1C8A799996BF0265B98B5D48AB919\""
    );
    assert_eq!(
        run(&script(
            &format!("aesGcmDecrypt(aesGcmEncrypt(\"hello\", {key}, {iv}), {key}, {iv}) as String"),
            imports
        )),
        "\"hello\""
    );
}

#[test]
fn aes_gcm_validates_keys_ivs_and_tags() {
    let imports = "import * from dw::extra::Common\nimport * from dw::core::Binaries";
    let key = "fromHex(\"0000000000000000000000000000000000000000000000000000000000000000\")";
    assert!(run_result(&script(
        "aesGcmEncrypt(\"x\", fromHex(\"0000000000000000000000000000000000000000\"), fromHex(\"000000000000000000000000\"))",
        imports
    ))
    .is_err());
    assert!(run_result(&script(
        &format!("aesGcmEncrypt(\"x\", {key}, fromHex(\"00000000000000000000000000000000\"))"),
        imports
    ))
    .is_err());
    assert!(run_result(&script(
        "aesGcmDecrypt(fromHex(\"00000000000000000000000000000000\"), fromHex(\"0000000000000000000000000000000000000000000000000000000000000000\"), fromHex(\"000000000000000000000000\"))",
        imports
    ))
    .is_err());
}

#[test]
fn extension_derivation_and_comparison_vectors() {
    let imports = "import * from dw::extra::Common\nimport * from dw::core::Binaries";
    assert_eq!(
        run(&script(
            "toHex(hkdfSha256(fromHex(\"0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b\"), fromHex(\"000102030405060708090a0b0c\"), fromHex(\"f0f1f2f3f4f5f6f7f8f9\"), 42))",
            imports
        )),
        "\"3CB25F25FAACD57A90434F64D0362F2A2D2D0A90CF1A5A4C5DB02D56ECC4C5BF34007208D5B887185865\""
    );
    assert_eq!(
        run(&script(
            "toHex(pbkdf2Sha256(\"password\", \"salt\", 1, 32))",
            imports
        )),
        "\"120FB6CFFCF8B32C43E7225256C4F837A86548C92CCC35480805987CB70BE17B\""
    );
    assert_eq!(
        run(&script("constantTimeEquals(\"a\", \"a\")", imports)),
        "true"
    );
    assert_eq!(
        run(&script("constantTimeEquals(\"a\", \"b\")", imports)),
        "false"
    );
    assert_eq!(run(&script("sizeOf(randomBytes(16))", imports)), "16");
    assert!(run_result(&script("randomBytes(-1)", imports)).is_err());
}
