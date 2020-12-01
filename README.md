# aoc-2020

Advent of Code 2020

## To build and run the project

This project uses [cargo-aoc](https://github.com/gobanos/cargo-aoc). More detailed instructions can be found at that project's [README](https://github.com/gobanos/cargo-aoc/blob/master/README.md) file.

1. Create an account at adventofcode.com
2. Get the value for your session cookie and configure `cargo-aoc`:

```
cargo aoc credentials -s TOKEN
```

3. Build and run the code with:

```
cargo aoc
```

## Log

### Day 1

I've used a nested loop to iterate over the input, with the only optimization of not iterating over pairs of numbers that have been tried before (since addition is commutative):

```rust
for i in 0..entries.len() {
  for j in (i + 1)..entries.len() {
    // ...
  }
```

For **part 2** I added a third nested loop to get the third element to add. We avoid entering into this loop if we know the solution is impossible to reach using that combination (if the first two element already exceed the target).

```rust
for i in 0..entries.len() {
  for j in (i + 1)..entries.len() {
    if entries[i] + entries[j] > TARGET {
      continue;
    }

    for k in (j + 1)..entries.len() {
      // ...
    }
  }
}
```
