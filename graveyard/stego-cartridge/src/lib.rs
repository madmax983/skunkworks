//! # Stego-Cartridge
//!
//! A fantasy console where executable programs are steganographically hidden
//! inside PNG images. This crate provides the foundational components:
//! a custom bytecode Virtual Machine ([`vm`]), an assembler ([`asm`]), and
//! the steganographic embedding/extracting utilities ([`stego`]).
//!
//! "The Cartridge is the Code."

/// Assembly compiler for the Stego-Cartridge bytecode.
pub(crate) mod asm;
/// LSB steganography operations for hiding/extracting programs in images.
pub(crate) mod stego;
/// The core stack-based Virtual Machine and instruction set.
pub(crate) mod vm;

// Facade API
pub use asm::*;
pub use stego::*;
pub use vm::*;
