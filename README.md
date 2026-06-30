# Padics

A Rust library for working with p-adic numbers and related mathematical operations.

## Overview

**⚠️ IMPORTANT: This library works exclusively with p-adic integers (Z_p), NOT with the full p-adic field Q_p.** All operations and implementations are limited to the ring of p-adic integers only.

This library provides a comprehensive implementation of p-adic integers in Rust, including arithmetic operations, polynomial root finding using Hensel's lemma, and computation of Teichmüller numbers. P-adic numbers are a fundamental concept in number theory, extending the arithmetic of rational numbers in a way that's analogous to how real numbers extend rational numbers through completion with respect to a different metric.

## Features

- **P-adic Integer Representation**: Trait-based implementation for flexible p-adic integer schemes (Z_p only)
- **Arithmetic Operations**: Addition, multiplication, division, exponentiation, and negation on p-adic integers
- **Rational Number Scheme**: Efficient representation of p-adic integers using rational coefficients
- **Hensel's Lemma**: Implementation for finding roots of polynomials in p-adic integers (Z_p)
- **Teichmüller Numbers**: Computation of Teichmüller representatives (roots of x^p - x) in Z_p
- **Mathematical Utilities**: Modular arithmetic, base-p conversion, and power modulo operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
padics = "0.1.0"
```

Or clone the repository and build locally:

```bash
git clone <repository-url>
cd padics
cargo build --release
```

## Usage

### Basic Example

```rust
use padics::padics::schemes::rational::RationalNumber;

const PRIME: u64 = 5;

// Create p-adic numbers
let one = RationalNumber::<PRIME>::one();
let minus_one = -RationalNumber::<PRIME>::one();

println!("ONE = {}", one);
println!("-ONE = {}", minus_one);
```

### Computing Teichmüller Numbers

```rust
use padics::features::teichmuller::compute_teichmuller_numbers;

const PRIME: u64 = 5;
let teichmuller_numbers = compute_teichmuller_numbers::<PRIME>();

for (index, t_number) in teichmuller_numbers.iter().enumerate() {
    println!("{}th Teichmüller number = {}", index, t_number);
}
```

### Finding Polynomial Roots with Hensel's Lemma

```rust
use padics::features::hensel_lemma::lemma::hensel_lemma;
use padics::features::hensel_lemma::polynomial::Polynomial;
use padics::padics::schemes::rational::RationalNumber;

const PRIME: u64 = 5;

// Find roots of X^3 - 2
let coefficients = vec![
    -RationalNumber::from_natural(2),
    RationalNumber::zero(),
    RationalNumber::zero(),
    RationalNumber::one()
];
let polynomial = Polynomial::<PRIME>::new(coefficients);

let roots = hensel_lemma(polynomial);

println!("5-ADIC ROOTS OF X^3 - 2");
for root in roots {
    println!("ROOT = {}", root);
}
```

## Project Structure

```
src/
├── main.rs              # Example usage and demonstrations
├── padics/              # Core p-adic number implementation
│   ├── core.rs          # Trait definitions (PAdicNumber, PAdicIter)
│   ├── arithmetic.rs    # Arithmetic operations
│   ├── constructors.rs  # Constructor functions
│   ├── schemes/         # Different p-adic number schemes
│   │   ├── rational.rs  # Rational number scheme
│   │   ├── sum.rs       # Addition scheme
│   │   ├── mul.rs       # Multiplication scheme
│   │   ├── div_p.rs     # Division by prime scheme
│   │   ├── pow.rs       # Power scheme
│   │   ├── neg.rs       # Negation scheme
│   │   └── mod.rs       # Module exports
│   └── mod.rs           # Module exports
├── features/            # Mathematical features
│   ├── hensel_lemma/    # Hensel's lemma implementation
│   │   ├── lemma.rs     # Main lemma implementation
│   │   ├── polynomial.rs # Polynomial structure
│   │   ├── scheme.rs    # Hensel number scheme
│   │   └── mod.rs       # Module exports
│   ├── teichmuller/     # Teichmüller numbers
│   │   ├── compute_teichmuller_numbers.rs
│   │   └── mod.rs       # Module exports
│   └── mod.rs           # Module exports
└── math_utils/          # Mathematical utilities
    ├── mod_inverse_prime.rs  # Modular inverse for primes
    ├── to_base_p.rs          # Base-p conversion
    ├── power_modulo.rs       # Power modulo operation
    └── mod.rs                # Module exports
```

## Mathematical Background

### P-adic Integers (Z_p)

**This library implements p-adic integers (Z_p), which are the ring of p-adic numbers without negative powers of p.** The full p-adic field Q_p includes fractions with negative powers of p, but this library is limited to Z_p only.

P-adic integers are the completion of the integers with respect to the p-adic metric, where the distance between two numbers is determined by the highest power of a prime p that divides their difference. Unlike real numbers, p-adic integers provide a different way to complete the integers, leading to unique properties and applications in number theory.

### Hensel's Lemma

Hensel's lemma is a fundamental result in p-adic analysis that allows lifting roots of polynomials from modulo p to modulo p^n, and ultimately to p-adic integers. This library implements a version of Hensel's lemma for finding all p-adic integer roots (Z_p) of a given polynomial.

### Teichmüller Numbers

Teichmüller numbers are the (p-1) distinct roots of the equation x^p - x = 0 in the p-adic integers (Z_p). They form a multiplicative group and are important in p-adic analysis and algebraic number theory. All Teichmüller numbers are units in Z_p.

## Requirements

- Rust 2024 edition or later
- Cargo package manager

## Dependencies

- `num-traits = "0.2.19"` - For numeric traits

## License

This project is provided as-is for educational and research purposes.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Examples

Run the example program to see the library in action:

```bash
cargo run
```

This will demonstrate:
- Basic p-adic integer creation (Z_p only)
- Teichmüller number computation for p=5 in Z_p
- Finding p-adic integer roots of X^3 - 2 using Hensel's lemma in Z_p

**Note: All examples work exclusively with p-adic integers (Z_p). The library does not support the full p-adic field Q_p.**
