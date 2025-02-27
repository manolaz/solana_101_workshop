# Module 3: Solana Marketplace with Anchor

This module demonstrates key Anchor concepts by implementing a simple marketplace program where users can:

1. Create listings for items with a specified price
2. Purchase items from listings
3. Cancel listings

## Key Concepts Demonstrated

### Program Structure with Account Validation
- Structured account validation using Anchor's constraint system
- Proper account initialization and data storage
- Input validation and error handling

### PDA Usage for Escrow and Listing Management
- Using PDAs to derive deterministic addresses for listings and escrow accounts
- Proper authority delegation for token transfers via PDAs
- Secure handling of escrow accounts

### Cross-Program Invocation with SPL Tokens
- Token transfers between user accounts
- Token transfers from/to escrow accounts
- Proper authority and signer handling

### Error Handling and Validation
- Custom error types with descriptive messages
- Input validation with proper error reporting
- Safe arithmetic operations to prevent overflows

### Testing with the Anchor Framework
- Setting up token mints and accounts for testing
- Testing all program instructions with assertions
- Verifying token balances and account state

## Getting Started

```bash
# Install dependencies
npm install

# Build the program
anchor build

# Deploy to localnet
anchor deploy

# Run tests
anchor test
```

## Program Instructions

### create_listing

Creates a new listing for an item.

Parameters:
- `price`: The price per item in payment tokens
- `quantity`: The number of items to list
- `name`: A descriptive name for the item (max 32 chars)

### purchase

Purchases items from an existing listing.

Parameters:
- `quantity`: The number of items to purchase

### cancel_listing

Cancels a listing and returns the items to the seller.

No parameters required.
