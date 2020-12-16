use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct RangeValidator(RangeInclusive<u64>);

impl FromStr for RangeValidator {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let error_msg = format!("Error parsing range validator `{}`", text);
    let values = text
      .split('-')
      .map(|x| x.parse::<u64>().map_err(|_| error_msg.clone()))
      .collect::<Result<Vec<u64>, String>>()?;

    let min = values.get(0).ok_or(error_msg.clone())?;
    let max = values.get(1).ok_or(error_msg.clone())?;

    Ok(Self(*min..=*max))
  }
}

impl RangeValidator {
  pub fn is_valid(&self, x: u64) -> bool {
    self.0.contains(&x)
  }
}

#[derive(Debug, Clone, PartialEq)]
struct NumFieldValidator {
  ranges: Vec<RangeValidator>,
}

impl FromStr for NumFieldValidator {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let ranges = text
      .split(" or ")
      .map(|x| x.parse::<RangeValidator>())
      .collect::<Result<Vec<RangeValidator>, String>>()?;
    Ok(Self { ranges: ranges })
  }
}

impl NumFieldValidator {
  fn is_valid(&self, x: u64) -> bool {
    self
      .ranges
      .iter()
      .map(|range| range.is_valid(x))
      .any(|res| !!res)
  }
}

#[derive(Debug, Clone, PartialEq)]
struct Field {
  name: String,
  validator: NumFieldValidator,
}

impl FromStr for Field {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let error_msg = format!("Error parsing `{}`", text);
    let mut chunks = text.split(": ");

    let name = chunks.next().ok_or(error_msg.clone())?;
    let validator = chunks
      .next()
      .ok_or(error_msg.clone())?
      .parse::<NumFieldValidator>()?;

    Ok(Self {
      name: name.to_string(),
      validator: validator,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketRules {
  fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
  fields: Vec<u64>,
}

impl FromStr for Ticket {
  type Err = std::num::ParseIntError;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let fields = text
      .split(',')
      .map(|x| x.parse::<u64>())
      .collect::<Result<Vec<u64>, Self::Err>>()?;
    Ok(Ticket { fields: fields })
  }
}

impl Ticket {
  fn get_invalid_fields(&self, rules: &TicketRules) -> Vec<u64> {
    self
      .fields
      .iter()
      .filter(|x| {
        !rules
          .fields
          .iter()
          .map(|field| &field.validator)
          .any(|validator| validator.is_valid(**x))
      })
      .map(|x| *x)
      .collect()
  }
}

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> Result<(TicketRules, Vec<Ticket>), String> {
  let mut sections = input.split("\n\n");

  // 1st section -> validation rules
  let raw_ruleset = sections.next().ok_or("No ruleset present".to_string())?;
  let fields: Vec<Field> = raw_ruleset
    .lines()
    .map(|raw_rule| raw_rule.parse::<Field>())
    .collect::<Result<Vec<Field>, String>>()?;

  let ticket_error_msg = "No ticket present";

  let tickets: Vec<Ticket> = vec![
    // 2nd section -> your ticket
    vec![sections
      .next()
      .ok_or(ticket_error_msg.to_string())?
      .lines()
      .skip(1) // NOTE: skip first line (it's a label)
      .next()
      .ok_or(ticket_error_msg.to_string())?
      .parse::<Ticket>()
      .or(Err("Invalid ticket".to_string()))?],
    sections
      .next()
      .ok_or(ticket_error_msg.to_string())?
      .lines()
      .skip(1) // NOTE: skip first line (it's a label)
      .map(|x| x.parse::<Ticket>())
      .collect::<Result<Vec<Ticket>, _>>()
      .or(Err("Invalid ticket".to_string()))?,
  ]
  .concat();

  Ok((TicketRules { fields: fields }, tickets))
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &(TicketRules, Vec<Ticket>)) -> u64 {
  let (ruleset, tickets) = input;

  tickets
    .iter()
    .skip(1)
    .map::<u64, _>(|ticket| ticket.get_invalid_fields(ruleset).iter().sum())
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_range_validator_parse() {
    assert_eq!("1-12".parse::<RangeValidator>(), Ok(RangeValidator(1..=12)));
    assert!("1-a".parse::<RangeValidator>().is_err());
    assert!("".parse::<RangeValidator>().is_err());
  }

  #[test]
  fn test_range_validator_is_valid() {
    let range = RangeValidator(1..=2);
    assert_eq!(range.is_valid(0), false);
    assert_eq!(range.is_valid(1), true);
    assert_eq!(range.is_valid(2), true);
    assert_eq!(range.is_valid(3), false);
  }

  #[test]
  fn test_numfield_validator_parse() {
    assert_eq!(
      "1-2 or 4-5".parse::<NumFieldValidator>(),
      Ok(NumFieldValidator {
        ranges: vec![RangeValidator(1..=2), RangeValidator(4..=5)]
      })
    );
  }

  #[test]
  fn test_field_parse() {
    assert_eq!(
      "class: 1-3 or 5-7".parse::<Field>(),
      Ok(Field {
        name: "class".to_string(),
        validator: NumFieldValidator {
          ranges: vec![RangeValidator(1..=3), RangeValidator(5..=7)]
        }
      })
    );

    assert!("class: ab".parse::<Field>().is_err());
  }

  #[test]
  fn test_parse_input() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
    let (ruleset, tickets) = parse_input(input).unwrap();
    assert_eq!(
      ruleset,
      TicketRules {
        fields: vec![
          Field {
            name: "class".to_string(),
            validator: NumFieldValidator {
              ranges: vec![RangeValidator(1..=3), RangeValidator(5..=7)]
            }
          },
          Field {
            name: "row".to_string(),
            validator: NumFieldValidator {
              ranges: vec![RangeValidator(6..=11), RangeValidator(33..=44)]
            }
          },
          Field {
            name: "seat".to_string(),
            validator: NumFieldValidator {
              ranges: vec![RangeValidator(13..=40), RangeValidator(45..=50)]
            }
          }
        ]
      },
    );

    assert_eq!(
      tickets,
      vec![
        Ticket {
          fields: vec![7, 1, 14]
        },
        Ticket {
          fields: vec![7, 3, 47]
        },
        Ticket {
          fields: vec![40, 4, 50]
        },
        Ticket {
          fields: vec![55, 2, 20]
        },
        Ticket {
          fields: vec![38, 6, 12]
        },
      ]
    );
  }

  #[test]
  fn test_solve_part1() {
    let input = (
      TicketRules {
        fields: vec![
          Field {
            name: "class".to_string(),
            validator: NumFieldValidator {
              ranges: vec![RangeValidator(1..=3), RangeValidator(5..=7)],
            },
          },
          Field {
            name: "row".to_string(),
            validator: NumFieldValidator {
              ranges: vec![RangeValidator(6..=11), RangeValidator(33..=44)],
            },
          },
          Field {
            name: "seat".to_string(),
            validator: NumFieldValidator {
              ranges: vec![RangeValidator(13..=40), RangeValidator(45..=50)],
            },
          },
        ],
      },
      vec![
        Ticket {
          fields: vec![7, 1, 14],
        },
        Ticket {
          fields: vec![7, 3, 47],
        },
        Ticket {
          fields: vec![40, 4, 50],
        },
        Ticket {
          fields: vec![55, 2, 20],
        },
        Ticket {
          fields: vec![38, 6, 12],
        },
      ],
    );

    assert_eq!(solve_part1(&input), 71);
  }
}
