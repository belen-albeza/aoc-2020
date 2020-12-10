use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

type Cache = HashMap<Vec<u64>, u64>;

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

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<u64> {
  let mut list: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();
  list.sort();
  list
}

#[aoc(day10, part1)]
pub fn solve_part1(adapters: &[u64]) -> Result<u64, &str> {
  let mut delta_freqs: [u64; 3] = [0, 0, 1]; // [1 jolt, 2 jolts, 3 jolts] freq
  for i in 0..adapters.len() {
    let last_joltage = if i > 0 { adapters[i - 1] } else { 0 };
    let delta = adapters[i] - last_joltage;

    match delta {
      1..=3 => delta_freqs[(delta - 1) as usize] += 1,
      _ => {
        return Err("Difference between adapters is too high");
      }
    }
  }

  Ok(delta_freqs[0] * delta_freqs[2])
}

#[aoc(day10, part2)]
pub fn solve_part2(adapters: &[u64]) -> Result<u64, &str> {
  let full_list = [
    vec![0],
    adapters.to_vec(),
    vec![adapters[adapters.len() - 1] + 3],
  ]
  .concat();

  let mut cache = Cache::new();
  Ok(find_n_paths(&full_list, &mut cache))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_part1() {
    let mut input = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    input.sort();
    assert_eq!(solve_part1(&input), Ok(7 * 5));
  }

  #[test]
  fn test_find_n_paths() {
    assert_eq!(find_n_paths(&vec![], &mut Cache::new()), 0);
    assert_eq!(find_n_paths(&vec![0], &mut Cache::new()), 1);
    assert_eq!(find_n_paths(&vec![0, 10], &mut Cache::new()), 0);
    assert_eq!(
      find_n_paths(&vec![0, 1, 3, 4, 5, 10, 11], &mut Cache::new()),
      0
    );
    assert_eq!(find_n_paths(&vec![0, 2, 3], &mut Cache::new()), 2);
    assert_eq!(find_n_paths(&vec![0, 1, 2, 3], &mut Cache::new()), 4);
  }

  #[test]
  fn test_solve_part2() {
    let mut input = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    input.sort();
    assert_eq!(solve_part2(&input), Ok(8));

    let mut input_alt = [
      28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17,
      7, 9, 4, 2, 34, 10, 3,
    ];
    input_alt.sort();
    assert_eq!(solve_part2(&input_alt), Ok(19208));
  }
}
