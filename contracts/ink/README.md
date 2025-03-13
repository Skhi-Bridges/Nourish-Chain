# Ink! Smart Contract Implementation

This directory contains the Ink! smart contract implementations for the NRSH (Nourish Chain) and ELXR (Elixir Chain) parachains. 

Ink! is a Rust-based smart contract language for Polkadot's Substrate. This framework ensures security, scalability, and seamless integration with the broader Polkadot ecosystem while supporting the unique requirements of decentralized food and beverage production.

For detailed information about the smart contract architecture, please see the [Smart Contract Framework](../../docs/smart-contract-framework.md) document.

## Structure

The smart contracts in this directory are organized according to the following structure:

- `governance/` - Governance-related contracts
- `registry/` - Registry contracts for products, ingredients, and certification
- `treasury/` - Treasury management contracts
- `token/` - Token implementation contracts
- `oracle/` - Oracle and data feed contracts
- `verification/` - Verification and compliance contracts

## Implementation

The implementation follows the Ink! 3.0 standard and is compatible with the Substrate/Polkadot ecosystem.

To build the contracts, use:

```bash
cargo +nightly contract build
```

To test the contracts, use:

```bash
cargo +nightly test
```
