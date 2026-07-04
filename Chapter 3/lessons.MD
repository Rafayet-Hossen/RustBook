# Rust Fundamentals: Variables, Mutability, and Data Types

Welcome to this comprehensive guide on **Chapter 3: Common Programming Concepts** from *The Rust Programming Language* book. This document is a complete reference covering variables, memory allocation strategies, and the data type system in Rust.

---

## 📂 Table of Contents

1. [Variables & Mutability](#1-variables--mutability)
2. [Constants vs. Variables](#2-constants-vs-variables)
3. [The Power of Shadowing](#3-the-power-of-shadowing)
4. [The Rust Type System](#4-the-rust-type-system)
5. [Scalar Data Types](#5-scalar-data-types)
6. [Compound Data Types](#6-compound-data-types)
7. [Memory Safety in Action (Runtime Behavior)](#7-memory-safety-in-action-runtime-behavior)

---

## 1. Variables & Mutability

In Rust, variables are **immutable by default**. This is a core safety feature that guarantees that once a value is bound to a name, it cannot change. This prevents a major category of bugs where state changes unexpectedly.

### ❌ Immutable Code (Triggers Compiler Error)

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // ERROR: cannot assign twice to immutable variable `x`
}
```

### ✅ Opting Into Mutability

To allow modifications, you must explicitly use the `mut` keyword. This signals your intent to both the compiler and future readers of your code.

```rust
fn main() {
    let mut x = 5;
    println!("Initial value: {x}"); // 5

    x = 6; // Allowed
    println!("Mutated value: {x}"); // 6
}
```

---

## 2. Constants vs. Variables

Constants (`const`) look similar to immutable variables, but they have fundamental structural differences:

| Feature | `let` (immutable) | `const` |
|---|---|---|
| `mut` allowed? | Yes (with keyword) | Never — always permanently immutable |
| Type annotation | Optional (inferred) | **Required** |
| Value source | Can be runtime-evaluated | **Compile-time only** (constant expression) |
| Scope | Function/block scoped | Can be declared in **any** scope, including global |

```rust
// Declared at global scope
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Constant value: {THREE_HOURS_IN_SECONDS}");
}
```

---

## 3. The Power of Shadowing

Shadowing allows you to declare a new variable with the exact same name as a previous variable using the `let` keyword again.

**Why use shadowing?**

- **Transforming mutability** — perform transformations on a value but keep the final variable completely immutable.
- **Changing data types** — unlike mutability (which forces you to keep the same data type), shadowing lets you shift types without inventing names like `spaces_str` and `spaces_num`.

```rust
fn main() {
    // 1. Scope shadowing
    let x = 5;
    let x = x + 1; // x is now 6
    {
        let x = x * 2; // shadows outer x within this block
        println!("Inner block x: {x}"); // 12
    }
    println!("Outer block x: {x}"); // 6

    // 2. Data type shifting
    let spaces = "   ";         // Type: &str (string slice)
    let spaces = spaces.len();  // Type: usize (integer)
    println!("Number of spaces: {spaces}"); // 3
}
```

> ⚠️ **Important contrast:** If you try to change types using a mutable variable —
> `let mut spaces = "   "; spaces = spaces.len();` — the compiler throws a
> **mismatched types** error. Mutability changes *values*; shadowing *recreates the variable*.

---

## 4. The Rust Type System

Rust is a **statically typed** language, meaning it must know the exact types of all variables at compile time.

The compiler can often perform **type inference** (guessing the type based on how a value is used), but in ambiguous cases — such as parsing text — explicit type annotations are mandatory.

```rust
let x = 2.0; // compiler infers f64 automatically

// Explicit annotation required because .parse() is generic
let guess: u32 = "42".trim().parse().expect("Not a number!");
```

---

## 5. Scalar Data Types

A scalar type represents a single, atomic value. Rust has four primary scalar types.

### A. Integers

Integers are numbers without a fractional component. They can be **signed** (can hold negative numbers, prefixed with `i`) or **unsigned** (always positive, prefixed with `u`).

| Size | Signed | Unsigned |
|---|---|---|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` (default) | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Arch-dependent | `isize` | `usize` |

> **`isize` / `usize`:** size depends on the machine's architecture (e.g. 64 bits on a 64-bit system). Commonly used for indexing collections or arrays.

**Integer overflow:** if a `u8` (range 0–255) holds `255` and you add `1`:

- In **debug** mode, Rust includes runtime checks that cause the program to **panic**.
- In **release** mode (`--release`), Rust performs **two's complement wrapping** — `255` wraps around to become `0`.

### B. Floating-Point Numbers

Rust has two primitive types for numbers with decimal points:

- **`f32`** — single precision.
- **`f64`** — double precision (default on modern CPUs; roughly the same speed as `f32` but significantly more precise).

```rust
let x = 2.0;      // f64 (default)
let y: f32 = 3.0; // f32
```

### C. The Boolean Type

Booleans represent truth values and are 1 byte in size.

```rust
let t: bool = true;
let f: bool = false;
```

### D. The Character Type (`char`)

A `char` represents a single Unicode Scalar Value.

- Defined using single quotes (`'`) instead of double quotes.
- 4 bytes in size — can store far more than ASCII, including accented letters, Chinese/Japanese/Korean scripts, and emojis.

```rust
let c = 'z';
let z: char = 'ℤ';
let heart_eye_cat = '😻';
```

---

## 6. Compound Data Types

Compound types group multiple values into a single comprehensive type.

### A. The Tuple Type

A tuple groups together values of **different types** into one multi-element structure. Tuples have a **fixed length** — they cannot grow or shrink once declared.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 1. Extracting values via destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // 2. Extracting values via direct dot-notation indexing
    let five_hundred = tup.0;
    let one = tup.2;
}
```

> **The unit type:** a tuple without any elements is written as `()`. It represents an empty value or an empty return type. Expressions implicitly return the unit type if they don't explicitly return another value.

### B. The Array Type

An array groups multiple values where **every element must be the same type**.

- Arrays have a **fixed length** (unlike vectors).
- Array data is allocated explicitly on the **stack** rather than the heap, making access highly optimized and predictable.

```rust
fn main() {
    // Explicit type signature: [Type; Length]
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Initializing an array with identical values: [Value; Count]
    let identical_array = [3; 5]; // evaluates to [3, 3, 3, 3, 3]

    // Accessing elements
    let first = numbers[0];
    let second = numbers[1];
}
```

---

## 7. Memory Safety in Action (Runtime Behavior)

Rust protects you against classic low-level memory vulnerability exploits.

Consider an **array out-of-bounds** error:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index]; // RUNTIME PANIC
    println!("The value of element is: {element}");
}
```

**What happens here?**

- In languages like C or C++, the compiler might allow this and look up the memory block located 10 steps away from the index pointer, fetching unallocated memory or garbage data — leading to security vulnerabilities.
- In Rust, the program immediately **panics** and terminates execution upon index verification failure. It prevents unmapped memory access entirely rather than allowing execution with faulty state.

---
