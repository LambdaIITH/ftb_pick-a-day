# Pick a Day

Kartheek is organizing Fix The Bug this year, and he needs to pick a day for the event. Since he is
extremely indecisive, he decides to generate a random weekday to hold the event, using the `rand`
and `time` crates in Rust. But he's stuck with a compiler error. The compiler is telling him that
the `Standard` trait is not implemented, even though it [clearly
is](https://docs.rs/time/latest/time/enum.Weekday.html#impl-Distribution%3CWeekday%3E-for-Standard).
Can you help him fix the bug?

### Expected Output

```
How about we conduct Fix The Bug on Tuesday?
```

### Current Output

No output, compiler error :(
