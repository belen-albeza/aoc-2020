use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

fn is_valid_number(value: u64, list: &[u64]) -> bool {
  for i in 0..list.len() {
    for j in i + 1..list.len() {
      if list[i] != list[j] && list[i] + list[j] == value {
        return true;
      }
    }
  }

  false
}

fn find_first_invalid_number(list: &[u64], preamble_length: usize) -> Result<(u64, usize), &str> {
  for i in preamble_length..list.len() {
    if !is_valid_number(list[i], &list[i - preamble_length..i]) {
      return Ok((list[i], i));
    }
  }

  Err("Could not find an invalid number")
}

fn find_summands_for_number(target: u64, list: &[u64]) -> Result<&[u64], &str> {
  for i in 0..list.len() {
    for j in i + 1..list.len() {
      let sum: u64 = list[i..j].iter().sum();
      if sum == target {
        return Ok(&list[i..j]);
      }
    }
  }

  Err("Could not find a contiguous set of summands")
}

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<u64> {
  input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(list: &[u64]) -> Result<u64, &str> {
  const PREAMBLE: usize = 25;
  Ok(find_first_invalid_number(&list, PREAMBLE)?.0)
}

#[aoc(day9, part2)]
pub fn solve_part2(list: &[u64]) -> Result<u64, &str> {
  const PREAMBLE: usize = 25;

  let (target, index) = find_first_invalid_number(&list, PREAMBLE)?;
  let summands = find_summands_for_number(target, &list[..index])?;

  Ok(summands.iter().max().unwrap() + summands.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_valid_number() {
    let list = [35, 20, 15, 25, 47];
    assert_eq!(is_valid_number(40, &list), true);
    assert_eq!(is_valid_number(50, &list), true);
    assert_eq!(is_valid_number(102, &list), false);
    assert_eq!(is_valid_number(10, &list), false);
  }

  #[test]
  fn test_find_first_invalid_number() {
    let list = vec![
      35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    assert_eq!(find_first_invalid_number(&list, 5), Ok((127, 14)));
  }

  #[test]
  fn test_find_summands_for_number() {
    let list = vec![
      35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127,
    ];

    let expected: &[u64] = &[15, 25, 47, 40];

    assert_eq!(find_summands_for_number(127, &list), Ok(expected));
  }

  #[test]
  fn test_parse_input() {
    let input = "35\n20\n15\n25\n47\n40";
    assert_eq!(parse_input(input), vec![35, 20, 15, 25, 47, 40]);
  }
}
