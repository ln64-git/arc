## Description

A decentralized archive system with cryptographic verification, end-to-end encryption, and source-agnostic transport. Scales from family photo sharing to sensitive journalism or intelligence operations.

## Skills / Tools / Stack

- Rust
- Cryptography
- Distributed Systems
- Content-Addressed Storage
- Peer-to-Peer Networking

# Summary

Arc is infrastructure for data that matters. The same architecture serves a family backing up photos and an intelligence network distributing sensitive material across hostile territory.

Think of it as:

- **Git** — but for data, not code
- **AirDrop** — but source agnostic
- **Internet Archive** — but decentralized
- **BitTorrent** — but optimized for integrity, versioning, and long-term preservation

What changes isn't the system—it's the topology.

## Network Topologies at Scale

**Personal Archive** — 1 node

- Laptop with encrypted local storage
- USB backup in a fireproof safe
- Version history, integrity verification
- You're the only node, but your data survives hardware failure

**Family Network** — 5-10 nodes

- Parents, siblings, grandparents each running a node
- Shared photo/video library syncs automatically when devices connect
- No cloud subscription, no platform lock-in
- Archive survives even if half the family loses their devices

**Community Library** — 50-200 nodes

- Local community, interest group, or extended network
- Shared media, documents, resources
- Sync at meetups, over local network, or through trusted relays
- Redundancy through distribution—no single point of failure

**Distributed Collective** — 500+ nodes

- Geographic distribution across cities, countries, continents
- High redundancy, high availability
- Subset syncs keep the archive alive even with significant node loss
- Think: community preservation projects, cultural archives, open research

---

## Examples That Define this Architecture

**Local Art Collective** 

- Artists sharing Dropbox got expensive, Google Drive keeps compressing everything
- Shared folder of music, posters, art, lost-media
- Sync at studio hangs or over home internet—no monthly subscription
- Version history means you can recover that iteration from three weeks ago
- The archive belongs to the group, not a platform that might change terms or shut down

**Media Server Owners**

- You built a home media library—movies, music, photos, personal recordings
- Streaming services remove content, cloud storage has monthly fees
- Your server works great until a drive fails and years of collecting disappears
- Arc syncs your library to other nodes: friend's place, family member, off-site backup
- If one copy goes down, the others keep everything intact
- Your collection, your terms, backed up without subscription costs

**Generational Preservation**

- Family archives meant to last 50+ years
- Current cloud providers won't exist in 20 years
- Key recovery through trusted family members (3-of-5 to recover)
- The archive outlives any individual, any company, any platform

**Journalist Network**

- Reporter in the field, editors in different countries, sources who can never be identified
- Material syncs through encrypted channels, dead drops, or physical handoff
- No central server to subpoena, no logs to seize
- Source protection isn't a feature—it's the topology

**Dissident Communication**

- Activists in hostile environments need to share information
- Government controls internet infrastructure, monitors cloud services
- Arc works over mesh networks, USB drives, local radio—whatever moves bits
- Seizure of one node doesn't compromise the network or identify others

**Corporate Espionage Defense**

- Sensitive R&D, trade secrets, strategic planning
- Data stays on controlled hardware, never touches third-party infrastructure
- Cryptographic proof of integrity—detect tampering immediately
- Access control through key distribution, not account permissions

---

## What Makes This Different

**Source-Agnostic Transport**

- Arc doesn't care how data moves—LAN, internet, USB, sneakernet, mesh, satellite
- Sync protocol works the same whether you're on home WiFi or passing a drive across a border
- The network adapts to whatever channels are available

**Cryptographic Chain of Custody**

- Every version signed, every change tracked
- Merkle proofs verify integrity without exposing content
- You can prove what you have is what was saved, and when

**Trust Through Topology, Not Authority**

- No central server, no certificate authority, no company to trust
- Security comes from key management and network structure
- You decide who can read, who can write, who can verify

**Graceful Degradation**

- Network loses 80% of nodes? Archive survives on the rest
- Internet goes down? Sync over local network or physical media
- Your device is seized? You have nothing to decrypt—keys are elsewhere

## Key Management

Security scales with stakes:

- **Tier 1** — Password-based. Easy setup for personal use.
- **Tier 2** — Device keys with offline backup. Recommended for families.
- **Tier 3** — Shamir's secret sharing (3-of-5 to recover). For communities and long-term preservation.
- **Tier 4** — Hardware security modules. For institutional and high-security deployments.

Upgrade without starting over. Rotate keys without losing history.

## Features

- Content-addressed storage with cryptographic verification
- End-to-end encryption at rest and in transit
- Merkle-tree history—tamper-evident versioning
- Transport-agnostic sync: TCP, USB, file copy, custom protocols
- Git-like CLI: init, add, commit, pull, push, verify
- Selective sync for large archives
- Access control through key distribution
- Works offline, syncs opportunistically
- No servers, no subscriptions, no dependencies

### Roadmap

1. Phase 1: Local archives with versioning and encryption (current)
2. Phase 2: Peer-to-peer sync over any medium
3. Phase 3: Arcnet—discovery and relay for distributed networks
4. Anonymous relay nodes for sensitive deployments
5. Hardware key integration
6. Mobile clients for field operations

### Instructions

1. Build from source with `cargo build --release`
2. Initialize an archive with `arc init`
3. Add files with `arc add <file>`
4. Save a version with `arc commit`
5. List archive contents with `arc list` (use `--full` for details)
6. Sync from peer or drive with `arc pull <source> [file]`
7. Encrypt files with `arc lock [file]`
8. Decrypt files with `arc unlock [target]`

### License

MIT
