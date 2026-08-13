# Contract Integration Guide

This guide explains how to integrate Auctra Soroban contracts with the backend services.

## Backend Integration

The backend provides TypeScript services for interacting with deployed contracts.

### Setup

1. **Install dependencies:**
   ```bash
   cd backend
   pnpm install
   ```

2. **Set environment variables:**
   ```bash
   export STELLAR_NETWORK="testnet"
   export STELLAR_SECRET_KEY="your-secret-key"
   export STELLAR_HORIZON_URL="https://horizon-testnet.stellar.org"
   export CONTRACT_ORACLE="<oracle-contract-id>"
   export CONTRACT_RESERVE_TRACKER="<reserve-tracker-contract-id>"
   export CONTRACT_MINTING="<minting-contract-id>"
   export CONTRACT_BURNING="<burning-contract-id>"
   # Optional (segment features):
   export CONTRACT_SAVINGS_VAULT="<savings-vault-contract-id>"
   export CONTRACT_LENDING_POOL="<lending-pool-contract-id>"
   export CONTRACT_ESCROW="<escrow-contract-id>"
   ```

### Using Contract Services

#### Minting Service (auctraMintingService)

```typescript
import { auctraMintingService } from './services/contracts';

// Mint Auctra from USDC
const result = await auctraMintingService.mintFromUsdc({
  usdcAmount: '10000000', // 10 USDC (7 decimals)
  recipient: 'G...', // Stellar address
});

console.log('Transaction:', result.transactionHash);
console.log('Auctra minted:', result.auctraAmount);

// Check if contract is paused
const isPaused = await auctraMintingService.isPaused();

// Get fee rate
const feeRate = await auctraMintingService.getFeeRate();
```

#### Burning Service (auctraBurningService)

```typescript
import { auctraBurningService } from './services/contracts';

// Burn Auctra for currency redemption
const result = await auctraBurningService.burnForCurrency({
  auctraAmount: '10000000', // 10 Auctra
  currency: 'NGN',
  recipientAccount: {
    accountNumber: '1234567890',
    bankCode: '058',
    accountName: 'John Doe',
  },
});

console.log('Transaction:', result.transactionHash);
console.log('Local amount:', result.localAmount);
```

#### Oracle Service (auctraOracleService)

```typescript
import { auctraOracleService } from './services/contracts';

// Update exchange rate (validator function)
await auctraOracleService.updateRate({
  currency: 'NGN',
  rate: '1234567', // 0.1234567 USD per NGN (7 decimals)
  sources: ['1230000', '1235000', '1239000'],
  timestamp: Date.now(),
});

// Get current rate
const rate = await auctraOracleService.getRate('NGN');

// Get Auctra/USD rate
const auctraRate = await auctraOracleService.getAuctraUsdRate();
```

#### Reserve Tracker Service (auctraReserveTrackerService)

```typescript
import { auctraReserveTrackerService } from './services/contracts';

// Update reserve (backend function)
await auctraReserveTrackerService.updateReserve({
  currency: 'NGN',
  amount: '1000000000', // Reserve amount
  valueUsd: '123456700', // Value in USD
});

// Get reserve data
const reserve = await auctraReserveTrackerService.getReserve('NGN');

// Verify reserves
const isValid = await auctraReserveTrackerService.verifyReserves();

// Get total reserve value
const totalValue = await auctraReserveTrackerService.getTotalReserveValue();
```

#### Savings, Lending, Escrow (optional)

When `CONTRACT_SAVINGS_VAULT`, `CONTRACT_LENDING_POOL`, or `CONTRACT_ESCROW` are set, the backend exposes `auctraSavingsVaultService`, `auctraLendingPoolService`, and `auctraEscrowService`. Segment routes (`/v1/savings`, `/v1/lending`, `/v1/gateway`) call these services. Event listeners `auctra_savings_vault_event_listener.ts`, `auctra_lending_pool_event_listener.ts`, and `auctra_escrow_event_listener.ts` enqueue events to **AUCTRA_SAVINGS_VAULT_EVENTS**, **AUCTRA_LENDING_POOL_EVENTS**, and **AUCTRA_ESCROW_EVENTS** respectively.

