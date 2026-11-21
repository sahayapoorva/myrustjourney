---
title: "Rust Data Types (Part 1): Scalar Types"
date: "2026-11-21"
---

Rust is **statically typed**, which means the compiler must know the type of every value before your program runs. Most of the time Rust figures types out for you (called *type inference*), but whenever more than one type could make sense, it asks you to be explicit instead of guessing:

```rust
let guess = "42".parse().expect("Not a number");      // compile error: type needed
let guess: u32 = "42".parse().expect("Not a number"); // ok: Rust knows you want u32
```

Rust's built-in primitive types come in two groups: **scalar** types (a single value — covered in this post) and **compound** types (multiple values grouped together — covered in [Part 2: Compound Types](/notes/rust/data-types-compound)).

## Integers

An integer is a whole number with no decimal point. Its name tells you two things:
- **`i` (signed)** can hold negative or positive numbers; **`u` (unsigned)** holds only zero and positive numbers.
- The number after it is the **bit width** (how much memory it uses).

| Bits | Signed | Unsigned | Unsigned range |
| :--- | :--- | :--- | :--- |
| 8 | `i8` | `u8` | `0` … `255` |
| 16 | `i16` | `u16` | `0` … `65,535` |
| 32 | `i32` | `u32` | `0` … ~4.3 billion |
| 64 | `i64` | `u64` | `0` … ~1.8 × 10¹⁹ |
| 128 | `i128` | `u128` | `0` … ~3.4 × 10³⁸ |
| arch | `isize` | `usize` | Matches your CPU (64-bit or 32-bit); `usize` is used for lengths and indexes |

If you don't specify a type, whole numbers default to `i32`. You can also write literals in hex (`0xff`), octal (`0o77`), binary (`0b1111_0000`), with `_` for readability (`98_222`), with a type suffix (`57u8`), or as a byte (`b'A'`, which is the `u8` number `65`, not a character).

### Integer quirks to watch out for

- **Signed ranges are lopsided**: Because signed integers use *two's complement* (the standard binary encoding for negative numbers), there is one extra negative value. An `i8` runs from `-128` to `127`, so `(-128i8).abs()` overflows because `+128` does not fit.
- **No implicit conversions**: You cannot mix integer types directly — not even widening a smaller one. Casting with `as` works, but silently wraps or truncates if the value doesn't fit:
  ```rust
  let n: usize = 5;
  let m: i32 = 3;
  // n + m            // compile error: mismatched types
  let sum = n + m as usize; // 8
  let wrapped = 300i32 as u8; // 44 (silently truncated!)
  ```
- **Division truncates toward zero**: `-7 / 2` is `-3` and `-7 % 3` is `-1` (unlike Python's `-4` and `2`). Use `(-7i32).rem_euclid(3)` if you want a non-negative remainder (`2`).
- **Out-of-range literals fail at compile time**: `let a: u8 = 256;` and `let c: u8 = -1;` are both compile errors.

## Integer Overflow

**Overflow** happens when math goes past a type's min or max (like `255u8 + 1`). What Rust does depends on *when* it's caught:

| When detected | What happens |
| :--- | :--- |
| Compile time (constants) | Build fails |
| Runtime, **debug** build | Program panics (stops with an error) |
| Runtime, **release** build | Silently **wraps** around like a car odometer (`255u8 + 1` becomes `0`) |

Because tests usually run in debug mode while production runs in release mode, never rely on default overflow behavior. Instead, pick one of Rust's four explicit method families — they behave identically in every build:

```rust
let a: u8 = 200;
a.checked_sub(250);     // None        — handle overflow safely as an error
a.saturating_sub(250);  // 0           — clamp at the type's min or max (e.g. volume, HP)
a.wrapping_sub(250);    // 206         — wrap around on purpose (e.g. hashes, ring buffers)
a.overflowing_sub(250); // (206, true) — wrapped value plus a bool flag
```

## Floating-Point Numbers

Floats represent numbers with decimals: `f32` (single precision) and `f64` (double precision, the default because it is roughly as fast on modern CPUs and more accurate). Both follow the IEEE-754 standard and are always signed.

- **Integers and floats never mix**: `5 + 2.5` is a compile error; write `5.0 + 2.5`.
- **Decimal math is approximate**: Just like in most languages, `0.1 + 0.2 == 0.3` is `false`. And because `NaN != NaN`, floats don't have a total ordering, so you can't call `.sort()` directly on a `Vec<f64>`.

## Booleans

A `bool` is either `true` or `false` and takes **1 byte** of memory.

Unlike Python or JavaScript, **nothing is "truthy" in Rust** — conditions in `if` or `while` must evaluate to an actual `bool`:

```rust
let n = 1;
// if n { }      // compile error: expected bool, found integer
if n != 0 { }    // ok
```

## Characters

A `char` represents one **Unicode scalar value** (an ASCII letter, accented letter, CJK character, or emoji) and is always **4 bytes**. Always use **single quotes** for a `char` and double quotes for strings:

```rust
let cat = '😻';  // char (4 bytes)
let s = "z";     // &str (UTF-8 string slice, not a char)
```

Two helpful details for newcomers:
- **Strings aren't arrays of `char`**: Rust strings are UTF-8 encoded, so `"abc".len()` is 3 bytes, while `[char; 3]` takes 12 bytes.
- **One `char` isn't always one visible symbol**: A letter like `"é"` can be stored as two `char`s (`e` plus a combining accent), and UTF-16 surrogate code points (`U+D800–U+DFFF`) are rejected by `char`.

## Key Takeaways

- Rust infers types when clear, refuses to guess when ambiguous, and never converts numbers implicitly.
- Always be deliberate about integer overflow with `checked_*`, `saturating_*`, `wrapping_*`, or `overflowing_*` instead of relying on debug vs. release behavior.
- Conditions must be explicit `bool` values, and `'a'` (`char`, 4 bytes) is very different from `"a"` (`&str`, UTF-8).

---

**Series navigation**: Next → [Part 2: Compound Types (Tuples & Arrays)](/notes/rust/data-types-compound)

## References

- [The Rust Programming Language, Ch. 3.2 — Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
