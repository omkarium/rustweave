# Combined scenarios

The three scenarios below use several modules at once. Run each command from the repository root.

## Order Summary

The script filters, totals, ranks, groups, and projects a set of orders. It combines extension helpers with Core collection and formatting functions.

```dataweave
%dw 2.0
import * from dw::extra::Common
output application/json
---
do {
  var paid = payload.orders filter ($.status == "paid")
  var priced = paid map (order) -> assign(order, {
    total: order.units * order.unitPrice,
    label: zfill(order.id, 7)
  })
  var ranked = sorted(priced, (order) -> order.total, true)
  var regions = unique(ranked map $.region)
  ---
  {
    regions: regions map (region) -> do {
      var lines = ranked filter ($.region == region)
      ---
      {
        name: title(region),
        orders: lines map (order) -> pick(order, ["label", "units", "total"]),
        units: fold(lines, 0, (order, sum) -> sum + order.units),
        revenue: toFixed(fold(lines, 0, (order, sum) -> sum + order.total), 2),
        notes: unique(compact(lines map $.note))
      }
    },
    topOrder: {
      label: ranked[0].label,
      total: toFixed(ranked[0].total, 2)
    },
    lead: toFixed(ranked[0].total - ranked[-1].total, 2),
    labels: ranked map $.label,
    paused: payload.orders
      filter ($.status != "paid")
      map (order) -> omit(order, ["note", "region"])
  }
}
```

```bash
./target/debug/rustweave -i payload=examples/data/orders.json -f examples/scripts/scenarios/order-summary.dwl
```

Expected output:

```json
{
  "regions": [
    {
      "name": "North",
      "orders": [
        {
          "units": 3,
          "total": 37.5,
          "label": "00A-104"
        },
        {
          "units": 2,
          "total": 19.5,
          "label": "00A-109"
        }
      ],
      "units": 5,
      "revenue": "57.00",
      "notes": [
        "express"
      ]
    },
    {
      "name": "East",
      "orders": [
        {
          "units": 5,
          "total": 21.25,
          "label": "00A-107"
        }
      ],
      "units": 5,
      "revenue": "21.25",
      "notes": [
        "gift"
      ]
    },
    {
      "name": "South",
      "orders": [
        {
          "units": 2,
          "total": 16,
          "label": "00A-105"
        }
      ],
      "units": 2,
      "revenue": "16.00",
      "notes": [
        
      ]
    }
  ],
  "topOrder": {
    "label": "00A-104",
    "total": "37.50"
  },
  "lead": "21.50",
  "labels": [
    "00A-104",
    "00A-107",
    "00A-109",
    "00A-105"
  ],
  "paused": [
    {
      "id": "A-106",
      "status": "open",
      "units": 1,
      "unitPrice": 30
    },
    {
      "id": "A-108",
      "status": "cancelled",
      "units": 4,
      "unitPrice": 6
    }
  ]
}
```

## Dispatch Board

The script sorts a ticket queue once, then uses that ordered view for binary lookup, rolling windows, batching, rotations, slot edits, and grouped totals.

```dataweave
%dw 2.0
import * from dw::extra::Common
output application/json
---
do {
  var ordered = sorted(payload.queue, (ticket) -> ticket.priority)
  var minutes = ordered map $.minutes
  var highPriority = binarySearch(ordered map $.priority, 4)
  ---
  {
    order: ordered map (ticket) -> {ticket: ticket.ticket, priority: ticket.priority},
    highPriorityStart: highPriority,
    rollingMinutes: windows(minutes, 3)
      map (group) -> toFixed(fold(group, 0, (n, sum) -> sum + n) / sizeOf(group), 1),
    sampled: stepBy(ordered, 2) map $.ticket,
    fullBatches: chunksExact(ordered, 3) map (batch) -> batch map $.ticket,
    firstBatch: truncate(ordered, 3) map $.ticket,
    rotatedLeft: rotateLeft(ordered, 1) map $.ticket,
    rotatedRight: rotateRight(ordered, 2) map $.ticket,
    swapped: swap(minutes, 0, 2),
    filled: fill(minutes, 0, 3, 5),
    nextHigh: position(ordered, (ticket) -> ticket.priority >= 4),
    lastLow: rposition(ordered, (ticket) -> ticket.priority <= 2),
    consecutiveZones: dedup(ordered map $.zone),
    zoneMinutes: fromEntries(
      unique(ordered map $.zone) map (zone) -> {
        key: zone,
        value: fold(ordered filter ($.zone == zone) map $.minutes, 0, (n, sum) -> sum + n)
      }
    )
  }
}
```

