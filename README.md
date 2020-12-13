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

### Day 2

I learned a bunch of Rust with today's puzzle:

- How to create regexes with named capture groups.
- Use the `lazy_static!` macro to compute a value only once
- How to return a closure

I used a regex to parse the policy and extract its config:

```rust
lazy_static! {
  static ref PARSER: Regex =
    Regex::new(r"(?P<num1>\d+)\-(?P<num2>\d+)\s(?P<character>\w)").unwrap();
}
```

And then I have two functions (one per password policy) that return a closure that performs the password validation. For instance, this is the closure returned for the first policy:

```rust
Box::new(move |text| {
  let amount = text
    .chars()
    .filter(|&x| x == policy.character)
    .collect::<String>()
    .len() as u32;
  amount >= policy.num1 && amount <= policy.num2
})
```

### Day 3

I took this as an opportunity to learn how to implement a custom iterator in Rust.

```rust
impl SlopeIterator {
  fn new(slope: (usize, usize), map: &Map) -> Self {
    SlopeIterator {
      slope: slope,
      current: (0, 0),
      limit: (usize::MAX, map.height - 1),
    }
  }
}

impl Iterator for SlopeIterator {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<(usize, usize)> {
    if self.current.0 > self.limit.0 || self.current.1 > self.limit.1 {
      return None;
    }

    let res = self.current;
    self.current.0 += self.slope.0;
    self.current.1 += self.slope.1;

    return Some(res);
  }
}
```

With that iterator, traversing the map to look for cells with trees it's quite simple:

```rust
iterator.fold(0, |total, (x, y)| {
  let inc = match map.get_cell(x, y) {
    Some(Cell::Tree) => 1,
    _ => 0,
  };

  total + inc
})
```

I had problems with **part 2**, because it asks to multiply partial results and supply that result as the answer for the puzzle, but I was using `u32` and there was an overflow I wasn't noticing… I got the real result once I switched to `u64`.

### Day 4

Today's puzzles were simple, but tedious. I'm thankful for how easy is to add and run unit tests in Rust.

I decided to use "new" (for me in Rust!) data structures: `HashMap` to hold the passport information, and `HashSet` for the required fields. This way, it's easy to see which fields are missing:

```rust
let required_fields: HashSet<String> = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt"]
    .iter()
    .map(|x| x.to_string())
    .collect();

let passport_fields: HashSet<String> = passport.keys().cloned().collect();
let missing: Vec<&String> = required_fields.difference(&passport_fields).collect();
```

For part 2, I ended up creating separate functions to validate each data type –so I could unit test them separately. Once that's in place, checking whether a field is valid with `match` is quite readable:

```rust
let is_valid: bool = match field.as_str() {
  "byr" => is_valid_number(value, 1920, 2002),
  "iyr" => is_valid_number(value, 2010, 2020),
  "eyr" => is_valid_number(value, 2020, 2030),
  "hgt" => is_valid_height(value),
  "hcl" => is_valid_hex_number(value),
  "ecl" => is_valid_eye_color(value),
  "pid" => is_valid_passport_id(value),
  "cid" => true,
  _ => false,
};
```

### Day 5

Today's been more fun that yesterday. I created a recursive function to do this partition of left/front vs right/back, and got to play with ranges in Rust:

```rust
pub fn locate(range: Range<usize>, locator: &[Dir]) -> usize {
  let length = range.len();

  if length > 2 {
    let new_range = match locator[0] {
      Dir::Lower => range.start..(range.start + length / 2),
      Dir::Higher => (range.start + length / 2)..range.end,
    };
    locate(new_range, &locator[1..])
  } else {
    match locator[0] {
      Dir::Lower => range.start,
      Dir::Higher => range.end - 1, // -1 because range.end is not inclusive
    }
  }
}
```

For the second part of the puzzle, the hardest bit was to understand the wording of the problem 😅

### Day 6

Today's puzzle was perfect for practicing with `HashSet`s. Part 1 was quite easy, I just had to build a list of sets with the answers of each group, removing any repitition (which is done automatically in sets):

```rust
type Group = HashSet<char>;

// ...
#[aoc_generator(day6, part1)]
pub fn parse_input_part1(input: &str) -> Vec<Group> {
  input
    .split("\n\n")
    .map(|raw_group| {
      raw_group
        .lines()
        .collect::<Vec<&str>>()
        .concat()
        .chars()
        .collect()
    })
    .collect()
}
```

