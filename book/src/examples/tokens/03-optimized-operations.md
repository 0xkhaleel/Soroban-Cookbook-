# 03 · Optimized Operations

**Source:** [`examples/tokens/03-optimized-operations/`](https://github.com/Soroban-Cookbook/Soroban-Cookbook-/tree/main/examples/tokens/03-optimized-operations)

Optimizations for cross-contract calls, including argument packing, call batching, and minimized round trips. Includes benchmarks comparing naïve and optimized implementations.

## What You'll Learn

- Argument packing to reduce cross-contract calldata overhead
- Call batching to minimize cross-contract round trips
- Benchmark harness using `cargo bench`

## Optimizations

| Technique | Saving |
|-----------|--------|
| Argument packing | Fewer bytes in calldata → lower fees |
| Call batching | Single transaction for multiple calls → fewer round trips |
| Minimized round trips | Reduced overhead per cross-contract interaction |

## Run the Example

```bash
cd examples/tokens/03-optimized-operations
cargo test
cargo bench   # compare before/after
```

## Next: [04 · Mint / Burn](./04-mint-burn.md)
