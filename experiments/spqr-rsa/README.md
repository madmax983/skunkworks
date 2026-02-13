# SPQR RSA: Cryptographia Romana

A Terminal User Interface (TUI) application that visualizes the RSA encryption algorithm using Roman Numerals.

## Concept

This experiment reimagines modern cryptography through the lens of Ancient Rome. It demonstrates the core principles of RSA (asymmetric key encryption) but presents all numbers in the Roman numeral system.

- **MODULUS (n)**: The product of two prime numbers.
- **PUBLICUS (e)**: The public exponent.
- **PRIVATUS (d)**: The private exponent.
- **NUNTIUS**: The message to be encrypted.
- **CRYPTA**: The encrypted ciphertext.
- **REVELATIO**: The decrypted message.

## Usage

Run the application with:
```bash
cargo run -p spqr-rsa
```

### Controls

| Key | Action | Description |
|-----|--------|-------------|
| **G** | Generate Keys | Generates a new RSA keypair (using small 16-bit primes for speed). |
| **E** | Encrypt | Encrypts the number 42 (XLII) with the public key. |
| **D** | Decrypt | Decrypts the ciphertext with the private key. |
| **Q** | Quit | Exits the application. |
| **Esc** | Quit | Exits the application. |

## Technical Details

- Built with `ratatui` for the TUI.
- Uses `num-bigint` for arbitrary precision arithmetic (though keys are kept small for demo purposes).
- Implements a custom `Roman` struct for handling Roman numeral conversion and arithmetic.
