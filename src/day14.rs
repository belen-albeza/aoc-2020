use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mask {
  or: u64,
  and: u64,
}

impl Mask {
  pub fn apply(&self, x: u64) -> u64 {
    (self.or | x) & self.and
  }
}

impl Default for Mask {
  fn default() -> Self {
    Mask {
      and: 0xffff_ffff_ffff_ffff,
      or: 0x0000_0000_0000_0000,
    }
  }
}

impl FromStr for Mask {
  type Err = std::num::ParseIntError;
  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let and_mask = text.to_lowercase().replace('x', "1");
    let or_mask = text.to_lowercase().replace('x', "0");

    Ok(Self {
      and: u64::from_str_radix(&and_mask, 2)?,
      or: u64::from_str_radix(&or_mask, 2)?,
    })
  }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
  SetMask(Mask),
  Mem(u64, u64), // address, value
}

impl FromStr for Instruction {
  type Err = String;
  fn from_str(text: &str) -> Result<Self, Self::Err> {
    lazy_static! { // use lazy_static to compile the regex only once
      static ref RE_SETMASK: Regex =
      Regex::new(r"^mask\s?=\s?(?P<mask>[01Xx]+)").unwrap();
      static ref RE_MEM: Regex =
      Regex::new(r"^mem\[(?P<address>\d+)\]\s?=\s?(?P<value>\d+)").unwrap();
    }

    let error_msg = format!("Cannot parse instruction: {}", text);

    if RE_SETMASK.is_match(text) {
      let captured = RE_SETMASK.captures(text).unwrap();
      let mask = captured
        .name("mask")
        .unwrap()
        .as_str()
        .parse::<Mask>()
        .or(Err(error_msg.to_string()))?;
      Ok(Self::SetMask(mask))
    } else if RE_MEM.is_match(text) {
      let captured = RE_MEM.captures(text).unwrap();
      let address = captured
        .name("address")
        .unwrap()
        .as_str()
        .parse::<u64>()
        .or(Err(error_msg.to_string()))?;
      let value = captured
        .name("value")
        .unwrap()
        .as_str()
        .parse::<u64>()
        .or(Err(error_msg.to_string()))?;
      Ok(Instruction::Mem(address, value))
    } else {
      Err(error_msg)
    }
  }
}

pub struct Machine {
  memory: HashMap<u64, u64>,
  mask: Mask,
}

impl Machine {
  pub fn new() -> Self {
    Machine {
      memory: HashMap::new(),
      mask: Default::default(),
    }
  }

  fn exec(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::SetMask(mask) => {
        self.mask = mask;
      }
      Instruction::Mem(address, value) => {
        self.memory.insert(address, self.mask.apply(value));
      }
    }
  }

  pub fn run(&mut self, program: &[Instruction]) {
    // reset internal state
    self.mask = Default::default();
    self.memory = HashMap::new();

    // exec all the instructions
    for instruction in program {
      self.exec(*instruction);
    }
  }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Result<Vec<Instruction>, String> {
  let instructions: Vec<Instruction> = input
    .lines()
    .map(|x| x.parse::<Instruction>().unwrap())
    .collect();

  Ok(instructions)
}

#[aoc(day14, part1)]
pub fn solve_part1(program: &[Instruction]) -> u64 {
  let mut machine = Machine::new();
  machine.run(program);

  machine.memory.values().sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mask_from_str() {
    assert_eq!(
      "x1xXxx0X".parse::<Mask>(),
      Ok(Mask {
        or: 0b01000000,
        and: 0b11111101
      })
    );

    assert!("x1200x".parse::<Mask>().is_err());
  }

  #[test]
  fn test_mask_apply() {
    // mask: X1XXXX0X
    let mask = Mask {
      or: 0b01000000,
      and: 0b11111101,
    };
    assert_eq!(mask.apply(11), 73);

    // check that default mask doesn't transform the number
    let mask: Mask = Default::default();
    assert_eq!(mask.apply(11), 11);
  }

  #[test]
  fn test_instruction_from_str() {
    assert_eq!(
      "mask = X1XXXX0X".parse::<Instruction>(),
      Ok(Instruction::SetMask("X1XXXX0X".parse::<Mask>().unwrap()))
    );
    assert!("mask = 21XX?X0X".parse::<Instruction>().is_err());
    assert!("mask =".parse::<Instruction>().is_err());

    assert_eq!(
      "mem[42] = 1234".parse::<Instruction>(),
      Ok(Instruction::Mem(42, 1234))
    );
    assert!("mem[a] = b".parse::<Instruction>().is_err());

    assert!("???".parse::<Instruction>().is_err());
  }

  #[test]
  fn test_machine_set_mask() {
    let mut machine: Machine = Machine::new();
    let mask = "x0x1".parse().unwrap();
    machine.exec(Instruction::SetMask(mask));
    assert_eq!(machine.mask, mask);
  }

  #[test]
  fn test_machine_mem() {
    let mut machine: Machine = Machine::new();
    machine.mask = "X1XXXX0X".parse().unwrap();
    machine.exec(Instruction::Mem(42, 11));
    assert_eq!(machine.memory.get(&42), Some(&73));
  }

  #[test]
  fn test_machine_run() {
    let mut machine: Machine = Machine::new();
    let program = vec![
      Instruction::SetMask("X1XXXX0X".parse().unwrap()),
      Instruction::Mem(42, 11),
    ];
    machine.run(&program);
    assert_eq!(machine.memory.get(&42), Some(&73));
    assert_eq!(machine.mask, "X1XXXX0X".parse().unwrap());
  }

  #[test]
  fn test_parse_input() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    assert_eq!(
      parse_input(input),
      Ok(vec![
        Instruction::SetMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap()),
        Instruction::Mem(8, 11),
        Instruction::Mem(7, 101),
        Instruction::Mem(8, 0),
      ])
    );
  }

  #[test]
  fn test_solve_part1() {
    let program = vec![
      Instruction::SetMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap()),
      Instruction::Mem(8, 11),
      Instruction::Mem(7, 101),
      Instruction::Mem(8, 0),
    ];

    assert_eq!(solve_part1(&program), 165);
  }
}
