# Day 3 – Variables, Mutability, and Data Types in Rust

**Date:** 02 January 2026  
**Time:** 22:08

---

## Variables and Mutability

By default, variables in Rust are **immutable**.

To make a variable mutable, use the `mut` keyword:

```rust
let mut x = 5;
x = 6;
```

---

## Constants

- Constants are **globally immutable**
- Their values **cannot change at any point in time**
- Declared using the `const` keyword (not `let`)
- Can be declared in **any scope**, including global scope
- Must be assigned **constant expressions**, not runtime-calculated values

### Example

```rust
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
```

### Use Cases

- Naming **hardcoded values**
- Improving code readability and maintainability

---

## Shadowing

Shadowing allows redeclaring a variable with the **same name** using `let`.

- The new variable **overrides** the previous one
- The shadowed variable remains hidden until:
  - It is shadowed again, or
  - The scope ends

### Key Benefits

- Perform transformations while keeping variables immutable
- Allows **changing data types** during reassignment

```rust
let x = 5;
let x = x + 1;
let x = "six";
```

---

## Data Types in Rust

Rust is a **statically typed** language.

### Type Categories

- **Scalar**
- **Compound**

If Rust cannot infer the type, explicit **type annotation** is required.

```rust
let guess: u32 = "42".parse().expect("Not a number");
```

---

## Scalar Data Types

Scalar types represent a **single value**.

1. Integer  
2. Floating-point  
3. Boolean  
4. Character

---

## Integer Types

Integers are numbers **without fractional parts**.

- `i` → signed integers  
- `u` → unsigned integers

### Integer Sizes

| Length | Signed | Unsigned |
|------|-------|----------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| Arch-dependent | isize | usize |

- Signed integers use **two’s complement**
- `isize` and `usize` depend on system architecture

---

## Integer Literals

| Literal Type | Example |
|-------------|---------|
| Decimal | `98_222` |
| Hexadecimal | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte | `b'A'` |

Default integer type: `i32`

---

## Integer Overflow

- **Debug mode:** panic
- **Release mode:** wrapping

### Handling Methods

- `wrapping_*`
- `checked_*`
- `overflowing_*`
- `saturating_*`

---

## Floating-Point Types

- `f32`
- `f64` (default)

Stored using **IEEE-754** standard.

---

## Numeric Operations

- `+` Addition  
- `-` Subtraction  
- `*` Multiplication  
- `/` Division  
- `%` Remainder

---

## Boolean Type

- `true`
- `false`

---

## Character Type

- Type: `char`
- Size: **4 bytes**
- Supports **Unicode scalar values**

Range:
```
U+0000 to U+D7FF
U+E000 to U+10FFFF
```

---

## Compound Data Types

- Tuple
- Array

---

## Tuple

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

```rust
let (x, y, z) = tup;
```

```rust
let first = tup.0;
```

---

## Arrays

- Same type elements
- Fixed length
- Stored on stack

```rust
let arr: [i32; 5] = [29, 12, 16, 11, 69];
```

```rust
let a = [3; 5];
```

```rust
let value = arr[0];
```

> For dynamic size, use `Vec<T>`

---

## End of Day 3 Notes

