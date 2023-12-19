use std::str::FromStr;
use crate::part::Part;
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