use dolphin_memory::Dolphin;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;

pub const KORL_PTR: usize = 0x803CA75C;
pub const KORL_SPEED_OFFSET: usize = 0x254;

pub const KORL_Y_OFFSET: usize = 0x1FC;

#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Height(f32);

impl Height {
    pub fn new(f: f32) -> Self {
        Self(f)
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.0 = d.read_f32(KORL_PTR, Some(&[KORL_Y_OFFSET]))?;

        Ok(*self)
    }
}

impl From<Height> for f32 {
    fn from(h: Height) -> Self {
        h.0
    }
}

#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Speed(f32);

impl Speed {
    pub fn new(f: f32) -> Self {
        Self(f)
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.0 = d.read_f32(KORL_PTR, Some(&[KORL_SPEED_OFFSET]))?;

        Ok(*self)
    }
}

impl From<Speed> for f32 {
    fn from(s: Speed) -> Self {
        s.0
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}
