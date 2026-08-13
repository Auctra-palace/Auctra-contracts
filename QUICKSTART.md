# Auctra Soroban Contracts - Quick Start

This is a quick start guide to get you up and running with Auctra Soroban contracts.

## What's Included

- **4 Core Contracts:**
  - Minting Contract - USDC/Fiat → Auctra
  - Burning Contract - Auctra → Fiat
  - Oracle Contract - Exchange rate aggregation
  - Reserve Tracker Contract - Reserve verification

- **Backend Integration:**
  - Stellar SDK client
  - Contract interaction services
  - Event listeners
  - TypeScript services for all contracts

## Quick Setup

### 1. Install Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Soroban CLI
cargo install --locked soroban-cli
```

### 2. Build Contracts

```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

### 3. Deploy to Testnet

```bash
export STELLAR_SECRET_KEY="your-testnet-secret-key"
chmod +x scripts/deploy_testnet.sh
./scripts/deploy_testnet.sh
```

### 4. Set Backend Environment Variables

```bash
export STELLAR_NETWORK="testnet"
export STELLAR_SECRET_KEY="your-secret-key"
export CONTRACT_ORACLE="<oracle-contract-id>"
export CONTRACT_RESERVE_TRACKER="<reserve-tracker-contract-id>"
export CONTRACT_MINTING="<minting-contract-id>"
export CONTRACT_BURNING="<burning-contract-id>"
```

### 5. Use in Backend

```typescript
import { mintingService } from './services/contracts';

// Mint Auctra from USDC
const result = await mintingService.mintFromUsdc({
  usdcAmount: '10000000', // 10 USDC
  recipient: 'G...',
});
```

## Next Steps

- Read [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment instructions
- Read [INTEGRATION.md](INTEGRATION.md) for backend integration guide
- Read individual contract READMEs for contract-specific documentation

## Project Structure

```
.
├── auctra_minting/
├── auctra_burning/
├── auctra_oracle/
├── auctra_reserve_tracker/
├── auctra_savings_vault/
├── auctra_lending_pool/
├── auctra_escrow/
├── auctra_multisig/
├── shared/
├── scripts/
├── docs/
├── tests/
└── README.md
```

## Support

For issues or questions:
1. Check the documentation files
2. Review contract READMEs
3. Check deployment logs
4. Verify environment variables
