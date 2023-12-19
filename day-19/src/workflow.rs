use std::str::FromStr;
use crate::part::Part;
use crate::part_range::PartRange;
use crate::rule::{Rule, RuleResult};
use crate::rule::RuleResult::{Accepted, Refused, Target};

#[derive(Debug)]
pub struct Workflow {
    pub rules: Vec<Rule>,
    pub result: RuleResult,
}

impl Workflow {
    pub fn process(&self, part: &Part) -> &RuleResult {
        self.rules.iter()
            .filter_map(|rule| rule.process(part))
            .next().unwrap_or(&self.result)
    }

    pub fn process_range(&self, mut part_range: PartRange) -> Vec<(PartRange, &RuleResult)> {
        let range = &mut part_range;

        let mut ranges: Vec<(PartRange, &RuleResult)> = self.rules.iter()
            .scan(range, |range, rule| {
                if range.is_empty() {
                    None
                } else {
                    let (pass, rest) = rule.process_range(range);
                    **range = rest;

                    Some((pass, &rule.result))
                }
            })
            .filter(|(range, _)| !range.is_empty())
            .collect();

        if !part_range.is_empty() {
            ranges.push((part_range, &self.result));
        }

        ranges
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules: Vec<&str> = s[1..(s.len() - 1)].split(',').collect();

        Ok(Workflow {
            rules: rules[..(rules.len() - 1)].iter().map(|s| s.parse().unwrap()).collect(),
            result: match rules.last() {
                Some(&"A") => Accepted,
                Some(&"R") => Refused,
                Some(&workflow) => Target(workflow.to_string()),
                None => panic!("No end result for workflow"),
            }
        })
    }
}