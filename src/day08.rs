use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
  Accumulate,
  Jump,
  NoOp,
}

impl FromStr for Opcode {
  type Err = String;
  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text {
      "acc" => Ok(Opcode::Accumulate),
      "jmp" => Ok(Opcode::Jump),
      "nop" => Ok(Opcode::NoOp),
      _ => Err(format!("Unrecognized opcode {}", text).to_string()),
    }
  }
}

pub type Instruction = (Opcode, i64);

#[derive(Debug, PartialEq)]
enum MachineError {
  InifiteLoop,
}

impl fmt::Display for MachineError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

struct Machine {
  program: Vec<Instruction>,
  ip: usize,
  accumulator: i64,
  ip_run: HashSet<usize>,
}

impl Machine {
  pub fn new(program: &[Instruction]) -> Self {
    Machine {
      program: program.to_vec(),
      ip: 0,
      accumulator: 0,
      ip_run: HashSet::new(),
    }
  }

  pub fn step(&mut self) -> Result<(), MachineError> {
    let (opcode, arg) = &self.program[self.ip];
    match opcode {
      Opcode::Accumulate => {
        self.accumulator += arg;
        self.ip += 1;
      }
      Opcode::NoOp => self.ip += 1,
      Opcode::Jump => self.ip += *arg as usize,
    };

    Ok(())
  }

  pub fn run(&mut self) -> Result<(), MachineError> {
    // loop thorugh program until the end
    while self.ip < self.program.len() {
      // return error if we detect a loop in the program
      if self.ip_run.contains(&self.ip) {
        return Err(MachineError::InifiteLoop);
      }
      self.ip_run.insert(self.ip);
      self.step()?;
    }

    Ok(())
  }
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|line| {
      let mut tokens = line.split(" ");
      let opcode = Opcode::from_str(tokens.next().unwrap()).unwrap();
      let arg = tokens.next().unwrap().parse().unwrap();

      (opcode, arg)
    })
    .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(program: &[Instruction]) -> i64 {
  let mut machine = Machine::new(program);
  let _ = machine.run();
  machine.accumulator
}

#[aoc(day8, part2)]
pub fn solve_part2(program: &[Instruction]) -> Result<i64, String> {
  for (i, (opcode, arg)) in program.iter().enumerate() {
    let instruction = match opcode {
      Opcode::NoOp => Some((Opcode::Jump, *arg)),
      Opcode::Jump => Some((Opcode::NoOp, *arg)),
      _ => None,
    };

    if let Some(patch) = instruction {
      let mut machine = Machine::new(&[&program[..i], &[patch], &program[i + 1..]].concat());
      let result = machine.run();
      if result.is_ok() {
        return Ok(machine.accumulator);
      }
    }
  }

  Err("Could not patch program to fix infinite loop.".to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_opcode_from_str() {
    assert_eq!(Opcode::from_str("acc"), Ok(Opcode::Accumulate));
    assert_eq!(Opcode::from_str("nop"), Ok(Opcode::NoOp));
    assert_eq!(Opcode::from_str("jmp"), Ok(Opcode::Jump));
    assert_eq!(
      Opcode::from_str("???"),
      Err("Unrecognized opcode ???".to_string())
    );
  }

  #[test]
  fn test_step() {
    let mut machine_acc = Machine::new(&[(Opcode::Accumulate, 4)]);
    let _ = machine_acc.step();
    assert_eq!(machine_acc.accumulator, 4);
    assert_eq!(machine_acc.ip, 1);

    let mut machine_nop = Machine::new(&[(Opcode::NoOp, 0)]);
    let _ = machine_nop.step();
    assert_eq!(machine_nop.accumulator, 0);
    assert_eq!(machine_nop.ip, 1);

    let mut machine_jmp = Machine::new(&[(Opcode::Jump, 12)]);
    let _ = machine_jmp.step();
    assert_eq!(machine_jmp.accumulator, 0);
    assert_eq!(machine_jmp.ip, 12);
  }

  #[test]
  fn test_run_ok() {
    let mut machine = Machine::new(&[(Opcode::NoOp, 0), (Opcode::Accumulate, 8)]);
    let result = machine.run();
    assert_eq!(machine.ip, 2);
    assert_eq!(machine.accumulator, 8);
    assert!(result.is_ok());
  }

  #[test]
  fn test_run_error_infinite_loop() {
    let mut machine = Machine::new(&[(Opcode::Jump, 0)]);
    let result = machine.run();
    assert_eq!(machine.ip, 0);
    assert_eq!(machine.accumulator, 0);
    assert_eq!(result.unwrap_err(), MachineError::InifiteLoop);
  }

  #[test]
  fn test_parse_input() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(
      parse_input(input),
      [
        (Opcode::NoOp, 0),
        (Opcode::Accumulate, 1),
        (Opcode::Jump, 4),
        (Opcode::Accumulate, 3),
        (Opcode::Jump, -3),
        (Opcode::Accumulate, -99),
        (Opcode::Accumulate, 1),
        (Opcode::Jump, -4),
        (Opcode::Accumulate, 6),
      ]
    );
  }
}
