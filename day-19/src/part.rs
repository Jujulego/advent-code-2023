use std::str::FromStr;

#[derive(Debug)]
pub enum PartNote {
    X,
    M,
    A,
    S
}

#[derive(Debug)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn get(&self, note: &PartNote) -> &u32 {
        match note {
            PartNote::X => &self.x,
            PartNote::M => &self.m,
            PartNote::A => &self.a,
            PartNote::S => &self.s,
        }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Part { x: 0, m: 0, a: 0, s: 0 };

        for note in s[1..(s.len() - 1)].split(',') {
            if let Some(eq) = note.find('=') {
                let v = u32::from_str(&note[(eq + 1)..]).map_err(|_| ())?;

                match &note[0..eq] {
                    "x" => part.x = v,
                    "m" => part.m = v,
                    "a" => part.a = v,
                    "s" => part.s = v,
                    _ => {}
                }
            }
        }

        Ok(part)
    }
}