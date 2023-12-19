use std::cmp::Ordering;
use std::str::FromStr;
use crate::part::{Part, PartNote};
use crate::part_range::PartRange;
use crate::rule::RuleResult::{Accepted, Refused, Target};

#[derive(Debug, Eq, PartialEq)]
pub enum RuleResult {
    Target(String),
    Accepted,
    Refused,
}

#[derive(Debug)]
pub struct Rule {
    pub note: PartNote,
    pub ordering: Ordering,
    pub threshold: u32,
    pub result: RuleResult,
}

impl Rule {
    pub fn process(&self, part: &Part) -> Option<&RuleResult> {
        if part.get(&self.note).cmp(&self.threshold) == self.ordering {
            Some(&self.result)
        } else {
            None
        }
    }
    
    pub fn process_range(&self, part_range: &PartRange) -> (PartRange, PartRange) {

        match self.ordering {
            Ordering::Less => part_range.split(&self.note, self.threshold),
            Ordering::Equal => panic!("This does not exists !"),
            Ordering::Greater => {
                let split = part_range.split(&self.note, self.threshold + 1);
                (split.1, split.0)
            }
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = match &s[..1] {
            "x" => PartNote::X,
            "m" => PartNote::M,
            "a" => PartNote::A,
            "s" => PartNote::S,
            n => panic!("Unknown note {n}"),
        };

        let ordering = match &s[1..2] {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            r => panic!("Unknown rule {r}"),
        };

        if let Some(colon) = s.find(':') {
            let threshold = u32::from_str(&s[2..colon]).expect("Invalid rule threshold");
            let result = match &s[(colon + 1)..] {
                "A" => Accepted,
                "R" => Refused,
                workflow => Target(workflow.to_string())
            };

            Ok(Rule { note, ordering, threshold, result })
        } else {
            panic!("Invalid rule");
        }
    }
}