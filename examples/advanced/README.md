# Advanced Soroban Examples

This folder contains the production-oriented patterns in the cookbook: authorization, timelocks, upgrades, oracles, bridges, optimization, and system-level safety controls. These examples assume you are already comfortable with the fundamentals from the basics and intermediate sections.

## Prerequisites

Before working through these examples, make sure you have:

- Rust stable installed and configured on your machine
- The `wasm32-unknown-unknown` target added:

```bash
rustup target add wasm32-unknown-unknown
```

- The Soroban CLI / Stellar CLI available in your environment
- A working understanding of:
  - `require_auth()` and authorization patterns
  - storage layout and instance vs persistent storage
  - events and error handling
  - contract interfaces and cross-contract calls
- Recommended: basic familiarity with the examples in [`../basics/`](../basics/) and [`../intermediate/`](../intermediate/)

## Difficulty Scale

- 🟢 Beginner: core concepts explained clearly with minimal tooling overhead
- 🟡 Intermediate: multi-step state logic or cross-contract coordination
- 🟠 Advanced: security-sensitive flows, upgrades, governance, and operational controls
- 🔴 Expert: full-system architecture, optimization, audits, or testing strategy

## Recommended Learning Path

For the fastest progression, follow this order:

1. Start with authorization and safety fundamentals
   - [`01-multi-party-auth`](./01-multi-party-auth/)
   - [`02-timelock`](./02-timelock/)
   - [`04-circuit-breaker`](./04-circuit-breaker/)

2. Learn pattern composition for production-grade contracts
   - [`03-rbac-modifiers`](./03-rbac-modifiers/)
   - [`03-registry-access-controls`](./03-registry-access-controls/)
   - [`05-hierarchical-access-control`](./05-hierarchical-access-control/)
   - [`05-rate-limiting`](./05-rate-limiting/)

3. Study oracle, bridge, and data movement patterns
   - [`03-oracle-pattern`](./03-oracle-pattern/)
   - [`03-data-aggregation-oracle`](./03-data-aggregation-oracle/)
   - [`03-cross-contract-optimization`](./03-cross-contract-optimization/)
   - [`03-cross-chain-bridge`](./03-cross-chain-bridge/)
   - [`04-oracle-integration`](./04-oracle-integration/)
   - [`04-bridge-validators`](./04-bridge-validators/)

4. Understand upgradeability and governance controls
   - [`02-beacon-proxy`](./02-beacon-proxy/)
   - [`03-beacon-proxy-factory`](./03-beacon-proxy-factory/)
   - [`03-proxy-admin`](./03-proxy-admin/)
   - [`04-upgradeable-proxy`](./04-upgradeable-proxy/)
   - [`06-beacon-management`](./06-beacon-management/)
   - [`07-upgrade-patterns`](./07-upgrade-patterns/)
   - [`11-version-registry`](./11-version-registry/)

5. Move into performance, safety, and validation
   - [`05-diamond-security`](./05-diamond-security/)
   - [`05-diamond-facets`](./05-diamond-facets/)
   - [`06-diamond-pattern`](./06-diamond-pattern/)
   - [`06-gas-optimization`](./06-gas-optimization/)
   - [`09-storage-optimization`](./09-storage-optimization/)
   - [`09-storage-layout-validator`](./09-storage-layout-validator/)
   - [`10-contract-migrations`](./10-contract-migrations/)

6. Finish with advanced testing and production hardening
   - [`03-gasless-relayer`](./03-gasless-relayer/)
   - [`03-permit-pattern`](./03-permit-pattern/)
   - [`03-merkle-airdrop`](./03-merkle-airdrop/)
   - [`03-merkle-whitelist`](./03-merkle-whitelist/)
   - [`05-batch-transfer`](./05-batch-transfer/)
   - [`08-batch-operations`](./08-batch-operations/)
   - [`09-fuzz-testing`](./09-fuzz-testing/)

## Quick Start

From the repository root:

```bash
cd examples/advanced/01-multi-party-auth
cargo test
cargo build --target wasm32-unknown-unknown --release
```

