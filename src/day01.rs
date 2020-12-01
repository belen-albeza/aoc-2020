use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn parse_day1(input: &str) -> Vec<u32> {
  return input.lines().map(|x| x.parse().unwrap()).collect();
}

#[aoc(day1, part1)]
pub fn solve_part1(entries: &[u32]) -> u32 {
  const TARGET: u32 = 2020;

  for i in 0..entries.len() {
    for j in (i + 1)..entries.len() {
      let sum = entries[i] + entries[j];
      if sum == TARGET {
        return entries[i] * entries[j];
      }
    }
  }

  unreachable!();
}

#[aoc(day1, part2)]
pub fn solve_part2(entries: &[u32]) -> u32 {
  const TARGET: u32 = 2020;

  for i in 0..entries.len() {
    for j in (i + 1)..entries.len() {
      if entries[i] + entries[j] > TARGET {
        continue;
      }

      for k in (j + 1)..entries.len() {
        let sum = entries[i] + entries[j] + entries[k];
        if sum == TARGET {
          return entries[i] * entries[j] * entries[k];
        }
      }
    }
  }

  unreachable!();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let input: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(solve_part1(&input), 514579);
  }

  #[test]
  fn test_part2() {
    let input: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(solve_part2(&input), 241861950);
  }
}
