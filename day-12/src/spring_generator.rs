use std::collections::HashMap;
use crate::spring_pattern::SpringPattern;
use crate::spring_state::SpringState::{*};

pub struct SpringGenerator<'a> {
    target: &'a SpringPattern,
    groups: Vec<u16>,
    group_sizes: Vec<usize>,
    cache: HashMap<(usize, usize), usize>,
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
        let group_sizes = group_sizes(&groups);

        SpringGenerator {
            target,
            groups,
            group_sizes,
            cache: HashMap::new()
        }
    }

    fn count_for(&mut self, pattern: SpringPattern, next_group: usize) -> usize {
        if !pattern.start_matches(self.target) {
            return 0;
        }

        if pattern.len() == self.target.len() && next_group == self.groups.len() {
            return 1;
        }

        if pattern.ends_with(Operational) {
            if let Some(&count) = self.cache.get(&(pattern.len(), next_group)) {
                return count;
            }
        }

        if pattern.len() < self.target.len() {
            let can_be_damaged = pattern.is_empty() || pattern.ends_with(Operational);
            let can_be_operational = pattern.len() + self.group_sizes[next_group] <= self.target.len();
            let mut count = 0;

            if can_be_damaged && next_group < self.groups.len() {
                let mut next = pattern.clone();

                for _ in 0..self.groups[next_group] {
                    next.push(Damaged);
                }

                count += self.count_for(next, next_group + 1);
            }

            if can_be_operational {
                let mut next = pattern.clone();
                next.push(Operational);

                count += self.count_for(next, next_group);
            }

            self.cache.insert((pattern.len(), next_group), count);
            return count;
        }

        return 0;
    }

    pub fn count(&mut self) -> usize {
        let mut pattern = SpringPattern::empty();
        let mut next_group = 0;

        for idx in 0..self.target.len() {
            match self.target[idx] {
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

        self.count_for(pattern, next_group)
    }
}
