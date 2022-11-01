# Kujira Factory

This contract allows for permission-less contract instantiation, whilst maintaining the security guarantees of the permissioned nature of the WASM module.

## Context

Kujira is a semi-permissioned blockchain. Smart contracts can _only_ be deployed (Instantiated) via the passing of a governance proposal.
This is to ensure a threshold of security and quality to the code that runs on the network.

However multiple proposals for the same contract (eg vaults) can get tiresome. There is a feature of the wasm module which allows the permitting of a single
address to Instantate a specific CODE ID. (`kujirad tx gov submit-proposal update-instantiate-config {codeid},{address}`). This allows contracts to be Instantiated
by `address`, without requiring a governance vote every time.

The shortcoming of this permission is that, if granted to a regular wallet account, the `admin` parameter of the instantiated code can be set to any value. This `admin`
address is able to `migrate` the instantiated code to a brand new Code ID, for which there is no requirement for any of the checks that the governance permissions create,
undermining the basic principles of a semi permissioned chain.

## Solution

This contract acts as a generic "factory" contract. Once instantiated itself, its own address can be used in the `tx gov submit-proposal update-instantiate-config` command.
It restricts the `admin` address to only that supplied during Instantiate, typically set to the governance module, so that any contracts instantiated via this method can _only_ be migrated via a governance vote.
