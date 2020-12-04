use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Passport = HashMap<String, String>;

fn parse_passport(input: &str) -> Passport {
  input
    .split_whitespace()
    .map(|pair| {
      let splitted: Vec<&str> = pair.split(":").collect();
      (splitted[0].to_string(), splitted[1].to_string())
    })
    .collect()
}

pub fn has_required_fields(passport: &Passport) -> bool {
  let required_fields: HashSet<String> = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt"]
    .iter()
    .map(|x| x.to_string())
    .collect();

  let passport_fields: HashSet<String> = passport.keys().cloned().collect();
  let missing: Vec<&String> = required_fields.difference(&passport_fields).collect();

  // no missing fields, or only missing "cid"
  missing.len() == 0 || (missing.len() == 1 && missing[0] == "cid")
}

fn is_valid_number(value: &str, min: u32, max: u32) -> bool {
  let x: u32 = value.parse().unwrap();
  x >= min && x <= max
}

fn is_valid_hex_number(value: &str) -> bool {
  lazy_static! { // use lazy_static to compile the regex only once
    static ref RE: Regex =
      Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  }

  RE.is_match(value)
}

fn is_valid_height(value: &str) -> bool {
  lazy_static! { // use lazy_static to compile the regex only once
    static ref RE: Regex =
      Regex::new(r"^(?P<height>\d+)(?P<unit>cm|in)$").unwrap();
  }

  let matched = RE.captures(value);
  if matched.is_none() {
    return false;
  }
  let captured = matched.unwrap();

  let unit = captured.name("unit").unwrap().as_str();
  let height = captured.name("height").unwrap().as_str();

  match unit {
    "cm" => is_valid_number(height, 150, 193),
    "in" => is_valid_number(height, 59, 76),
    _ => unreachable!(),
  }
}

fn is_valid_eye_color(value: &str) -> bool {
  let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
  valid_colors.iter().find(|&&x| value == x).is_some()
}

fn is_valid_passport_id(value: &str) -> bool {
  lazy_static! { // use lazy_static to compile the regex only once
    static ref RE: Regex =
      Regex::new(r"^\d{9}$").unwrap();
  }

  RE.is_match(value)
}

pub fn is_valid_passport(passport: &Passport) -> bool {
  if !has_required_fields(passport) {
    return false;
  }

  for (field, value) in passport {
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

    if !is_valid {
      return false;
    }
  }

  return true;
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Passport> {
  input.split("\n\n").map(parse_passport).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(passports: &[Passport]) -> usize {
  passports.iter().fold(0, |total, passport| {
    total + if has_required_fields(&passport) { 1 } else { 0 }
  })
}

#[aoc(day4, part2)]
pub fn solve_part2(passports: &[Passport]) -> usize {
  passports.iter().fold(0, |total, passport| {
    total + if is_valid_passport(&passport) { 1 } else { 0 }
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_passport() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
    let passport = parse_passport(input);
    let expected: Passport = [
      (String::from("ecl"), String::from("gry")),
      (String::from("pid"), String::from("860033327")),
      (String::from("eyr"), String::from("2020")),
      (String::from("hcl"), String::from("#fffffd")),
      (String::from("byr"), String::from("1937")),
      (String::from("iyr"), String::from("2017")),
      (String::from("cid"), String::from("147")),
      (String::from("hgt"), String::from("183cm")),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(passport, expected);
  }

  #[test]
  fn test_has_required_fields() {
    // test a password with all fields
    let complete = parse_passport(
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
    );
    assert_eq!(has_required_fields(&complete), true);

    // test a passport with missing fields
    let incomplete =
      parse_passport("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929");
    assert_eq!(has_required_fields(&incomplete), false);

    // test a north pole passport
    let complete_northpole =
      parse_passport("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm");
    assert_eq!(has_required_fields(&complete_northpole), true);

    // test a north pole passport with missing fields
    let incomplete_northpole =
      parse_passport("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in");
    assert_eq!(has_required_fields(&incomplete_northpole), false);
  }

  #[test]
  fn test_is_valid_passport() {
    let valid_passports = [
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
      "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
      "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
      "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    ]
    .iter()
    .map(|x| parse_passport(x));

    for passport in valid_passports {
      assert_eq!(is_valid_passport(&passport), true)
    }

    let invalid_passports = [
      "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
      "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
      "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
      "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
    ]
    .iter()
    .map(|x| parse_passport(x));

    for passport in invalid_passports {
      assert_eq!(is_valid_passport(&passport), false)
    }
  }

  #[test]
  fn test_is_valid_height() {
    assert_eq!(is_valid_height("60in"), true);
    assert_eq!(is_valid_height("190cm"), true);
    assert_eq!(is_valid_height("190in"), false);
    assert_eq!(is_valid_height("190"), false);
  }

  #[test]
  fn test_is_valid_eye_color() {
    for color in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter() {
      assert_eq!(is_valid_eye_color(color), true);
    }
    for color in ["black", "aaa", "ccc"].iter() {
      assert_eq!(is_valid_eye_color(color), false);
    }
  }

  #[test]
  fn test_is_valid_hex_number() {
    assert_eq!(is_valid_hex_number("#123abc"), true);
    assert_eq!(is_valid_hex_number("#123abz"), false);
    assert_eq!(is_valid_hex_number("123abc"), false);
    assert_eq!(is_valid_hex_number("#123"), false);
  }

  #[test]
  fn test_is_valid_passport_id() {
    assert_eq!(is_valid_passport_id("012"), false);
    assert_eq!(is_valid_passport_id("012345678"), true);
    assert_eq!(is_valid_passport_id("0123456789"), false);
  }

  #[test]
  fn test_solve_part1() {
    let passports = [
      parse_passport(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
      ),
      parse_passport("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"),
      parse_passport("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm"),
      parse_passport("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"),
    ];

    assert_eq!(solve_part1(&passports), 2);
  }

  #[test]
  fn test_solve_part2() {
    let passports: Vec<Passport> = [
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
      "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
      "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
      "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
      "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
      "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
      "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
      "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
    ]
    .iter()
    .map(|x| parse_passport(x))
    .collect();

    assert_eq!(solve_part2(&passports), 4);
  }
}
