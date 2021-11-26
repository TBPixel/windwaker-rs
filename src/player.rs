use dolphin_memory::Dolphin;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;

// Inventory
pub const RUPEES: usize = 0x803c4c0c;

// Player

// NOTE: this pointer points to the character
// the player is controlling, including link,
// medlii, makar, tower servant, seagull
// when you are controlling them
pub const PLAYER_PTR: usize = 0x803ca410;

// NOTE: This pointer always points to Link.
pub const LINK_PTR: usize = 0x803CA754;

pub const PLAYER_X: usize = 0x803E440C;
pub const PLAYER_Y: usize = 0x803E4410;
pub const PLAYER_Z: usize = 0x803E4414;

pub const PLAYER_SPEED_OFFSET: usize = 0x35bc;

pub const PLAYER_SPEED_MAX: usize = 0x8035CEEC;

pub const PLAYER_HP: usize = 0x803C4C0A;
pub const PLAYER_HP_MAX: usize = 0x803C4C08;
pub const PLAYER_MP: usize = 0x803C4C1C;
pub const PLAYER_MP_MAX: usize = 0x803C4C1B;

#[derive(Default, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.x = d.read_f32(PLAYER_X, None)?;
        self.y = d.read_f32(PLAYER_Y, None)?;
        self.z = d.read_f32(PLAYER_Z, None)?;

        Ok(*self)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {:.2} y: {:.2} z: {:.2}", self.x, self.y, self.z)
    }
}

#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Speed(f32);

impl Speed {
    pub fn new(f: f32) -> Self {
        Self(f)
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.0 = d.read_f32(PLAYER_PTR, Some(&[PLAYER_SPEED_OFFSET]))?;

        Ok(*self)
    }

    pub fn write(&mut self, f: f32, d: &Dolphin) -> io::Result<Self> {
        d.write_f32(f, PLAYER_PTR, Some(&[PLAYER_SPEED_OFFSET]))?;
        self.0 = f;

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

#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SpeedMax(f32);

impl SpeedMax {
    pub fn new(f: f32) -> Self {
        Self(f)
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.0 = d.read_f32(PLAYER_SPEED_MAX, None)?;

        Ok(*self)
    }

    pub fn write(&mut self, f: f32, d: &Dolphin) -> io::Result<Self> {
        d.write_f32(f, PLAYER_SPEED_MAX, None)?;
        self.0 = f;

        Ok(*self)
    }
}

impl From<SpeedMax> for f32 {
    fn from(s: SpeedMax) -> Self {
        s.0
    }
}

impl fmt::Display for SpeedMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rupees(u16);

impl Rupees {
    pub fn new(n: u16) -> Self {
        Self(n)
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.0 = d.read_u16(RUPEES, None)?;

        Ok(*self)
    }
}

impl fmt::Display for Rupees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Rupees> for u16 {
    fn from(rupees: Rupees) -> Self {
        rupees.0
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Hp {
    pub current: u16,
    pub max: u16,
}

impl Hp {
    pub fn new(current: u16, max: u16) -> Self {
        Self { current, max }
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.current = d.read_u16(PLAYER_HP, None)?;
        self.max = d.read_u16(PLAYER_HP_MAX, None)?;

        Ok(*self)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Mp {
    pub current: u8,
    pub max: u8,
}

impl Mp {
    pub fn new(current: u8, max: u8) -> Self {
        Self { current, max }
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.current = d.read_u8(PLAYER_MP, None)?;
        self.max = d.read_u8(PLAYER_MP_MAX, None)?;

        Ok(*self)
    }

    pub fn write_current(&mut self, current: u8, d: &Dolphin) -> io::Result<Self> {
        d.write_u8(current, PLAYER_MP, None)?;
        self.current = current;

        Ok(*self)
    }
}
