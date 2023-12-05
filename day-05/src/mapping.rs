use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
pub struct Mapping {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

impl FromStr for Mapping {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split(' ');

        Ok(Mapping {
            dst_start: u64::from_str_radix(numbers.next().unwrap(), 10)?,
            src_start: u64::from_str_radix(numbers.next().unwrap(), 10)?,
            length: u64::from_str_radix(numbers.next().unwrap(), 10)?,
        })
    }
}

impl Mapping {
    pub fn map(&self, val: u64) -> Option<u64> {
        if val >= self.src_start && val - self.src_start < self.length {
            Some(self.dst_start + (val - self.src_start))
        } else {
            None
        }
    }

    pub fn map_range(&self, range: &Range<u64>) -> (Option<Range<u64>>, Vec<Range<u64>>) {
        let dst_end = self.dst_start + self.length;
        let src_end = self.src_start + self.length;

        // Easy cases
        if range.end <= self.src_start {
            return (None, vec![range.clone()]);
        }

        if range.start >= src_end {
            return (None, vec![range.clone()]);
        }

        // Overlap 1st bound
        if range.start < self.src_start && range.end < src_end {
            return (
                Some(self.dst_start..self.map(range.end).unwrap()),
                vec![range.start..self.src_start]
            );
        }

        // Overlap 2nd bound
        if range.start >= self.src_start && range.end >= src_end {
            return (
                Some(self.map(range.start).unwrap()..dst_end),
                vec![src_end..range.end]
            );
        }

        // Inbound
        if range.start >= self.src_start && range.end < src_end {
            return (
                Some(self.map(range.start).unwrap()..self.map(range.end).unwrap()),
                vec![]
            );
        }

        // Outbound
        if range.start < self.src_start && range.end >= src_end {
            return (
                Some(self.dst_start..dst_end),
                vec![
                    range.start..self.src_start,
                    src_end..range.end
                ]
            );
        }

        panic!("This should not be possible ! {:?} on {:?}", self, range);
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_before() {
        let mapping = Mapping {
            dst_start: 52,
            src_start: 50,
            length: 48
        };

        assert_eq!(
            mapping.map_range(&(0..10)),
            (None, vec![0..10])
        );
    }

    #[test]
    fn test_after() {
        let mapping = Mapping {
            dst_start: 52,
            src_start: 50,
            length: 48
        };

        assert_eq!(
            mapping.map_range(&(100..110)),
            (None, vec![100..110])
        );
    }

    #[test]
    fn test_overlap_1st_bound() {
        let mapping = Mapping {
            dst_start: 52,
            src_start: 50,
            length: 48
        };

        assert_eq!(
            mapping.map_range(&(40..60)),
            (Some(52..62), vec![40..50])
        );
    }

    #[test]
    fn test_inbound() {
        let mapping = Mapping {
            dst_start: 52,
            src_start: 50,
            length: 48
        };

        assert_eq!(
            mapping.map_range(&(60..70)),
            (Some(62..72), vec![])
        );
    }

    #[test]
    fn test_overlap_2nd_bound() {
        let mapping = Mapping {
            dst_start: 52,
            src_start: 50,
            length: 48
        };

        assert_eq!(
            mapping.map_range(&(70..100)),
            (Some(72..100), vec![98..100])
        );
    }

    #[test]
    fn test_outbound() {
        let mapping = Mapping {
            dst_start: 52,
            src_start: 50,
            length: 48
        };

        assert_eq!(
            mapping.map_range(&(40..100)),
            (Some(52..100), vec![40..50, 98..100])
        );
    }
}