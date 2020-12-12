use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
  North,
  South,
  East,
  West,
}

impl Dir {
  fn to_delta(&self) -> (i64, i64) {
    match self {
      Self::North => (0, -1),
      Self::South => (0, 1),
      Self::East => (1, 0),
      Self::West => (-1, 0),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
  Move(Dir, i64),
  Rotate(i64),
  Advance(i64),
}

impl FromStr for Action {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let action_code = text
      .chars()
      .next()
      .expect(format!("can't parse action code from `{}`", text).as_str());
    let value: i64 = text[1..]
      .parse()
      .expect(format!("can't parse action value from `{}`", text).as_str());
    match action_code {
      'F' => Ok(Action::Advance(value)),
      'R' => Ok(Action::Rotate(value)),
      'L' => Ok(Action::Rotate(-value)),
      'N' => Ok(Action::Move(Dir::North, value)),
      'S' => Ok(Action::Move(Dir::South, value)),
      'E' => Ok(Action::Move(Dir::East, value)),
      'W' => Ok(Action::Move(Dir::West, value)),
      _ => Err(format!("Unrecognized action {}", action_code)),
    }
  }
}

struct Ship {
  facing: Dir,
  position: (i64, i64),
}

impl Ship {
  pub fn new() -> Self {
    Ship {
      facing: Dir::East,
      position: (0, 0),
    }
  }

  fn move_to(&mut self, direction: Dir, distance: i64) {
    let delta = direction.to_delta();
    self.position.0 += distance * delta.0;
    self.position.1 += distance * delta.1;
  }

  fn rotate(&mut self, angle: i64) {
    const DIRS: [Dir; 4] = [Dir::East, Dir::South, Dir::West, Dir::North];
    let steps = angle / 90; // we only allow +-90 angle increments

    let current = DIRS.iter().position(|&x| x == self.facing).unwrap() as i64;
    let index = (steps + current).rem_euclid(4);

    self.facing = DIRS[index as usize];
  }

  pub fn exec(&mut self, action: Action) {
    match action {
      Action::Advance(value) => self.move_to(self.facing, value),
      Action::Move(dir, value) => self.move_to(dir, value),
      Action::Rotate(angle) => self.rotate(angle),
    };
  }
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<Action> {
  input
    .split_whitespace()
    .map(|x| Action::from_str(x).unwrap())
    .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(actions: &[Action]) -> i64 {
  let mut ship = Ship::new();

  for action in actions.into_iter() {
    ship.exec(*action);
  }

  ship.position.0 + ship.position.1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_action_from_str() {
    assert_eq!(Action::from_str("F10"), Ok(Action::Advance(10)));
    assert_eq!(Action::from_str("R180"), Ok(Action::Rotate(180)));
    assert_eq!(Action::from_str("L90"), Ok(Action::Rotate(-90)));
    assert_eq!(Action::from_str("N5"), Ok(Action::Move(Dir::North, 5)));
    assert_eq!(Action::from_str("S50"), Ok(Action::Move(Dir::South, 50)));
    assert_eq!(Action::from_str("E2"), Ok(Action::Move(Dir::East, 2)));
    assert_eq!(Action::from_str("W2"), Ok(Action::Move(Dir::West, 2)));
    assert!(Action::from_str("?10").is_err());
  }

  #[test]
  fn test_exec_rotate() {
    let mut ship = Ship::new();
    ship.facing = Dir::East;
    ship.exec(Action::Rotate(-90));
    assert_eq!(ship.facing, Dir::North);

    let mut ship = Ship::new();
    ship.facing = Dir::North;
    ship.exec(Action::Rotate(180));
    assert_eq!(ship.facing, Dir::South);
  }

  #[test]
  fn test_exec_move() {
    let mut ship = Ship::new();
    ship.position = (0, 0);

    ship.exec(Action::Move(Dir::North, 2));
    assert_eq!(ship.position, (0, -2));
    ship.exec(Action::Move(Dir::East, 4));
    assert_eq!(ship.position, (4, -2));
    ship.exec(Action::Move(Dir::West, 5));
    assert_eq!(ship.position, (-1, -2));
    ship.exec(Action::Move(Dir::South, 5));
    assert_eq!(ship.position, (-1, 3));
  }

  #[test]
  fn test_exec_advance() {
    let mut ship = Ship::new();
    ship.facing = Dir::North;

    ship.exec(Action::Advance(10));
    assert_eq!(ship.position, (0, -10));
  }

  #[test]
  fn test_parse_input() {
    let input = "F10 N3 F7 R90 F11";
    assert_eq!(
      parse_input(input),
      vec![
        Action::Advance(10),
        Action::Move(Dir::North, 3),
        Action::Advance(7),
        Action::Rotate(90),
        Action::Advance(11),
      ]
    );
  }

  #[test]
  fn test_solve_part1() {
    let input = vec![
      Action::Advance(10),
      Action::Move(Dir::North, 3),
      Action::Advance(7),
      Action::Rotate(90),
      Action::Advance(11),
    ];
    assert_eq!(solve_part1(&input), 25);
  }
}
