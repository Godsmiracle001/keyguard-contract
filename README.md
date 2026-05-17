# keyguard-contract

> The Soroban smart contract powering trustless key ownership, multi-sig enforcement, and decentralized account recovery for KeyGuard on the Stellar blockchain.

---

## Overview

`keyguard-contract` is the on-chain layer of KeyGuard. Written in Rust using the Soroban SDK, it enforces the rules of key registration, multi-signature thresholds, and guardian-based account recovery in a fully trustless and verifiable way. No centralized server can override these rules — all critical operations are validated on-chain before execution.

This repo is part of the KeyGuard monorepo ecosystem:

| Repo | Description |
|---|---|
| [keyguard-app](https://github.com/keyguard-stellar/keyguard-frontend) | Next.js frontend |
| [keyguard-api](https://github.com/keyguard-stellar/keyguard-api) | NestJS REST API |
| **keyguard-contract** | Soroban smart contract (this repo) |

---

## Tech Stack

- **Language:** Rust
- **Smart Contract SDK:** soroban-sdk 20.x
- **Network:** Stellar (Soroban)
- **Target:** `wasm32-unknown-unknown`
- **Tooling:** Soroban CLI, Makefile

---

## Contract Functions

### Key Registry

| Function | Description |
|---|---|
| `register_key(env, owner, key_id, label)` | Register a new key on-chain for an owner |
| `revoke_key(env, owner, key_id)` | Remove a key from the registry |

### Multi-Sig

| Function | Description |
|---|---|
| `set_threshold(env, owner, threshold)` | Set the required approval threshold for an account |
| `get_threshold(env, owner)` | Read the current threshold setting |

### Guardians

| Function | Description |
|---|---|
| `add_guardian(env, owner, guardian)` | Add a trusted recovery guardian address |
| `remove_guardian(env, owner, guardian)` | Remove a guardian address |
| `list_guardians(env, owner)` | Return all guardians for an owner |

### Recovery

| Function | Description |
|---|---|
| `initiate_recovery(env, lost_key, new_key, guardian_signatures)` | Execute key recovery with majority guardian approval |

### Admin

| Function | Description |
|---|---|
| `set_admin(env, new_admin)` | Transfer admin role |
| `pause(env)` | Pause the contract (emergency) |
| `unpause(env)` | Resume normal contract operation |

---

## Events

The contract emits the following events for off-chain indexing:

| Event | Trigger |
|---|---|
| `KeyRegistered` | A new key is successfully registered |
| `KeyRevoked` | A key is removed from the registry |
| `GuardianAdded` | A guardian address is added |
| `GuardianRemoved` | A guardian address is removed |
| `RecoveryExecuted` | A recovery flow completes successfully |

---

## Project Structure

```
keyguard-contract/
├── Cargo.toml                      # Workspace + dependency config
├── Makefile                        # Build, test, deploy targets
├── .env.example                    # Required environment variables
├── deployments.json.example        # Deployment output template
├── src/
│   ├── lib.rs                      # Crate root, module exports
│   ├── contract.rs                 # Main #[contract] entry points
│   ├── types.rs                    # Shared structs and enums (KeyRecord, DataKey, etc.)
│   ├── storage.rs                  # Storage helpers (get/set/remove/has)
│   ├── register.rs                 # register_key() implementation
│   ├── revoke.rs                   # revoke_key() implementation
│   ├── multisig.rs                 # Threshold storage and enforcement
│   ├── guardians.rs                # Guardian list management
│   ├── recovery.rs                 # Recovery execution with signature validation
│   ├── admin.rs                    # Admin role, pause/unpause
│   ├── events.rs                   # Event emission helpers
│   └── tests/
│       ├── mod.rs
│       ├── register_tests.rs
│       ├── recovery_tests.rs
│       ├── multisig_tests.rs
│       └── guardian_tests.rs
└── scripts/
    ├── deploy.sh                   # Build + deploy to testnet
    └── verify.sh                   # Post-deploy verification call
```

---

## Getting Started

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Soroban CLI installed
- A funded Stellar testnet keypair

### Install Rust target

```bash
rustup target add wasm32-unknown-unknown
```

### Install Soroban CLI

```bash
cargo install --locked soroban-cli
```

### Environment Variables

```bash
cp .env.example .env
```

```env
STELLAR_NETWORK=testnet
DEPLOYER_SECRET_KEY=S...
ADMIN_PUBLIC_KEY=G...
```

---

## Build

```bash
make build
# or
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM will be at `target/wasm32-unknown-unknown/release/keyguard_contract.wasm`.

---

## Test

```bash
make test
# or
cargo test
```

All unit tests use `soroban_sdk::testutils::Env::default()` and run fully in-process without requiring a network connection.

---

## Deploy to Testnet

```bash
make deploy
```

This will:
1. Build the WASM binary
2. Deploy to Stellar testnet via Soroban CLI
3. Call `set_admin` to initialize the contract
4. Write the contract ID and deployment timestamp to `deployments.json`

### Verify deployment

```bash
make verify
```

Calls `get_threshold` on the deployed contract to confirm it is live and responsive.

---

## Security Model

KeyGuard's contract is designed around three security principles:

**Invoker authorization** — all mutating functions require `env.invoker()` to be the key owner. No third party can modify another user's keys.

**Guardian majority** — account recovery requires a majority of the owner's designated guardians to co-sign the recovery payload. A single compromised guardian cannot authorize recovery alone.

**Admin pause** — in the event of a critical vulnerability, the admin can pause the contract to prevent new registrations and revocations while a fix is prepared. The admin role can be transferred or renounced.

---

## Contributing

This repository participates in the **Stellar Wave Program** on Drips Wave. Contributors can pick up scoped issues during active Wave cycles and earn points for merged work.

### How to contribute

1. Browse open issues labeled `Stellar Wave` in this repository.
2. Apply to work on an issue via the [Drips Wave app](https://wave.drips.network).
3. Wait for the maintainer to assign you.
4. Fork the repo, create a branch named `feat/KG-CON-XXX-short-description`, and open a PR against `main`.

### Branch naming

```
feat/KG-CON-003-register-key
fix/KG-CON-007-recovery-signature-check
test/KG-CON-009-unit-test-suite
```

### Pull Request checklist

- [ ] `cargo build --target wasm32-unknown-unknown` succeeds with zero warnings
- [ ] `cargo test` passes with zero failures
- [ ] New functions have corresponding unit tests covering happy path and error cases
- [ ] Events emitted where appropriate
- [ ] PR description references the issue number (e.g. `Closes KG-CON-005`)

---

## Issue Complexity & Points

| Label | Complexity | Points |
|---|---|---|
| `complexity: trivial` | Scaffold, deploy scripts, docs | 100 pts |
| `complexity: medium` | Storage schema, CRUD functions, tests | 150 pts |
| `complexity: high` | Multi-sig logic, recovery execution, admin | 200 pts |

---

## License

MIT — see [LICENSE](./LICENSE) for details.