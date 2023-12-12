use std::collections::VecDeque;
use crate::spring_pattern::SpringPattern;
use crate::spring_state::SpringState::{*};

pub struct SpringGenerator<'a> {
    target: &'a SpringPattern,
    groups: Vec<u16>,
    group_sizes: Vec<usize>,
    stack: VecDeque<Node>,
}

#[derive(Debug)]
struct Node {
    pattern: SpringPattern,
    next_group: usize,
}

fn group_sizes(groups: &Vec<u16>) -> Vec<usize> {
    let mut sizes = Vec::new();

    for i in 0..groups.len() {
        let part = &groups[i..];
        let sum: u16 = part.iter().sum();

        sizes.push((sum as usize) + part.len() - 1);
    }

    sizes.push(0);

    sizes
}

impl<'a> SpringGenerator<'a> {
    pub fn new(target: &'a SpringPattern, groups: Vec<u16>) -> Self {
        let mut stack = VecDeque::new();
        let mut pattern = SpringPattern::empty();
        let mut next_group = 0;

        for idx in 0..target.len() {
            match target[idx] {
                Damaged => pattern.push(Damaged),
                Operational => {
                    if pattern.len() > 0 && pattern.ends_with(Damaged) {
                        next_group += 1;
                    }

                    pattern.push(Operational);
                },
                Unknown => break,
            }
        }

        println!("Start with \"{pattern}\" {next_group}");

        stack.push_back(Node { pattern, next_group });

        let group_sizes = group_sizes(&groups);

        SpringGenerator { target, groups, group_sizes, stack }
    }
}

impl<'a> Iterator for SpringGenerator<'a> {
    type Item = SpringPattern;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mut node) = self.stack.pop_back() {
                if !node.pattern.start_matches(self.target) {
                    continue;
                }

                if node.pattern.len() < self.target.len() {
                    let can_be_damaged = node.pattern.is_empty() || node.pattern.ends_with(Operational);
                    let can_be_operational = node.pattern.len() + self.group_sizes[node.next_group] <= self.target.len();

                    if can_be_damaged && node.next_group < self.groups.len() {
                        let mut next = node.pattern.clone();

                        for _ in 0..self.groups[node.next_group] {
                            next.push(Damaged);
                        }

                        self.stack.push_back(Node {
                            pattern: next,
                            next_group: node.next_group + 1
                        });
                    }

                    if can_be_operational {
                        node.pattern.push(Operational);
                        self.stack.push_back(node);
                    }
                } else if node.pattern.len() == self.target.len() && node.next_group == self.groups.len() {
                    return Some(node.pattern);
                }
            } else {
                return None
            }
        }
    }
}