### Event Listening

Backend jobs listen to contract events and enqueue for off-chain processing:

- **auctra_minting_event_listener.ts** – MintEvent → USDC_CONVERSION queue
- **auctra_burning_event_listener.ts** – BurnEvent → WITHDRAWAL_PROCESSING queue
- **auctra_savings_vault_event_listener.ts** → AUCTRA_SAVINGS_VAULT_EVENTS
- **auctra_lending_pool_event_listener.ts** → AUCTRA_LENDING_POOL_EVENTS
- **auctra_escrow_event_listener.ts** → AUCTRA_ESCROW_EVENTS

### Error Handling

All contract services throw errors that should be caught:

```typescript
try {
  const result = await auctraMintingService.mintFromUsdc({
    usdcAmount: '10000000',
    recipient: 'G...',
  });
} catch (error) {
  if (error.message.includes('Insufficient reserves')) {
    // Handle insufficient reserves
  } else if (error.message.includes('Contract is paused')) {
    // Handle paused contract
  } else {
    // Handle other errors
  }
}
```

## Contract Interaction Flow

### Minting Flow

1. User deposits USDC or fiat
2. Backend calls `auctraMintingService.mintFromUsdc()` or `auctraMintingService.mintFromFiat()`
3. Contract verifies reserves and mints Auctra
4. Contract emits `MintEvent`
5. Backend event listener processes event
6. Backend triggers USDC conversion worker (if needed)

### Burning Flow

1. User requests Auctra redemption
2. Backend calls `auctraBurningService.burnForCurrency()` or `auctraBurningService.burnForBasket()`
3. Contract burns Auctra and emits `BurnEvent`
4. Backend event listener processes event
5. Backend triggers withdrawal processor
6. Fiat is disbursed to user's account

### Oracle Update Flow

1. Validator fetches rates from multiple sources
2. Validator calls `auctraOracleService.updateRate()`
3. Contract calculates median and updates rate
4. Contract emits `RateUpdateEvent`
5. Backend updates database with new rates

### Reserve Update Flow

1. Backend tracks reserves from fintech partners
2. Backend calls `auctraReserveTrackerService.updateReserve()`
3. Contract stores reserve data
4. Backend verifies reserves using `auctraReserveTrackerService.verifyReserves()`

## Testing

### Unit Tests

```typescript
import { auctraMintingService } from './services/contracts';

describe('auctraMintingService', () => {
  it('should mint Auctra from USDC', async () => {
    const result = await auctraMintingService.mintFromUsdc({
      usdcAmount: '10000000',
      recipient: 'G...',
    });
    
    expect(result.transactionHash).toBeDefined();
    expect(result.auctraAmount).toBeDefined();
  });
});
```

### Integration Tests

Test full flows with deployed contracts:

```typescript
describe('Mint-Burn Flow', () => {
  it('should complete full mint and burn cycle', async () => {
    // Mint
    const mintResult = await auctraMintingService.mintFromUsdc({...});
    
    // Burn
    const burnResult = await auctraBurningService.burnForCurrency({...});
    
    expect(mintResult.transactionHash).toBeDefined();
    expect(burnResult.transactionHash).toBeDefined();
  });
});
```

## Best Practices

1. **Always check contract state** before operations (paused, fee rates, etc.)
2. **Handle errors gracefully** with appropriate user feedback
3. **Monitor events** for off-chain processing
4. **Verify reserves** before minting operations
5. **Use proper error handling** for all contract calls
6. **Log all transactions** for audit purposes
7. **Test thoroughly** on testnet before mainnet

## Security Considerations

1. **Never expose secret keys** in client-side code
2. **Validate all inputs** before calling contracts
3. **Use rate limiting** for contract operations
4. **Monitor for suspicious activity**
5. **Implement circuit breakers** for contract failures
6. **Use multisig** for admin operations
