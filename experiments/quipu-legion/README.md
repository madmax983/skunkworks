# 🧬 Quipu Legion

> "To the glory of the Empire and the wisdom of the Andes."

**Quipu Legion** is a hybrid experiment visualizing RSA cryptography through the lens of Roman Numerals and Incan Quipu knots. It bridges two ancient civilizations to demonstrate modern public-key encryption.

## 🔬 Lineage

- **Parent A:** `experiments/spqr-rsa` (Roman Numeral Arithmetic & RSA)
  - *Allele Inherited:* `Roman` struct, base-10 symbol logic, cryptographic intent.
- **Parent B:** `experiments/quipu-symphony` (Quipu Data Structure & Visualization)
  - *Allele Inherited:* `Cord` and `Knot` structs, vertical data representation.

**Emergent Trait:** *Cryptographic Knotting*. The transformation of a message (Roman Numeral) into a ciphertext (Quipu Cord) visualizes the mathematical "knot" of the RSA trapdoor function.

## 🎮 Controls

| Key | Action |
|-----|--------|
| `G` | **Generate Keys** (Forges 64-bit RSA keypair) |
| `E` | **Enter Message** (Input mode) |
| `Enter` | **Encrypt** (In input mode) |
| `D` | **Decrypt** (Converts Quipu back to Roman) |
| `R` | **Reset** (Clear message) |
| `Q` | **Quit** |

## 🧪 Usage

1.  Launch with `cargo run -p quipu-legion`.
2.  Press `G` to generate a Public/Private keypair.
    - *Observe:* The Public Key (Modulus N, Exponent E) is displayed in Roman Numerals.
3.  Type a number (e.g., `42`) or a Roman Numeral (e.g., `XLII`).
4.  Press `Enter` to encrypt.
    - *Observe:* The message is converted to a Quipu Cord (Ciphertext). The knots represent the encrypted value $C = M^e \mod n$.
5.  Press `D` to decrypt.
    - *Observe:* The knots are untied, revealing the original Roman Numeral $M = C^d \mod n$.

## 🧬 Splice Surgeon's Notes

> "Fascinating. The rigid structure of the Roman system (Base-10, symbolic) maps perfectly to the positional knots of the Quipu. By using RSA, we turn the act of encryption into the physical act of tying complex knots that only the key-holder can untie. The hybrid exhibits 'Textile Cryptography'."