Part 2 was more challenging in terms of references, copying, etc. This solution is not performant memory-wise (since a new set is created for each intersection operation), but it works:

```rust
input
  .split("\n\n")
  .map(|raw_group| {
    let answers: Vec<Group> = raw_group
      .lines()
      .collect::<Vec<&str>>()
      .iter()
      .map(|person_answers| person_answers.chars().collect::<Group>())
      .collect();

    answers.iter().fold(answers[0].clone(), |result, partial| {
      result.intersection(&partial).copied().collect::<Group>()
    })
  })
  .collect()`
```

### Day 7

So this one was frustrating, because what I chose to do for part 1 was not the proper approach to handle part 2… Initially I had a ruleset in the form of a `HashMap`, whose keys were the _contained_ bag, and the values where the _container_ bags.

```rust
type Ruleset = HashMap<String, Vec<String>>;

fn find_containers(ruleset: &Ruleset, targets: &[String]) -> HashSet<String> {
  let mut result: HashSet<String> = HashSet::new();
  for target in targets.iter() {
    if let Some(containers) = ruleset.get(&target.to_string()) {
      result.extend(containers.iter().cloned());
      result.extend(find_containers(ruleset, containers));
    }
  }
  result
}
```

For part 2, not only we had to take into account how many other bags a bag could hold, but it required a top-bottom approach (i.e. start from one bag, and go through all of its contents). So I changed the ruleset for a new `HashMap`, this time with the _containers_ as keys, and their contents as `(amount, bag_color)` pairs.

Since today I had a day off at work, and I had the time, I ended up refactoring part 1 code so it could use this same data structure:

```rust
fn find_containers(ruleset: &Ruleset, target: &str) -> HashSet<String> {
  let mut result: HashSet<String> = HashSet::new();

  for (outer, inner) in ruleset.iter() {
    if inner.iter().find(|(_, color)| color == target).is_some() {
      result.insert(outer.to_string());
      result.extend(find_containers(ruleset, outer))
    }
  }

  result
}

fn find_amount_contained(ruleset: &Ruleset, target: &str) -> u32 {
  let mut result: u32 = 0;

  if let Some(contained) = ruleset.get(target) {
    for (amount, bag) in contained {
      result += amount + amount * find_amount_contained(ruleset, bag);
    }
  }

  result
}
```

### Day 8

Today was fun, since I love the topic of VM's, compilers, etc. I'm doing this challenge within a group of other developers so we can compare solutions, help each other, etc. and somebody suggested to make today's puzzle "nice and clean" because we might want to re-use it for other puzzles –apparently this happened at a previous year.

Since today was a public holiday, I put in the time to use proper types/structures and I also introduced some error handling, which may come useful if we end up having to reuse this code for other puzzles.

My main types are:

```rust
pub enum Opcode {
  Accumulate,
  Jump,
  NoOp,
}

pub type Instruction = (Opcode, i64);

