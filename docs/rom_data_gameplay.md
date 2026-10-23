# European static gameplay records

This is a bounded static inventory. Current EUR e22-d21does not yet extract or
consume gameplay tables. New model/original execution/managed/native/lifecycle/
ordinary-scene checks remain pending; new Linux/USA verification stays pending.
Live units/inventory/task state are separate from these original ROM records.

## Item record source

Following the original shop list071030through category classifier0cf794roots
property reader080ce1ec. Its checked literal080ce20ccontains086e0bd0and
original instructions compute base + (u16item_id <<5). Independent list builder
080cf820uses the same source via literal080cf8dc; it enumerates1..361or
362..375for the separate mission category. Original inventory helper080cec58
independently uses boundary IDs361/375and mission count14, but returns live
EWRAM inventory02001940, not static item records.

The next independently rooted table is086e3ad0. Reader08072134loads the item
base through08072150, fetches item byte1dand uses it as a20byte index into that
next table through08072154. Together the independent roots and enumerated IDs
bound376records ×32bytes =12032bytes at ROM6e0bd0..6e3ad0(exclusive), including
record0without inventing a meaning for it. This span overlaps no e22resource.
A raw pointer-word search finds70matches to the item root; these are discovery
candidates, not70verified code readers.

## Original field access contract

Property reader080ce1ectruncates item ID to u16and property ID to u8. It has
no local item-ID bounds check. Properties0..18dispatch through a code jump
table at080ce214; other property values return0. That table remains code.

Properties0/1/2read unsigned halfwords at offsets00/02/04;3..8read unsigned
bytes08..0d;9reads halfword0e;10..17read signed bytes10..17;18reads unsigned
byte1d. Every32byte record must be preserved, including fields not exposed by
this getter. A lossless candidate representation separates four halfwords00..06,
eight bytes08..0f, eight signed bytes10..17and eight bytes18..1f. Semantic names
for most fields remain unproved; do not infer item names, prices, abilities or
job rules solely from plausible numeric values.

Properties10..13are dynamic. Original getter080ccf00reads live EWRAM bytes at
02002030+u16index. With byte13nonzero, the property reader returns the raw
signed field. With that byte zero, item IDs0033/012e/0094/011eadd live bytes
1/3/2/4to properties10/11/12/13respectively, then apply the original unsigned
comparison/clamp to255. Negative raw values and that unsigned clamp must not
be normalized into a new rule. Live modifiers remain guest code/state, never
frozen into extracted ROM records.

## Evidence and next criterion

Private read-only evidence stays under ignored target/rom-data-gameplay-items:
shop-inventory-071030.txt, callee-080cf794.txt, callee-080ce1ec.txt,
callee-080cec3c.txt, callee-080cec58.txt, item-reader-*.txt,
item-live-modifier-reader.txt, inventory-items.py, item-inventory.json and
item-records-independent.json. Independent static checks verify the exact EUR
hash, selected LDR literals/stride instructions/code targets, both enumeration
bounds, nonoverlap and complete12032byte typed-field roundtrip.

No original execution, extractor/runtime change, gameplay test, build or
performance/space measurement is claimed by this inventory. The next criterion
is a bounded typed catalog/source oracle for all376records and19properties,
both live-modifier branches/special items, invalid property/u16-u8truncation,
whole state/clock and source/strict JSON guards. Subsequent managed/direct
consumption must retain all prior files/generations and prove actual use.
The following20byte table, jobs/abilities/laws/AI, other item readers and regional
catalogs remain separate work.
