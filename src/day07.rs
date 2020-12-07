use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Ruleset = HashMap<String, Vec<String>>;
type ExtendedRuleset = HashMap<String, Vec<(u32, String)>>;

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

fn find_amount_contained(ruleset: &ExtendedRuleset, target: &str) -> u32 {
  let mut result: u32 = 0;

  if let Some(contained) = ruleset.get(target) {
    result += contained
      .iter()
      .fold(0, |total, (amount, _)| total + amount);
    for (amount, bag) in contained {
      result += amount * find_amount_contained(ruleset, bag);
    }
  }

  result
}

fn parse_extended_ruleset(input: &str) -> ExtendedRuleset {
  let mut ruleset = ExtendedRuleset::new();

  for line in input.lines() {
    let parts: Vec<&str> = line
      .trim_end_matches(".")
      .split("contain")
      .map(|x| x.trim())
      .collect();
    let inner: Vec<(u32, String)> = parts[1]
      .split(", ")
      .map(parse_bag_spec)
      .filter(|x| x.is_some())
      .map(|x| x.unwrap())
      .collect();
    let outer = parse_bag_spec(parts[0]).unwrap().1;

    ruleset.insert(outer, inner);
  }

  ruleset
}

fn parse_bag_spec(input: &str) -> Option<(u32, String)> {
  lazy_static! { // use lazy_static to compile the regex only once
    static ref RE: Regex =
      Regex::new(r"((?P<amount>\d+) )?(?P<color>\w+ \w+) bags?$").unwrap();
  }

  if input != "no other bags" {
    let captured = RE.captures(input)?;
    let amount = captured.name("amount");
    Some((
      if amount.is_some() {
        amount.unwrap().as_str().parse().unwrap()
      } else {
        0
      },
      captured.name("color")?.as_str().to_string(),
    ))
  } else {
    None
  }
}

fn parse_ruleset(input: &str) -> Ruleset {
  let mut ruleset = Ruleset::new();

  for line in input.lines() {
    let parts: Vec<&str> = line
      .trim_end_matches(".")
      .split("contain")
      .map(|x| x.trim())
      .collect();
    let inner = parts[1]
      .split(", ")
      .map(parse_bag_spec)
      .filter(|x| x.is_some())
      .map(|x| x.unwrap());

    for bag in inner {
      let outer = parse_bag_spec(parts[0]).unwrap().1;
      if let Some(containers) = ruleset.get_mut(&bag.1) {
        containers.extend(vec![outer.to_string()]);
      } else {
        ruleset.insert(bag.1, vec![outer.to_string()]);
      }
    }
  }

  ruleset
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> (Ruleset, ExtendedRuleset) {
  (parse_ruleset(input), parse_extended_ruleset(input))
}

#[aoc(day7, part1)]
pub fn solve_part1(rulesets: &(Ruleset, ExtendedRuleset)) -> u32 {
  let target = String::from("shiny gold");
  find_containers(&rulesets.0, &vec![target]).len() as u32
}

#[aoc(day7, part2)]
pub fn solve_part2(ruleset: &(Ruleset, ExtendedRuleset)) -> u32 {
  find_amount_contained(&ruleset.1, "shiny gold")
}

#[cfg(test)]
mod tests {
  use super::*;

  const WHITE: &str = "bright white";
  const RED: &str = "light red";
  const ORANGE: &str = "dark orange";
  const YELLOW: &str = "muted yellow";
  const GOLD: &str = "shiny gold";
  const BLUE: &str = "faded blue";
  const OLIVE: &str = "dark olive";
  const PLUM: &str = "vibrant plum";
  const BLACK: &str = "dotted black";

  #[test]
  fn test_parse_bag_spec() {
    assert_eq!(
      parse_bag_spec("1 shiny gold bag"),
      Some((1, "shiny gold".to_string()))
    );
    assert_eq!(
      parse_bag_spec("2 light red bags"),
      Some((2, "light red".to_string()))
    );
    assert_eq!(
      parse_bag_spec("light red bags"),
      Some((0, "light red".to_string()))
    );
    assert_eq!(parse_bag_spec("no other bags"), None);
  }

  #[test]
  fn test_parse_extended_ruleset() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.
shiny gold bags contain 2 bright white bags.
faded blue bags contain no other bags.";

    let expected_ruleset: ExtendedRuleset = vec![
      (RED, vec![(1, WHITE), (2, YELLOW)]),
      (WHITE, vec![(1, GOLD)]),
      (GOLD, vec![(2, WHITE)]),
      (BLUE, vec![]),
    ]
    .iter()
    .map(|(key, value)| {
      (
        key.to_string(),
        value
          .iter()
          .map(|(amount, color)| (*amount, color.to_string()))
          .collect(),
      )
    })
    .collect();

    assert_eq!(parse_extended_ruleset(input), expected_ruleset);
  }

  #[test]
  fn test_parse_ruleset() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.
shiny gold bags contain 2 bright white bags.
faded blue bags contain no other bags.";

    let expected_ruleset: Ruleset = vec![
      (WHITE, vec![RED, GOLD]),
      (YELLOW, vec![RED]),
      (GOLD, vec![WHITE]),
    ]
    .iter()
    .map(|(key, value)| {
      (
        key.to_string(),
        value.iter().map(|x| x.to_string()).collect(),
      )
    })
    .collect();

    assert_eq!(parse_ruleset(input), expected_ruleset);
  }

  #[test]
  fn test_find_contained() {
    let ruleset: ExtendedRuleset = vec![
      (RED, vec![(1, WHITE), (2, YELLOW)]),
      (ORANGE, vec![(3, WHITE), (4, YELLOW)]),
      (WHITE, vec![(1, GOLD)]),
      (YELLOW, vec![(2, GOLD), (9, BLUE)]),
      (GOLD, vec![(1, OLIVE), (2, PLUM)]),
      (OLIVE, vec![(3, BLUE), (4, BLACK)]),
      (PLUM, vec![(5, BLUE), (6, BLACK)]),
      (BLUE, vec![]),
      (BLACK, vec![]),
    ]
    .iter()
    .map(|(key, value)| {
      (
        key.to_string(),
        value
          .iter()
          .map(|(amount, color)| (*amount, color.to_string()))
          .collect(),
      )
    })
    .collect();

    assert_eq!(find_amount_contained(&ruleset, GOLD), 32);
  }

  #[test]
  fn test_find_containers() {
    let ruleset: Ruleset = vec![
      (WHITE, vec![RED, ORANGE]),
      (YELLOW, vec![RED, ORANGE]),
      (GOLD, vec![WHITE, YELLOW]),
      (BLUE, vec![YELLOW, OLIVE, PLUM]),
      (OLIVE, vec![GOLD]),
      (PLUM, vec![GOLD]),
      (BLACK, vec![OLIVE, PLUM]),
    ]
    .iter()
    .map(|(key, value)| {
      (
        key.to_string(),
        value.iter().map(|x| x.to_string()).collect(),
      )
    })
    .collect();

    assert_eq!(
      find_containers(&ruleset, &[GOLD.to_string()]),
      vec![ORANGE, YELLOW, RED, WHITE]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<HashSet<String>>()
    );
  }
}
