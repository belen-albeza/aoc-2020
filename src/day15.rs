use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

pub struct Game {
  turn: u64,
  memory: HashMap<u64, (u64, u64)>, // (latest_turn, age)
  starters: Vec<u64>,
  last_number: u64,
}

impl Game {
  pub fn new(starters: &[u64]) -> Self {
    Game {
      turn: 0,
      memory: HashMap::new(),
      starters: starters.to_vec(),
      last_number: 0,
    }
  }

  fn record_turn(&mut self, number: u64) {
    let mut age = 0;

    if let Some((latest_turn, _)) = self.memory.get(&number) {
      age = self.turn - latest_turn;
    }

    self.memory.insert(number, (self.turn, age));
  }

  fn get_turns_apart_for(&self, number: u64) -> u64 {
    let (_, age) = self.memory.get(&number).unwrap();
    *age
  }
}

impl Iterator for Game {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {
    let mut number = 0;

    // first turns: starters are spoken
    if (self.turn as usize) < self.starters.len() {
      number = self.starters[self.turn as usize];
    }
    // following turns: number exists
    else if self.memory.contains_key(&self.last_number) {
      number = self.get_turns_apart_for(self.last_number);
    }
    // following turns: number doesn't exist -> speak 0

    self.record_turn(number);
    self.turn += 1;
    self.last_number = number;

    Some(number)
  }
}

fn run_game(starters: &[u64], turns: u64) -> Result<u64, String> {
  let mut game = Game::new(starters);
  game
    .nth((turns - 1) as usize)
    .ok_or(format!("Game ended before turn #{}", turns))
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
  input
    .split(',')
    .map(|x| x.parse::<u64>())
    .collect::<Result<Vec<u64>, std::num::ParseIntError>>()
}

#[aoc(day15, part1)]
fn solve_part1(input: &[u64]) -> Result<u64, String> {
  run_game(input, 2020)
}

#[aoc(day15, part2)]
fn solve_part2(input: &[u64]) -> Result<u64, String> {
  run_game(input, 30000000)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_game_starters() {
    let mut game = Game::new(&[0, 3, 6]);
    assert_eq!(game.next(), Some(0));
    assert_eq!(game.next(), Some(3));
    assert_eq!(game.next(), Some(6));
  }

  #[test]
  fn test_game_following_turns() {
    let mut game = Game::new(&[0, 3, 6]).skip(3);

    assert_eq!(game.next(), Some(0));
    assert_eq!(game.next(), Some(3));
    assert_eq!(game.next(), Some(3));
    assert_eq!(game.next(), Some(1));
    assert_eq!(game.next(), Some(0));
    assert_eq!(game.next(), Some(4));
    assert_eq!(game.next(), Some(0));
  }

  #[test]
  fn test_parse_input() {
    assert_eq!(parse_input("0,3,6"), Ok(vec![0, 3, 6]));
  }

  #[test]
  fn test_solve_part1() {
    assert_eq!(solve_part1(&vec![0, 3, 6]), Ok(436));
    assert_eq!(solve_part1(&vec![1, 3, 2]), Ok(1));
    assert_eq!(solve_part1(&vec![2, 1, 3]), Ok(10));
    assert_eq!(solve_part1(&vec![1, 2, 3]), Ok(27));
    assert_eq!(solve_part1(&vec![3, 1, 2]), Ok(1836));
  }
}
