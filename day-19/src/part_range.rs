use std::ops::Range;
use crate::part::PartNote;

#[derive(Clone, Debug)]
pub struct PartRange {
    pub x: Range<u32>,
    pub m: Range<u32>,
    pub a: Range<u32>,
    pub s: Range<u32>,
}

impl PartRange {
    pub fn new() -> Self {
        PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    pub fn get(&self, note: &PartNote) -> &Range<u32> {
        match note {
            PartNote::X => &self.x,
            PartNote::M => &self.m,
            PartNote::A => &self.a,
            PartNote::S => &self.s,
        }
    }

    pub fn get_mut(&mut self, note: &PartNote) -> &mut Range<u32> {
        match note {
            PartNote::X => &mut self.x,
            PartNote::M => &mut self.m,
            PartNote::A => &mut self.a,
            PartNote::S => &mut self.s,
        }
    }

    pub fn split(&self, note: &PartNote, value: u32) -> (Self, Self) {
        let mut before = self.clone();
        let mut after = self.clone();

        before.get_mut(note).end = value;
        after.get_mut(note).start = value;

        (before, after)
    }

    pub fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }

    pub fn size(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}
