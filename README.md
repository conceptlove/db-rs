# ConceptDB

WIP

## Design

> Note: Still pulling my notes in.

### Fact

`Fact(Entity, Attr, Value, Txn)`

A `fact` is the basic building block of conceptDB.

### Aggregations

An aggregation reduces a set into a single value.

`count`, `sum`, `uniq`, etc.

### Id mappings within other programs

Use UUIDv5 to map external ids to uuids.
