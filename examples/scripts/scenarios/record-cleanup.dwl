%dw 2.0
import * from dw::extra::Common
output application/json
---
do {
  var clean = payload.records map (record) -> do {
    var split = rsplitOnce(record.path, "/")
    var folder = if (split == null) "root" else split[0]
    var file = unwrapOr(stripPrefix(record.path, folder ++ "/"), record.path)
    var lowerFile = lower(file)
    var dot = splitOnce(lowerFile, ".")
    var extension = unwrapOr(andThen(dot, (parts) -> parts[1]), "")
    var stem = if (dot == null) lowerFile else unwrapOr(stripSuffix(lowerFile, "." ++ extension), lowerFile)
    var size = orElse(parseFloat(lstrip(record.size)), 0)
    ---
    {
      folder: folder,
      name: title(stem replace "_" with " "),
      extension: extension,
      size: toFixed(clamp(size, 0, 100), 2),
      owner: mapOr(record.owner, "unassigned", (name) -> title(trim(name))),
      ownerProvided: hasOwn(record, "owner"),
      tags: dedup(record.tags)
    }
  }
  ---
  {
    files: clean map (record) -> pick(record, ["folder", "name", "extension", "size", "owner"]),
    folders: fromEntries(
      unique(clean map $.folder) map (folder) -> {
        key: folder,
        value: fold(clean filter ($.folder == folder), 0, (record, sum) -> sum + 1)
      }
    ),
    ownerCount: sizeOf(compact(clean map (record) ->
      if (record.owner == "unassigned") null else record.owner
    )),
    extensions: unique(compact(clean map (record) ->
      if (record.extension == "") null else record.extension
    )),
    archiveView: clean
      filter ($.folder == "archive")
      map (record) -> omit(record, ["folder", "ownerProvided"])
  }
}
