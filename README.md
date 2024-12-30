# NFT Minting Smart Contract

## Part 2: Rust Task Objective

The objective of this task is to implement a basic NFT minting smart contract on the Solana blockchain using the Rust programming language and the Anchor framework.

## Project Overview

This project features a smart contract that allows users to mint unique Non-Fungible Tokens (NFTs). Each NFT will have a unique ID and associated metadata, including a title, description, and URI.

## Requirements

### Features

The smart contract implements the following functionalities:

1. **Minting NFTs**:

   - **Function**: `mint_nft(owner: Pubkey, metadata: String)`
   - Description: Mints an NFT for the specified owner with the given metadata.

2. **Retrieving NFT Details**:

   - **Function**: `get_nft(id: u64)`
   - Description: Retrieves details of an NFT by its unique ID.

3. **Transferring Ownership**:
   - **Function**: `transfer_nft(id: u64, new_owner: Pubkey)`
   - Description: Transfers ownership of an NFT to a new owner, ensuring ownership validation before transfer.

## Deliverables

- The main Rust contract code is located in `nft_minting.rs`.
- Test cases are written in Ts using the Anchor framework to ensure functionality and security.

## Evaluation Criteria

The project will be evaluated based on the following criteria:

1. **Code Quality**:

   - Code should be clean, readable, and follow best practices for Rust development.

2. **Functionality**:

   - The contract must fulfill all specified requirements without errors.

3. **Security**:

   - Proper handling of edge cases and potential vulnerabilities should be implemented.

4. **Testing**:

   - Comprehensive test coverage for all functions must be provided to ensure reliability.

5. **Documentation**:
   - A brief explanation of design choices, contract workflow, and instructions on how to run tests should be included.

## Getting Started

### Prerequisites

Ensure you have the following installed:

- Rust programming language
- Solana CLI
- Anchor framework

### Installation

1. Clone the repository:

### Running Build and Tests

To run the tests for this smart contract, use:

```shell
anchor build
anchor test
```

## Design Choices

- The contract utilizes the Anchor framework for simplified development and security features.
- Unique IDs are generated using a counter mechanism to ensure no duplicates occur.
- Metadata is stored in a structured format to facilitate easy retrieval and display.
