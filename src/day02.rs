use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use lazy_static::lazy_static;
use regex::Regex;

struct PolicyConfig {
  num1: u32,
  num2: u32,
  character: char,
}

type Validator = Box<dyn Fn(&str) -> bool>;

fn parse_policy(raw_policy: &str) -> PolicyConfig {
  lazy_static! { // use lazy_static to compile the regex only once
    static ref PARSER: Regex =
      Regex::new(r"(?P<num1>\d+)\-(?P<num2>\d+)\s(?P<character>\w)").unwrap();
  }

  let captured = PARSER.captures(raw_policy).unwrap();
  let num1 = captured.name("num1").unwrap().as_str().parse().unwrap();
  let num2 = captured.name("num2").unwrap().as_str().parse().unwrap();
  let character = captured
    .name("character")
    .unwrap()
    .as_str()
    .chars()
    .next()
    .unwrap();

  return PolicyConfig {
    num1: num1,
    num2: num2,
    character: character,
  };
}

fn build_old_validator(raw_policy: &str) -> Validator {
  // parse the policy to get its config
  let policy = parse_policy(raw_policy);

  // build a function from that config
  return Box::new(move |text| {
    let amount = text
      .chars()
      .filter(|&x| x == policy.character)
      .collect::<String>()
      .len() as u32;
    amount >= policy.num1 && amount <= policy.num2
  });
}

fn build_new_validator(raw_policy: &str) -> Validator {
  // parse the policy to get its config
  let policy = parse_policy(raw_policy);

  // build a function from that config
  return Box::new(move |text| {
    let is_at_1st_pos = text.chars().nth((policy.num1 as usize) - 1).unwrap() == policy.character;
    let is_at_2nd_pos = text.chars().nth((policy.num2 as usize) - 1).unwrap() == policy.character;

    is_at_1st_pos ^ is_at_2nd_pos
  });
}

pub fn is_valid_password(
  policy: &str,
  password: &str,
  validator_builder: fn(&str) -> Validator,
) -> bool {
  let validator = validator_builder(policy);
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
    let result = if is_valid_password(&entry.0, &entry.1, build_old_validator) {
      1
    } else {
      0
    };
    total + result
  })
}

#[aoc(day2, part2)]
pub fn solve_part2(entries: &[(String, String)]) -> u32 {
  entries.iter().fold(0, |total, entry| {
    let result = if is_valid_password(&entry.0, &entry.1, build_new_validator) {
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
  fn test_is_valid_password_with_old_policy() {
    assert_eq!(
      is_valid_password("1-3 a", "abcde", build_old_validator),
      true
    );
    assert_eq!(
      is_valid_password("1-3 b", "cdefg", build_old_validator),
      false
    );
    assert_eq!(
      is_valid_password("2-9 c", "ccccccccc", build_old_validator),
      true
    );
    assert_eq!(
      is_valid_password("2-2 a", "fafa", build_old_validator),
      true
    );
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

  #[test]
  fn test_is_valid_password_with_new_policy() {
    assert_eq!(
      is_valid_password("1-3 a", "abcde", build_new_validator),
      true
    );
    assert_eq!(
      is_valid_password("1-3 b", "cdefg", build_new_validator),
      false
    );
    assert_eq!(
      is_valid_password("2-9 c", "ccccccccc", build_new_validator),
      false
    );
    assert_eq!(
      is_valid_password("1-4 a", "fafa", build_new_validator),
      true
    );
  }

  #[test]
  fn test_solve_part2() {
    let entries: [(String, String); 3] = [
      ("1-3 a".to_string(), "abcde".to_string()),
      ("1-3 b".to_string(), "cdefg".to_string()),
      ("2-9 c".to_string(), "ccccccccc".to_string()),
    ];
    assert_eq!(solve_part2(&entries), 1);
  }
}