Use the same pattern for any advanced example in this folder.

## Example Catalog

### Authorization, Access Control, and Safety

| Example | Difficulty | Focus |
|---|---|---|
| [`01-multi-party-auth`](./01-multi-party-auth/) | 🟠 Advanced | Threshold authorization and multi-signer approval flows |
| [`02-timelock`](./02-timelock/) | 🟠 Advanced | Delayed execution and queued governance actions |
| [`03-rbac-modifiers`](./03-rbac-modifiers/) | 🟡 Intermediate | Role-based access through reusable modifiers |
| [`03-registry-access-controls`](./03-registry-access-controls/) | 🟡 Intermediate | Registry-driven access checks and permission lookup |
| [`04-circuit-breaker`](./04-circuit-breaker/) | 🟡 Intermediate | Pause/resume logic and failure-threshold recovery |
| [`05-hierarchical-access-control`](./05-hierarchical-access-control/) | 🟠 Advanced | Nested roles and escalation-safe permission trees |
| [`05-rate-limiting`](./05-rate-limiting/) | 🟡 Intermediate | Time- and volume-based operational controls |
| [`05-reentrancy-guard`](./05-reentrancy-guard/) | 🟠 Advanced | Re-entrancy defenses and invariant guard patterns |

### Oracles, Bridges, and Data Flows

| Example | Difficulty | Focus |
|---|---|---|
| [`03-oracle-pattern`](./03-oracle-pattern/) | 🟡 Intermediate | Basic oracle feed with freshness and validation |
| [`03-data-aggregation-oracle`](./03-data-aggregation-oracle/) | 🟠 Advanced | Aggregating and validating multiple price sources |
| [`03-cross-contract-optimization`](./03-cross-contract-optimization/) | 🟠 Advanced | Minimizing contract calls and storage churn |
| [`03-cross-chain-bridge`](./03-cross-chain-bridge/) | 🔴 Expert | Cross-chain release logic and bridge safety controls |
| [`04-oracle-integration`](./04-oracle-integration/) | 🟠 Advanced | Integrating external price feeds into contract logic |
| [`04-bridge-validators`](./04-bridge-validators/) | 🔴 Expert | Validator sets, threshold checks, and outbound safety |
| [`05-bridge-security`](./05-bridge-security/) | 🔴 Expert | Challenge windows, fraud-proof flows, and pause logic |

### Upgradeability, Proxies, and Versioning

| Example | Difficulty | Focus |
|---|---|---|
| [`02-beacon-proxy`](./02-beacon-proxy/) | 🟠 Advanced | Proxy pattern with upgradeable implementation targeting |
| [`03-beacon-proxy-factory`](./03-beacon-proxy-factory/) | 🟠 Advanced | Factory-managed beacon deployment and rolling upgrades |
| [`03-proxy-admin`](./03-proxy-admin/) | 🟠 Advanced | Timelocked admin-driven upgrades |
| [`04-upgradeable-proxy`](./04-upgradeable-proxy/) | 🟠 Advanced | Upgrade contracts with safe migration boundaries |
| [`06-beacon-management`](./06-beacon-management/) | 🟠 Advanced | Beacon versioning and rollback-friendly admin workflows |
| [`07-upgrade-patterns`](./07-upgrade-patterns/) | 🔴 Expert | Direct WASM upgrades, migration strategies, and init guards |
| [`10-contract-migrations`](./10-contract-migrations/) | 🔴 Expert | Follow-on storage migration and compatibility rules |
| [`11-version-registry`](./11-version-registry/) | 🟠 Advanced | Contract registry, historical version tracking, rollback support |
| [`contract-registry`](./contract-registry/) | 🟡 Intermediate | Centralized contract registry and address discovery |

### Batch Operations, Merkle Patterns, and Relayers

