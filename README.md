# Solana Anchor Vault Stream Protocol

![Banner](assets/banner.svg)

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Solana](https://img.shields.io/badge/Solana-Mainnet%20%2F%20Devnet-9945FF?logo=solana)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-0.30.1-14F195)](https://coral-xyz.github.io/anchor/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.5-3178C6?logo=typescript)](https://www.typescriptlang.org/)

> **A high-performance Solana smart contract and TypeScript SDK for time-weighted continuous streaming payments and linear token vesting.**

---

## 🌟 Key Features

- **Continuous Per-Second Vesting**: Computes deterministic token entitlement mathematically based on clock timestamps with zero rounding loss.
- **Cliff Period Support**: Configurable cliff enforcement preventing early withdrawals before project milestones are achieved.
- **PDA Vault Security**: Non-custodial escrow vaults governed strictly by Anchor Seeds (`[b"stream", sender, stream_id]` & `[b"vault", stream]`).
- **Client SDK Included**: TypeScript SDK providing PDA derivation utilities, vesting simulators, and instruction builders.

---

## 🛠️ Instructions Specification

### 1. `initialize_stream`
Creates a stream escrow PDA and deposits SPL tokens into the program-controlled vault:
- `stream_id`: Unique sequence number per sender
- `total_amount`: Total SPL tokens locked in the stream
- `start_time`: Unix timestamp when vesting begins
- `end_time`: Unix timestamp when 100% of tokens are released
- `cliff_time`: Unix timestamp before which 0 tokens can be claimed

### 2. `withdraw`
Called by the designated recipient to claim unlocked vested tokens continuously.

---

## 🚀 Quick Start

### Build & Test TypeScript SDK

```bash
npm install
npm run test
npm run build
```

---

## 📄 License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.