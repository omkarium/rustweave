# URL functions in one script

URI components are encoded and decoded, path segments are composed into a URL, and a full URL is parsed into its parts.

This script uses all 6 functions listed for `dw::core::URL`.

Import `dw::core::URL` to use the module functions.

## Script

```dataweave
%dw 2.0
import * from dw::core::URL
output application/json
---
{
  encodeURIComponent: encodeURIComponent("a b/c?d=e&f"),
  encodeURI: encodeURI("a b/c?d=e&f#g"),
  decodeURIComponent: decodeURIComponent("%20PATH/%20TO%20/DECODE%20"),
  decodeURI: decodeURI("%20PATH/%20TO%20/DECODE%20"),
  compose: compose(["https://docs.test/", "/guide.html"], ["field notes"]),
  parseURI: parseURI("https://docs.test/guide#intro")
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/url.dwl
```

## Expected output

```json
{
  "encodeURIComponent": "a%20b%2Fc%3Fd%3De%26f",
  "encodeURI": "a%20b/c?d=e&f#g",
  "decodeURIComponent": " PATH/ TO /DECODE ",
  "decodeURI": " PATH/ TO /DECODE ",
  "compose": "https://docs.test/field%20notes/guide.html",
  "parseURI": {
    "isValid": true,
    "raw": "https://docs.test/guide#intro",
    "host": "docs.test",
    "authority": "docs.test",
    "fragment": "intro",
    "path": "/guide",
    "scheme": "https",
    "isAbsolute": true,
    "isOpaque": false
  }
}
```

## Functions shown

`compose`, `decodeURI`, `decodeURIComponent`, `encodeURI`, `encodeURIComponent`, `parseURI`
