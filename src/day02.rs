use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use lazy_static::lazy_static;
use regex::Regex;

struct Policy {
  min: u32,
  max: u32,
  character: char,
}

fn parse_policy(raw_policy: &str) -> Policy {
  lazy_static! { // use lazy_static to compile the regex only once
    static ref PARSER: Regex =
      Regex::new(r"(?P<min>\d+)\-(?P<max>\d+)\s(?P<character>\w)").unwrap();
  }

  let captured = PARSER.captures(raw_policy).unwrap();
  let min = captured.name("min").unwrap().as_str().parse().unwrap();
  let max = captured.name("max").unwrap().as_str().parse().unwrap();
  let character = captured
    .name("character")
    .unwrap()
    .as_str()
    .chars()
    .next()
    .unwrap();

  return Policy {
    min: min,
    max: max,
    character: character,
  };
}

fn build_validator_regex(raw_policy: &str) -> Box<dyn Fn(&str) -> bool> {
  // parse the policy to get its config
  let policy = parse_policy(raw_policy);

  // build a function from that config
  return Box::new(move |text| {
    let amount = text
      .chars()
      .filter(|&x| x == policy.character)
      .collect::<String>()
      .len() as u32;
    amount >= policy.min && amount <= policy.max
  });
}

pub fn is_valid_password(policy: &str, password: &str) -> bool {
  let validator = build_validator_regex(policy);
  return validator(password);
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<(String, String)> {
  input
    .lines()
    .map(|line| {
      let entry: Vec<&str> = line.split(':').map(|x| x.trim()).collect();
      return (entry[0].to_string(), entry[1].to_string());
    })
    .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(entries: &[(String, String)]) -> u32 {
  entries.iter().fold(0, |total, entry| {
    let result = if is_valid_password(&entry.0, &entry.1) {
      1
    } else {
      0
    };
    total + result
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_valid_password() {
    assert_eq!(is_valid_password("1-3 a", "abcde"), true);
    assert_eq!(is_valid_password("1-3 b", "cdefg"), false);
    assert_eq!(is_valid_password("2-9 c", "ccccccccc"), true);
    assert_eq!(is_valid_password("2-2 a", "fafa"), true);
  }

  #[test]
  fn test_solve_part1() {
    let entries: [(String, String); 3] = [
      ("1-3 a".to_string(), "abcde".to_string()),
      ("1-3 b".to_string(), "cdefg".to_string()),
      ("2-9 c".to_string(), "ccccccccc".to_string()),
    ];
    assert_eq!(solve_part1(&entries), 2);
  }
}
