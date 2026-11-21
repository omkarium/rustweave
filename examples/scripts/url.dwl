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
