# Rust AES-256-GCM Encryption

This project demonstrates using AES-256-GCM symmetric encryption in Rust with the Ring crypto library.

## Overview

This simple command line application: 

- Generates an AES-256 encryption key
- Accepts user input text to encrypt
- Encrypts the input text using AES-256-GCM 
- Decrypts the ciphertext back to plaintext
- Outputs the decrypted user text

It uses the [Ring](https://github.com/briansmith/ring) library for all cryptographic operations.

## Usage

To encrypt user input text:

```
cargo run
```

Follow the prompts to enter text to encrypt. The encrypted hexadecimal string will be printed, along with the decrypted plaintext to demonstrate the full roundtrip encryption.

## Implementation

The application logic is contained in `src/main.rs`:

- Key generation uses `ring::rand` and `ring::aead`
- Encryption uses `aead::seal()` with a random nonce
- Decryption uses `aead::open()` to decrypt the ciphertext
- Associated data ties the encryption to the application context

## Requirements

Rust 1.59.0 or later is required. Other dependencies are fetched by Cargo.

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for more details.