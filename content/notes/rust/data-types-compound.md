---
title: "Rust Data Types (Part 2): Compound Types"
date: "2026-11-21"
---

In [Part 1: Scalar Types](/notes/rust/data-types), we looked at types that hold a single value. **Compound types** group multiple values into one. Rust has two primitive compound types — **tuples** and **arrays** — and both have a **fixed length** baked into their type, so neither can grow or shrink after creation.

## Tuples

A **tuple** bundles a fixed number of values that **can have different types**. It's a quick way to group related values (like returning two results from a function) without creating a custom struct.

You can read values out by **destructuring** (unpacking into variables) or by dot-index (`tup.0`, `tup.1`, …):

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup; // destructuring into x, y, z
let first = tup.0;   // 500 (zero-based index)
```

### The unit type `()`

An empty tuple `()` is called **unit**. It represents "no meaningful value" — functions and blocks that don't return anything else automatically return `()`.

### Tuple quirks to watch out for

- **The index must be a literal number**: Because each position can hold a different type, the compiler has to know *which* field you mean at compile time. `tup.1` works (and chains like `((1, 2), 3.0).0.1` work), but `let i = 1; tup.i` is a compile error.
- **Destructuring must match the exact length**: `let (x, y) = tup;` fails on a 3-element tuple. Use `_` to ignore values you don't need: `let (x, y, _) = tup;`.
- **A stray semicolon turns a value into `()`**: A block evaluates to its final expression, *unless* you put a semicolon after it:
  ```rust
  let a = { let n = 5; n + 1 };  // a: i32 = 6
  let b = { let n = 5; n + 1; }; // b: ()  — the semicolon turned it into a statement!
  ```

## Arrays

An **array** holds a fixed number of values that **all share the exact same type**, stored side-by-side on the **stack** (fast, fixed-size memory). Its type includes both the element type and the length, written `[T; N]`.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let repeated = [3; 5]; // shorthand for [3, 3, 3, 3, 3]
let first = a[0];      // 1 (zero-based indexing)
```

Use an array when the count never changes (like the 12 months of the year). When you need a list that can grow or shrink, use `Vec` (a growable vector stored on the heap).

### Array quirks to watch out for

- **Length is part of the type**: `[i32; 3]` and `[i32; 5]` are completely different types, and every element must share one type (`[1, 2.0, 3]` fails to compile).
- **Indexes must be `usize`**: You cannot index an array with an `i32` (`a[i]` fails if `i: i32`). Casting `i as usize` works, but remember a negative number wraps into a huge `usize`!
- **Bounds are checked at runtime**: If an index comes from user input and goes out of bounds, Rust **panics** safely instead of reading garbage memory like C. If you want to avoid panicking, call `.get(index)`, which returns an `Option` (`Some(&value)` if valid, or `None` if out of bounds):
  ```rust
  let safe = a.get(10); // None — no crash
  // let crash = a[index]; // panics at runtime if index >= 5
  ```
- **Pattern matching must match length**: `let [x, y] = a;` fails on a 5-element array unless you add a rest pattern (`..`) to ignore the remaining elements: `let [x, y, ..] = a;`.

## Tuple vs. Array at a Glance

| Feature | Tuple | Array |
| :--- | :--- | :--- |
| Element types | Can differ | Must all be the same |
| Type syntax | `(i32, f64, u8)` | `[i32; 5]` |
| Access | `t.0` (compile-time literal only) | `a[i]` (runtime `usize` index) |
| Bad index | Compile error | Runtime panic (or `None` with `.get()`) |
| Best for | Grouping a few mixed values | Fixed-size collections of one type |

## Key Takeaways

- Because a tuple's fields can have different types, its index must be known at compile time (`t.0`). Because an array's elements share one type, it can be indexed at runtime (`a[i]`) with automatic bounds checking.
- Watch out for stray semicolons at the end of blocks — they turn the block's result into unit `()`.
- Reach for `.get(index)` on arrays when an out-of-range index is possible and you want `None` instead of a panic.

---

**Series navigation**: ← Previous: [Part 1: Scalar Types](/notes/rust/data-types)

## References

- [The Rust Programming Language, Ch. 3.2 — Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
