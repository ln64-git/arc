## Description

A Rust-built decentralized archive system with cryptographic verification, end-to-end encryption, and source-agnostic transport—designed for long-term data preservation without cloud dependency.

## Skills / Tools / Stack

- Rust
- Cryptography
- Distributed Systems
- Content-Addressed Storage
- Peer-to-Peer Networking

# Summary

Arc is a local-first archive system. Your data, your keys, your devices.

Think of it as:

- **Git** — but for everything, not just code
- **AirDrop** — but works over USB, local network, internet, or a drive in your backpack
- **Internet Archive** — but running on your own hardware
- **BitTorrent** — but optimized for integrity, versioning, and long-term preservation

No cloud required. No account required. No trust required.

## Why Arc?

Hard drives fail. Services shut down. Companies get acquired and delete your library. Streaming platforms pull content without warning. Google kills products. Your data shouldn't depend on someone else's business model.

Arc lets you:

- Keep versioned backups you actually control
- Prove files haven't been tampered with
- Sync between your own devices without a middleman
- Share archives with family, friends, or community—over any connection
- Build a personal or community archive that outlasts any platform

## Who It's For

- Home server owners who want better backup and sync
- Data archivists preserving media, documents, research
- Families keeping photos, videos, records across generations
- Communities building shared libraries without platform dependency
- Anyone who's lost data to a service shutdown and said "never again"

## What It Does

1. **Preserves** — versioned, encrypted, content-addressed storage you control
2. **Verifies** — cryptographic proof that what you have is what was saved
3. **Syncs** — peer-to-peer over LAN, USB, internet, mesh, satellite—whatever moves bits

## Features

- Local-first—data lives on your devices, not someone else's server
- End-to-end encryption at rest and in transit
- Tamper-proof history with Merkle proofs
- Sync anywhere: USB drive, local network, across the internet, offline sneakernet
- No servers to maintain, no subscriptions, no lock-in
- Git-like workflow: init, add, commit, pull, push
- Every version saved—nothing gets overwritten or lost
- Lock and unlock files for access control

## Key Management

Start simple, strengthen when you're ready:

- **Tier 1** — Password-based. Easy setup. "This password is your archive."
- **Tier 2** — Device keys with backup seed on USB or paper
- **Tier 3** — Split your key across trusted people (3-of-5 to recover)
- **Tier 4** — Hardware keys for institutions and high-security needs

You can upgrade without starting over.

### Roadmap

1. Phase 1: Local archives with versioning and encryption (current)
2. Phase 2: Peer-to-peer sync over any medium
3. Phase 3: Arcnet—discover and connect with nearby archives
4. Selective sync for large collections
5. Key rotation and recovery tools

### Instructions

1. Build from source with `cargo build --release`
2. Initialize an archive with `arc init`
3. Add files with `arc add <file>`
4. Save a version with `arc commit`
5. Sync from peer or drive with `arc pull <source>`
6. Check integrity with `arc verify`

### License

MIT
