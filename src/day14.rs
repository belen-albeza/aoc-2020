use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Mask {
  or: u64,
  and: u64,
  raw: String,
}

impl Mask {
  pub fn apply_v1(&self, x: u64) -> u64 {
    (self.or | x) & self.and
  }

  pub fn apply_v2(&self, x: u64) -> Vec<u64> {
    let value = format!("{:036b}", x);
    let floating_mask: String = value
      .chars()
      .enumerate()
      .map(|(i, value_char)| match self.raw.chars().nth(i).unwrap() {
        'x' => 'x',
        '0' => value_char,
        '1' => '1',
        _ => unreachable!(),
      })
      .collect();
    self.get_values_for_floating(&floating_mask)
  }

  fn get_values_for_floating(&self, floating_mask: &str) -> Vec<u64> {
    let mut partials: Vec<String> = vec![];

    for char in floating_mask.chars() {
      match char {
        'x' => {
          if partials.len() == 0 {
            partials.push("0".to_string());
            partials.push("1".to_string());
          } else {
            let mut zero_partials = partials.clone();
            let mut one_partials = partials.clone();
            for partial in zero_partials.iter_mut() {
              partial.push_str("0");
            }
            for partial in one_partials.iter_mut() {
              partial.push_str("1");
            }

            partials = [zero_partials, one_partials].concat();
          }
        }
        _ => {
          if partials.len() == 0 {
            partials.push(char.to_string());
          } else {
            for partial in partials.iter_mut() {
              partial.push_str(&char.to_string())
            }
          }
        }
      }
    }

    partials
      .iter()
      .map(|x| u64::from_str_radix(x, 2).unwrap())
      .collect()
  }
}

impl Default for Mask {
  fn default() -> Self {
    Mask {
      and: 0xffff_ffff_ffff_ffff,
      or: 0x0000_0000_0000_0000,
      raw: "0000000000000000".to_string(),
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
      raw: text.to_lowercase(),
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Version {
  V1,
  V2,
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

  fn exec(&mut self, instruction: &Instruction, version: Version) {
    match instruction {
      Instruction::SetMask(mask) => {
        self.mask = mask.clone();
      }
      Instruction::Mem(address, value) => match version {
        Version::V1 => {
          self.memory.insert(*address, self.mask.apply_v1(*value));
        }
        Version::V2 => {
          for addr in self.mask.apply_v2(*address).iter() {
            self.memory.insert(*addr, *value);
          }
        }
      },
    }
  }

  pub fn run(&mut self, program: &[Instruction], version: Version) {
    // reset internal state
    self.mask = Default::default();
    self.memory = HashMap::new();

    // exec all the instructions
    for instruction in program.into_iter() {
      self.exec(&instruction, version);
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
  machine.run(program, Version::V1);

  machine.memory.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(program: &[Instruction]) -> u64 {
  let mut machine = Machine::new();
  machine.run(program, Version::V2);

  machine.memory.values().sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mask_from_str() {
    assert_eq!(
      "x10X".parse::<Mask>(),
      Ok(Mask {
        or: 0b0100,
        and: 0b1101,
        raw: "x10x".to_string()
      })
    );

    assert!("x1200x".parse::<Mask>().is_err());
  }

  #[test]
  fn test_mask_apply_v1() {
    let mask: Mask = "X1XXXX0X".parse().unwrap();
    assert_eq!(mask.apply_v1(11), 73);

    // check that default mask doesn't transform the number
    let mask: Mask = Default::default();
    assert_eq!(mask.apply_v1(11), 11);
  }

  #[test]
  fn test_mask_apply_v2() {
    let mask: Mask = "00X1001X".parse().unwrap();
    assert_eq!(mask.apply_v2(42), vec![26, 58, 27, 59]);
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
    let mask: Mask = "x0x1".parse().unwrap();
    machine.exec(&Instruction::SetMask(mask.clone()), Version::V1);
    assert_eq!(machine.mask, mask);
  }

  #[test]
  fn test_machine_mem_v1() {
    let mut machine: Machine = Machine::new();
    machine.mask = "X1XXXX0X".parse().unwrap();
    machine.exec(&Instruction::Mem(42, 11), Version::V1);
    assert_eq!(machine.memory.get(&42), Some(&73));
  }

  #[test]
  fn test_machine_mem_v2() {
    let mut machine: Machine = Machine::new();
    machine.mask = "00X1001X".parse().unwrap();
    machine.exec(&Instruction::Mem(42, 11), Version::V2);
    assert_eq!(machine.memory.get(&26), Some(&11));
    assert_eq!(machine.memory.get(&27), Some(&11));
    assert_eq!(machine.memory.get(&58), Some(&11));
    assert_eq!(machine.memory.get(&59), Some(&11));
  }

  #[test]
  fn test_machine_run_v1() {
    let mut machine: Machine = Machine::new();
    let program = vec![
      Instruction::SetMask("X1XXXX0X".parse().unwrap()),
      Instruction::Mem(42, 11),
    ];
    machine.run(&program, Version::V1);
    assert_eq!(machine.memory.get(&42), Some(&73));
    assert_eq!(machine.mask, "X1XXXX0X".parse().unwrap());
  }

  #[test]
  fn test_parse_input() {
    let input = "mask = 000000000000000000000000000000X1001X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    assert_eq!(
      parse_input(input),
      Ok(vec![
        Instruction::SetMask("000000000000000000000000000000X1001X".parse().unwrap()),
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

  #[test]
  fn test_solve_part2() {
    let program = vec![
      Instruction::SetMask("000000000000000000000000000000X1001X".parse().unwrap()),
      Instruction::Mem(42, 100),
      Instruction::SetMask("00000000000000000000000000000000X0XX".parse().unwrap()),
      Instruction::Mem(26, 1),
    ];
    assert_eq!(solve_part2(&program), 208);
  }
}
