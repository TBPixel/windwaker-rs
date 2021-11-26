use dolphin_memory::Dolphin;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;

// STAGE_ID refers to the current stage that the player is in.
// This is quite a high-level representation and further digging
// must be done to determine the exact location of the player.
// For example, this byte will tell us that the player is
// on the overworld (sea), but not what quadrant their in.
pub const STAGE_ID: usize = 0x803C53A4;

// 803C9D3C,8 - The current stage name.
pub const STAGE_NAME: usize = 0x803C9D3C;
// 803C9D48,8 - The next stage name the player is about to go to.
pub const NEXT_STAGE_NAME: usize = 0x803C9D48;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Interiors {
    Ships,
    Houses,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sea {
    Overworld,
    Alt,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Caves {
    Interiors,
    Alt,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StageId {
    Sea(Sea),
    ForsakenFortress,
    DragonRoostCavern,
    ForbiddenWoods,
    TowerOfTheGods,
    EarthTemple,
    WindTemple,
    GanonsTower,
    Hyrule,
    Interiors(Interiors),
    Caves(Caves),
    TestMaps,
    Unknown,
}

impl StageId {
    pub fn new(id: u8) -> Self {
        match id {
            0x0 => Self::Sea(Sea::Overworld),
            0x1 => Self::Sea(Sea::Alt),
            0x2 => Self::ForsakenFortress,
            0x3 => Self::DragonRoostCavern,
            0x4 => Self::ForbiddenWoods,
            0x5 => Self::TowerOfTheGods,
            0x6 => Self::EarthTemple,
            0x7 => Self::WindTemple,
            0x8 => Self::GanonsTower,
            0x9 => Self::Hyrule,
            0xA => Self::Interiors(Interiors::Ships),
            0xB => Self::Interiors(Interiors::Houses),
            0xC => Self::Caves(Caves::Interiors),
            0xD => Self::Caves(Caves::Alt),
            0xE => Self::Unknown,
            0xF => Self::TestMaps,
            _ => Self::Unknown,
        }
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        let id = d.read_u8(STAGE_ID, None)?;

        Ok(Self::new(id))
    }
}

impl fmt::Display for StageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sea(sea) => match *sea {
                Sea::Overworld => write!(f, "Overworld"),
                Sea::Alt => write!(f, "Overworld (alt)"),
            },
            Self::ForsakenFortress => write!(f, "Forsaken Fortress"),
            Self::DragonRoostCavern => write!(f, "Dragon Roost Cavern"),
            Self::ForbiddenWoods => write!(f, "Forbidden Woods"),
            Self::TowerOfTheGods => write!(f, "Tower of the Gods"),
            Self::EarthTemple => write!(f, "Earth Temple"),
            Self::WindTemple => write!(f, "Wind Temple"),
            Self::GanonsTower => write!(f, "Ganons Tower"),
            Self::Hyrule => write!(f, "Hyrule"),
            Self::Interiors(interiors) => match *interiors {
                Interiors::Houses => write!(f, "Interiors (houses)"),
                Interiors::Ships => write!(f, "Interiors (ships)"),
            },
            Self::Caves(caves) => match *caves {
                Caves::Interiors => write!(f, "Caves (interior)"),
                Caves::Alt => write!(f, "Caves (alt)"),
            },
            Self::TestMaps => write!(f, "Test Maps"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for StageId {
    fn default() -> Self {
        StageId::Unknown
    }
}

#[derive(Default, Debug, Clone)]
pub struct StageName(String);

impl StageName {
    pub fn new(raw: &str) -> Self {
        Self(raw.to_owned())
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.0 = d
            .read_string(8, STAGE_NAME, None)?
            .trim_matches(char::from(0))
            .to_owned();

        Ok(self.clone())
    }
}

impl From<StageName> for String {
    fn from(stage: StageName) -> Self {
        stage.0
    }
}

impl fmt::Display for StageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, Clone)]
pub struct NextStageName(String);

impl NextStageName {
    pub fn new(raw: &str) -> Self {
        Self(raw.to_owned())
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.0 = d
            .read_string(8, NEXT_STAGE_NAME, None)?
            .trim_matches(char::from(0))
            .to_owned();

        Ok(self.clone())
    }
}

impl From<NextStageName> for String {
    fn from(stage: NextStageName) -> Self {
        stage.0
    }
}

impl fmt::Display for NextStageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
