# Pick a Day

Kartheek is organizing Fix The Bug this year, and he needs to pick a day for the event. Given his
extreme indecisiveness, he instead decides to generate a random weekday to hold the event, using
the `rand` and `time` crates in Rust. But he's stuck with a compiler error. The compiler is telling
him that the `Standard` trait is not implemented, even though it [clearly is](https://docs.rs/time/latest/time/enum.Weekday.html#impl-Distribution%3CWeekday%3E-for-Standard).
Can you help him fix the bug?

Note: The goal here is to figure out why the compiler is complaining, and fix it. Don't completely
change the code to use a different method.

### Getting Started

This repository contains a Rust project, so make sure you have `rustc` and `cargo` installed on your
machine. If you don't, you can install them using [rustup](https://rustup.rs/), the official Rust
installer.

1. Clone this repository and `cd` into it.
2. Run the program using `cargo run`.

### Expected Output

```
How about we conduct Fix The Bug on Tuesday?
```

(`Tuesday` is just an example, the output should be a random day of the week.)

### Current Output

No output, compiler error :(
