# Priority Queue

This intermediate example shows a heap-backed priority queue implementation in Soroban.

It demonstrates:

- `push(item, priority)` insertion
- `peek_max()` to inspect the highest-priority item
- `pop_max()` to remove and return the highest-priority item
- heap integrity via binary heap operations stored on-chain

## API

| Function | Purpose |
|----------|---------|
| `push(item, priority)` | Insert `item` and sift it up to its place |
| `peek_max()` | Highest-priority item, or `None` when empty |
| `pop_max()` | Remove and return the highest-priority item; panics when empty |
| `len()` / `is_empty()` | Queue size |
| `all()` | The raw heap array, useful for inspecting heap order |

## How It Works

The queue is a max-heap stored as a single `Vec<HeapEntry>` in persistent
storage. `push` appends and sifts up; `pop_max` swaps the last element into the
root and sifts down. Both are O(log n) in the number of entries, and the whole
vector is read and written per call — so this suits queues of tens to low
hundreds of entries, not unbounded ones.

The heap invariant is that every parent's priority is greater than or equal to
its children's, which makes index `0` the maximum at all times.

## Use Cases

- Ordered withdrawal or payout queues
- Priority-based task scheduling inside a contract
- Auction or bid ranking where the top entry is read repeatedly

## Running Tests

```bash
cargo test -p priority-queue
```

Tests cover push/peek/pop ordering, length tracking, the empty-queue panic, and
heap integrity across interleaved pushes and pops.