| Example | Difficulty | Focus |
|---|---|---|
| [`03-gasless-relayer`](./03-gasless-relayer/) | 🟠 Advanced | Meta-transactions, nonces, and signature validation |
| [`03-permit-pattern`](./03-permit-pattern/) | 🟡 Intermediate | Permit-style approvals with expiry enforcement |
| [`03-merkle-airdrop`](./03-merkle-airdrop/) | 🟠 Advanced | Merkle-based claim verification and efficient airdrops |
| [`03-merkle-whitelist`](./03-merkle-whitelist/) | 🟠 Advanced | Claim gating, whitelist verification, and deterministic proofs |
| [`05-batch-transfer`](./05-batch-transfer/) | 🟡 Intermediate | Batched asset movement and atomic transfer workflows |
| [`05-merkle-proofs`](./05-merkle-proofs/) | 🟡 Intermediate | Merkle root verification and proof validation |
| [`08-batch-operations`](./08-batch-operations/) | 🟠 Advanced | Batch interfaces with atomic rollback behavior |

### Optimization, Diamond, and Testing

| Example | Difficulty | Focus |
|---|---|---|
| [`05-diamond-facets`](./05-diamond-facets/) | 🔴 Expert | Facet-based modular contract architecture |
| [`05-diamond-security`](./05-diamond-security/) | 🔴 Expert | Security model for diamond architectures |
| [`06-diamond-pattern`](./06-diamond-pattern/) | 🔴 Expert | A complete diamond pattern implementation |
| [`06-gas-optimization`](./06-gas-optimization/) | 🟠 Advanced | Reducing storage and execution overhead |
| [`06-price-oracle`](./06-price-oracle/) | 🟠 Advanced | Oracle pricing logic and data freshness checks |
| [`07-trusted-forwarder`](./07-trusted-forwarder/) | 🟠 Advanced | Trusted meta-transaction forwarding and auth propagation |
| [`09-fuzz-testing`](./09-fuzz-testing/) | 🟠 Advanced | Property-based testing and fuzzing workflows |
| [`09-storage-layout-validator`](./09-storage-layout-validator/) | 🟠 Advanced | Contract layout validation and migration safety |
| [`09-storage-optimization`](./09-storage-optimization/) | 🟠 Advanced | Packed storage and lazy loading strategies |

## Suggested Reading Order by Experience

### New to Advanced Soroban

- [`01-multi-party-auth`](./01-multi-party-auth/)
- [`02-timelock`](./02-timelock/)
- [`04-circuit-breaker`](./04-circuit-breaker/)
- [`03-permit-pattern`](./03-permit-pattern/)

### Ready for Multi-Contract Systems

- [`03-oracle-pattern`](./03-oracle-pattern/)
- [`03-data-aggregation-oracle`](./03-data-aggregation-oracle/)
- [`03-cross-contract-optimization`](./03-cross-contract-optimization/)
- [`05-bridge-security`](./05-bridge-security/)

### Production and Upgrade Safety

- [`03-proxy-admin`](./03-proxy-admin/)
- [`07-upgrade-patterns`](./07-upgrade-patterns/)
- [`10-contract-migrations`](./10-contract-migrations/)
- [`11-version-registry`](./11-version-registry/)

### Performance and Hardening

- [`06-gas-optimization`](./06-gas-optimization/)
- [`09-storage-optimization`](./09-storage-optimization/)
- [`09-fuzz-testing`](./09-fuzz-testing/)
- [`05-diamond-security`](./05-diamond-security/)

## Best Practices

- Build from the simpler authorization and control patterns before moving to upgradeable systems.
- Treat timelocks, role checks, and bridge validation as security-critical primitives.
- Validate storage layout changes carefully before introducing migrations.
- Prefer explicit errors and typed state transitions over implicit fallback behavior.
- Combine examples such as multi-party auth + timelock + upgrade guard for real production systems.

## Related Documentation

- [`../../docs/advanced-patterns.md`](../../docs/advanced-patterns.md)
- [`../../docs/security-best-practices.md`](../../docs/security-best-practices.md)
- [`../../docs/governance-rbac-multisig-timelock.md`](../../docs/governance-rbac-multisig-timelock.md)

## Next Step

Choose a path based on your project goal: access control, oracle reliability, bridge safety, migration strategy, or optimization.
