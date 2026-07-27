# Priority Queue

This intermediate example shows a heap-backed priority queue implementation in Soroban.

## Features

- `push(item, priority)` — insert an item with a given priority
- `peek_max()` — inspect the highest-priority item without removing it
- `pop_max()` — remove and return the highest-priority item
- `len()` / `is_empty()` — query queue size
- `all()` — return all entries
- `initialize(admin)` — set the admin who controls advanced operations
- `bulk_push(items, priorities)` — atomically insert multiple items, heapifying once for efficiency
- `remove(item)` — remove a specific item from the queue regardless of its position
- `merge(other_queue)` — merge all entries from another priority queue into this one

## Advanced Operations

The `bulk_push`, `remove`, and `merge` operations require admin authorization,
demonstrating access control patterns for managing complex queue workflows.

- `bulk_push` is more efficient than repeated `push` calls because it heapifies
  the entire structure once after all insertions (O(n) vs O(n log n)).
- `remove` locates an item by value and removes it while maintaining heap integrity.
- `merge` reads entries from another queue contract and combines them via heapify.