```bash
./target/debug/rustweave -i payload=examples/data/tickets.json -f examples/scripts/scenarios/dispatch-board.dwl
```

Expected output:

```json
{
  "order": [
    {
      "ticket": "Q-31",
      "priority": 1
    },
    {
      "ticket": "Q-32",
      "priority": 2
    },
    {
      "ticket": "Q-33",
      "priority": 2
    },
    {
      "ticket": "Q-34",
      "priority": 3
    },
    {
      "ticket": "Q-35",
      "priority": 4
    },
    {
      "ticket": "Q-36",
      "priority": 5
    }
  ],
  "highPriorityStart": {
    "found": true,
    "index": 4
  },
  "rollingMinutes": [
    "13.3",
    "11.0",
    "14.0",
    "10.3"
  ],
  "sampled": [
    "Q-31",
    "Q-33",
    "Q-35"
  ],
  "fullBatches": [
    [
      "Q-31",
      "Q-32",
      "Q-33"
    ],
    [
      "Q-34",
      "Q-35",
      "Q-36"
    ]
  ],
  "firstBatch": [
    "Q-31",
    "Q-32",
    "Q-33"
  ],
  "rotatedLeft": [
    "Q-32",
    "Q-33",
    "Q-34",
    "Q-35",
    "Q-36",
    "Q-31"
  ],
  "rotatedRight": [
    "Q-35",
    "Q-36",
    "Q-31",
    "Q-32",
    "Q-33",
    "Q-34"
  ],
  "swapped": [
    20,
    8,
    12,
    5,
    17,
    9
  ],
  "filled": [
    12,
    8,
    20,
    0,
    0,
    9
  ],
  "nextHigh": 4,
  "lastLow": 2,
  "consecutiveZones": [
    "north",
    "south",
    "east",
    "north",
    "west",
    "south"
  ],
  "zoneMinutes": {
    "north": 17,
    "south": 17,
    "east": 20,
    "west": 17
  }
}
```

## Record Cleanup

The script cleans paths and file names, parses sizes, chooses defaults for missing values, and builds folder and extension summaries.

```dataweave
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
```

```bash
./target/debug/rustweave -i payload=examples/data/records.json -f examples/scripts/scenarios/record-cleanup.dwl
```

Expected output:

```json
{
  "files": [
    {
      "folder": "incoming",
      "name": "Harbor Map",
      "extension": "pdf",
      "size": "12.50",
      "owner": "unassigned"
    },
    {
      "folder": "incoming",
      "name": "Tide Chart",
      "extension": "csv",
      "size": "3.00",
      "owner": "Ana"
    },
    {
      "folder": "archive",
      "name": "Notes",
      "extension": "txt",
      "size": "0.00",
      "owner": "unassigned"
    },
    {
      "folder": "incoming",
      "name": "Manifest",
      "extension": "json",
      "size": "0.75",
      "owner": "Ravi"
    },
    {
      "folder": "archive",
      "name": "Old Map",
      "extension": "pdf",
      "size": "8.00",
      "owner": "Ana"
    }
  ],
  "folders": {
    "incoming": 3,
    "archive": 2
  },
  "ownerCount": 3,
  "extensions": [
    "pdf",
    "csv",
    "txt",
    "json"
  ],
  "archiveView": [
    {
      "name": "Notes",
      "extension": "txt",
      "size": "0.00",
      "owner": "unassigned",
      "tags": [
        
      ]
    },
    {
      "name": "Old Map",
      "extension": "pdf",
      "size": "8.00",
      "owner": "Ana",
      "tags": [
        "map"
      ]
    }
  ]
}
```