struct Machine {
  program: Vec<Instruction>,
  ip: usize,
  accumulator: i64,
  ip_run: HashSet<usize>,
}
```

The `ip_run` set is mean to track which program instructions we have visited already, so we can detect loops, which was the point of both part 1 and part 2.

The method that runs the whole program until we reach the end (returns `Ok()`) or we detect a loop (returns a custom `InfiniteLoop` error) is this one:

```rust
pub fn run(&mut self) -> Result<(), MachineError> {
  // loop thorugh program until the end
  while self.ip < self.program.len() {
    // return error if we detect a loop in the program
    if self.ip_run.contains(&self.ip) {
      return Err(MachineError::InifiteLoop);
    }
    self.ip_run.insert(self.ip);
    self.step()?;
  }

  Ok(())
}
```

With that in place, solving part 1 is straight-forward. For part 2, I opted for a brute-force approach, iterating through all the instructions, finding lines that could be patched, and try out that patched program to see if it fixed the infinite loop:

```rust
for (i, (opcode, arg)) in program.iter().enumerate() {
  // ...
  if let Some(patch) = instruction {
    let mut machine = Machine::new(&[&program[..i], &[patch], &program[i + 1..]].concat());
    let result = machine.run();
    if result.is_ok() {
      return Ok(machine.accumulator);
    }
  }
}
```

### Day 9

This puzzle was quick and easy, and I could solve without doing any optimizations at all –not even checking in part 2 if the partial sum was still over target to try a different set.

```rust
fn find_summands_for_number(target: u64, list: &[u64]) -> Result<&[u64], &str> {
  for i in 0..list.len() {
    for j in i + 1..list.len() {
      let sum = list[i..j].iter().sum::<u64>();
      if sum == target {
        return Ok(&list[i..j]);
      }
    }
  }

  Err("Could not find a contiguous set of summands")
}
```

### Day 10

Today was a pathfinding problem. In part 1, we already got a path given (the one that used all of the nodes) and just needed to perform a calculation on that one.

In part 2, we needed to find _how many paths_ we could build with the given input. I had problems with this part, since I initially coded a function that returned the paths themselves, and not the amount of paths available.

When it came to memoize one to save calculations (needed because the result was in the order of trillions), it got stuck. I thought it was a problem in my logic, but it came to the amount of memory needed! I ended up changing it for a function that returned the amount of paths instead, and memoizing that made the trick.

In the end it doesn't look that bad:

```rust
fn find_n_paths(list: &[u64], cache: &mut Cache) -> u64 {
  if list.len() <= 1 {
    return list.len() as u64;
  }

  if let Some(path_count) = cache.get(list) {
    return *path_count;
  }

  let mut result: u64 = 0;
  for i in 1..list.len() {
    if list[i] - list[0] <= 3 {
      result += find_n_paths(&list[i..], cache);
    }
  }

  cache.insert(list.to_vec(), result);
  result
}
```

### Day 11

Today was easier than yesterday – no optimization was needed. It was very similar to the [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

I feel my code ended up too verbose. I opted to have an enum to model whether a cell in the map grid was a empty/occupied seat, or floor, plus I stored the whole map in a single, 1-dimensional vector.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
  Seat(bool), // Seat(is_occupied)
  Floor,
}

pub struct Grid {
  cells: Vec<Cell>,
  previous_cells: Vec<Cell>,
  width: usize,
  height: usize,
}
```

I then implemented a `step` method for the `Grid` struct, that mutates the grid into the next tick. The `Grid` also stores the cells as they were in the previous steps, so checking if it didn't change at all after a step was possible.

### Day 12

I wanted to avoid using floats and `sin`, `cos`, etc. to avoid any kind of rounding errors, and opted to work with integers instead. Luckily, all angles were provided in steps of `±90º`, so rotation was simplified a lot.

This is the method to rotate the `Ship`, which takes as origin its own center. This is simple since we only need to calculate which new direction the `Ship` will be facing:

```rust
fn rotate(&mut self, angle: i64) {
  const DIRS: [Dir; 4] = [Dir::East, Dir::South, Dir::West, Dir::North];
  let steps = angle / 90; // we only allow +-90 angle increments

  let current = DIRS.iter().position(|&x| x == self.facing).unwrap() as i64;
  let index = (steps + current).rem_euclid(4);

  self.facing = DIRS[index as usize];
}
```

For waypoints, they rotate around the `Ship`, so we don't need to calculate a new direction, but a new set of coordinates:

```rust
  pub fn rotate(&mut self, angle: i64) {
    let angle = angle.rem_euclid(360); // clamp the angle to 360º

    match angle {
      90 | -270 => self.position = (-self.position.1, self.position.0), // (-y, x)
      -90 | 270 => self.position = (self.position.1, -self.position.0), // (y, -x)
      180 | -180 => self.position = (-self.position.0, -self.position.1), // (-x, -y)
      _ => unreachable!(),
    };
  }
```

### Day 13

Today was rough, because of part 2. Here a simple brute force approach like this one works for smaller list, but not when we start to have more than 4 buses in our list.

```rust
 let mut timestamp = 0;

  loop {
    if buses
      .iter()
      .enumerate()
      .all(|(gap, &bus)| is_valid_timestamp(timestamp, bus, gap as u64))
    {
      break;
    }

    timestamp += 1;
  }
```

For optimization, this is some kind of "prune", in the sense that we can't be checking all potential results, but need a way to discard in advance those numbers that we know are invalid in advance.

I'm sure there must be a nice and simple, mathematical theorem about this, and probably with a formula, but I'm not a major in Maths. It turns out we only need to keep accumulating the buses ID's as multipliers to have a potential valid timestamp for all the buses.

```rust
let mut t = 0;
let mut coeff = 1;
for i in 1..buses.len() {
  let (curr_gap, curr_bus_id) = buses[i];
  let (_, prev_bus_id) = buses[i - 1];

  coeff = coeff * prev_bus_id;

  loop {
    if (t + curr_gap) % curr_bus_id == 0 {
      break;
    }
    t += coeff;
  }
}

Ok(t)
```
