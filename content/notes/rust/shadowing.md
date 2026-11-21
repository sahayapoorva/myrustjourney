---
title: "Shadowing in Rust"
date: "2026-11-21"
---

In Rust, **shadowing** lets you declare a new variable with the exact same name as an earlier variable by using `let` again. The new variable *shadows* the previous one until either it gets shadowed again or its scope ends.

## Basic Example & Scope

```rust
fn main() {
    let x = 5;
    let x = x + 1; // x is now 6

    {
        let x = x * 2; // shadows x only inside this inner scope (12)
        println!("Inner scope x: {x}");
    }

    println!("Outer scope x: {x}"); // back to 6
}
```

## Shadowing vs. `mut`

At first shadowing looks similar to making a variable mutable (`let mut`), but there are two important differences:

1. **Immutability after transformation**: You can transform a value across a few steps using `let`, and once you're done, the variable stays immutable by default.
2. **Changing the type**: Because shadowing creates a brand-new variable, you can reuse the same name with a different type — something `mut` does not allow.

```rust
let spaces = "   ";       // &str
let spaces = spaces.len(); // usize — works with shadowing!
```

Trying the same thing with `let mut spaces = "   "; spaces = spaces.len();` fails to compile because you cannot mutate a variable's type.

## Key Takeaway

Use shadowing when you want to transform a value (or change its type) while keeping the same meaningful variable name — without leaving the variable permanently mutable.
