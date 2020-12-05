use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

#[derive(Debug)]
pub enum Dir {
  Lower,
  Higher,
}

impl Dir {
  pub fn new(character: char) -> Self {
    match character {
      'L' | 'F' => Dir::Lower,
      'R' | 'B' => Dir::Higher,
      _ => unreachable!(),
    }
  }
}

pub struct BoardingPass {
  row_locator: Vec<Dir>,
  col_locator: Vec<Dir>,
}

impl BoardingPass {
  pub fn new(locator_id: &str) -> Self {
    lazy_static! { // use lazy_static to compile the regex only once
      static ref RE: Regex = Regex::new(r"^(?P<row>\w{7})(?P<col>\w{3})$").unwrap();
    }
    let captured = RE.captures(locator_id).unwrap();
    let row_locator = captured.name("row").unwrap().as_str();
    let col_locator = captured.name("col").unwrap().as_str();

    BoardingPass {
      row_locator: row_locator.chars().map(Dir::new).collect(),
      col_locator: col_locator.chars().map(Dir::new).collect(),
    }
  }

  pub fn get_seat_id(&self) -> usize {
    let row = locate(0..128, &self.row_locator);
    let col = locate(0..8, &self.col_locator);
    row * 8 + col
  }
}

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

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<BoardingPass> {
  input.lines().map(BoardingPass::new).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(boarding_passes: &[BoardingPass]) -> usize {
  boarding_passes
    .iter()
    .map(|x| x.get_seat_id())
    .max()
    .unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(boarding_passes: &[BoardingPass]) -> usize {
  let mut seat_ids: Vec<usize> = boarding_passes.iter().map(|x| x.get_seat_id()).collect();
  seat_ids.sort();

  for i in 0..seat_ids.len() {
    // skip first and last seats
    if (i == 0 || i >= seat_ids.len() - 1) {
      continue;
    }

    // detect a gap of a single seat
    if (seat_ids[i - 1] == seat_ids[i] - 2) {
      return seat_ids[i] - 1 as usize;
    }
  }

  unreachable!();
}

#[cfg(test)]
mod tests {
  use super::*;
  use Dir::*;

  #[test]
  fn test_get_seat_id() {
    assert_eq!(BoardingPass::new("BFFFBBFRRR").get_seat_id(), 567);
    assert_eq!(BoardingPass::new("FFFBBBFRRR").get_seat_id(), 119);
    assert_eq!(BoardingPass::new("BBFFBBFRLL").get_seat_id(), 820);
  }

  #[test]
  fn test_solve_part1() {
    let boarding_passes = [
      BoardingPass::new("BFFFBBFRRR"),
      BoardingPass::new("FFFBBBFRRR"),
      BoardingPass::new("BBFFBBFRLL"),
    ];

    assert_eq!(solve_part1(&boarding_passes), 820);
  }

  #[test]
  fn test_locate() {
    // BFFFBBF
    assert_eq!(
      locate(
        0..128,
        &[Higher, Lower, Lower, Lower, Higher, Higher, Lower]
      ),
      70
    );
    assert_eq!(
      locate(
        0..128,
        &[Lower, Lower, Lower, Higher, Higher, Higher, Lower]
      ),
      14
    );
    assert_eq!(
      locate(
        0..128,
        &[Higher, Higher, Lower, Lower, Higher, Higher, Lower]
      ),
      102
    );

    assert_eq!(locate(0..8, &[Higher, Higher, Higher]), 7);
    assert_eq!(locate(0..8, &[Higher, Lower, Lower]), 4);
  }
}
