use crate::player;
use crate::stage;
use std::fmt;

use serde::{Deserialize, Serialize};

const MAP_CELLS: usize = 7;
const MAP_RANGE: (f32, f32) = (-350000.0, 350000.0);
const QUADRANT_SIZE: f32 = 100000.0;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Quadrant {
    // row 1
    ForsakenFortress,
    StarIsland,
    NorthernFairyIsland,
    GaleIsle,
    CresentMoonIsland,
    SevenStarIsles,
    OverlookIsland,

    // row 2
    FourEyeReef,
    MotherAndChildIsles,
    SpectacleIsland,
    WindfallIsland,
    PawprintIsle,
    DragonRoostIsland,
    FlightControlPlatform,

    // row 3
    WesternFairyIsland,
    RockSpireIsle,
    TingleIsland,
    NorthernTriangleIsle,
    EasternFairyIsland,
    FireMountain,
    StarBeltArchipelago,

    // row 4
    ThreeEyeRoof,
    GreatfishIsle,
    CyclopsReef,
    SixEyeReef, // note, this is the exact center of the map according to Link's position when in the overworld
    TowerOfTheGods,
    EasternTriangleIsland,
    ThornedFairyIsland,

    // row 5
    NeedleRockIsle,
    IsletOfSteel,
    StoneWatcherIsland,
    SouthernTriangleIsland,
    PrivateOasis,
    BombIsland,
    BirdsPeakRock,

    // row 6
    DiamondSteppeIsland,
    FiveEyeReef,
    SharkIsland,
    SouthernFairyIsland,
    IceRingIsle,
    ForestHaven,
    CliffPlateauIsles,

    // row 7
    HorseshoeIsland,
    OutsetIsland,
    HeadstoneIsland,
    TwoEyeReef,
    AngularIsles,
    BoatingCourse,
    FiveStarIsles,

    Unknown,
}

impl Quadrant {
    pub fn new(position: (usize, usize)) -> Self {
        match position {
            // row 1
            (0, 0) => Self::ForsakenFortress,
            (0, 1) => Self::StarIsland,
            (0, 2) => Self::NorthernFairyIsland,
            (0, 3) => Self::GaleIsle,
            (0, 4) => Self::CresentMoonIsland,
            (0, 5) => Self::SevenStarIsles,
            (0, 6) => Self::OverlookIsland,

            // row 2
            (1, 0) => Self::FourEyeReef,
            (1, 1) => Self::MotherAndChildIsles,
            (1, 2) => Self::SpectacleIsland,
            (1, 3) => Self::WindfallIsland,
            (1, 4) => Self::PawprintIsle,
            (1, 5) => Self::DragonRoostIsland,
            (1, 6) => Self::FlightControlPlatform,

            // row 3
            (2, 0) => Self::WesternFairyIsland,
            (2, 1) => Self::RockSpireIsle,
            (2, 2) => Self::TingleIsland,
            (2, 3) => Self::NorthernTriangleIsle,
            (2, 4) => Self::EasternFairyIsland,
            (2, 5) => Self::FireMountain,
            (2, 6) => Self::StarBeltArchipelago,

            // row 4
            (3, 0) => Self::ThreeEyeRoof,
            (3, 1) => Self::GreatfishIsle,
            (3, 2) => Self::CyclopsReef,
            (3, 3) => Self::SixEyeReef,
            (3, 4) => Self::TowerOfTheGods,
            (3, 5) => Self::EasternTriangleIsland,
            (3, 6) => Self::ThornedFairyIsland,

            // row 5
            (4, 0) => Self::NeedleRockIsle,
            (4, 1) => Self::IsletOfSteel,
            (4, 2) => Self::StoneWatcherIsland,
            (4, 3) => Self::SouthernTriangleIsland,
            (4, 4) => Self::PrivateOasis,
            (4, 5) => Self::BombIsland,
            (4, 6) => Self::BirdsPeakRock,

            // row 6
            (5, 0) => Self::DiamondSteppeIsland,
            (5, 1) => Self::FiveEyeReef,
            (5, 2) => Self::SharkIsland,
            (5, 3) => Self::SouthernFairyIsland,
            (5, 4) => Self::IceRingIsle,
            (5, 5) => Self::ForestHaven,
            (5, 6) => Self::CliffPlateauIsles,

            // row 7
            (6, 0) => Self::HorseshoeIsland,
            (6, 1) => Self::OutsetIsland,
            (6, 2) => Self::HeadstoneIsland,
            (6, 3) => Self::TwoEyeReef,
            (6, 4) => Self::AngularIsles,
            (6, 5) => Self::BoatingCourse,
            (6, 6) => Self::FiveStarIsles,

            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Quadrant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            // row 1
            Self::ForsakenFortress => write!(f, "Forsaken Fortress"),
            Self::StarIsland => write!(f, "Star Island"),
            Self::NorthernFairyIsland => write!(f, "Northern Fairy Island"),
            Self::GaleIsle => write!(f, "Gale Island"),
            Self::CresentMoonIsland => write!(f, "Crescent Moon Island"),
            Self::SevenStarIsles => write!(f, "Seven-Star Island"),
            Self::OverlookIsland => write!(f, "Overlook Island"),

            // row 2
            Self::FourEyeReef => write!(f, "Four-Eye Reef"),
            Self::MotherAndChildIsles => write!(f, "Mother & Child Isles"),
            Self::SpectacleIsland => write!(f, "Spectacle Island"),
            Self::WindfallIsland => write!(f, "Windfall Island"),
            Self::PawprintIsle => write!(f, "Pawprint Island"),
            Self::DragonRoostIsland => write!(f, "Dragon Roost Island"),
            Self::FlightControlPlatform => write!(f, "Flight Control Platform"),

            // row 3
            Self::WesternFairyIsland => write!(f, "Western Fairy Island"),
            Self::RockSpireIsle => write!(f, "Rock Spire Isle"),
            Self::TingleIsland => write!(f, "Tingle Island"),
            Self::NorthernTriangleIsle => write!(f, "Northern Triangle Isle"),
            Self::EasternFairyIsland => write!(f, "Eastern Fairy Island"),
            Self::FireMountain => write!(f, "Fire Mountain"),
            Self::StarBeltArchipelago => write!(f, "Star Belt Archipelago"),

            // row 4
            Self::ThreeEyeRoof => write!(f, "Three-Eye Reef"),
            Self::GreatfishIsle => write!(f, "Greatfish Isle"),
            Self::CyclopsReef => write!(f, "Cyclops Reef"),
            Self::SixEyeReef => write!(f, "Six-Eye Reef"),
            Self::TowerOfTheGods => write!(f, "Tower of the Gods"),
            Self::EasternTriangleIsland => write!(f, "Eastern Triangle Island"),
            Self::ThornedFairyIsland => write!(f, "Thorned Fairy Island"),

            // row 5
            Self::NeedleRockIsle => write!(f, "Needle rock Isle"),
            Self::IsletOfSteel => write!(f, "Islet of Steel"),
            Self::StoneWatcherIsland => write!(f, "Stone Watcher Island"),
            Self::SouthernTriangleIsland => write!(f, "Southern Triangle Island"),
            Self::PrivateOasis => write!(f, "Private Oasis"),
            Self::BombIsland => write!(f, "Bomb Island"),
            Self::BirdsPeakRock => write!(f, "Birds Peak Rock"),

            // row 6
            Self::DiamondSteppeIsland => write!(f, "Diamond Steppe Island"),
            Self::FiveEyeReef => write!(f, "Five-Eye Reef"),
            Self::SharkIsland => write!(f, "Shark Island"),
            Self::SouthernFairyIsland => write!(f, "Southern Fairy Island"),
            Self::IceRingIsle => write!(f, "Ice Ring Isle"),
            Self::ForestHaven => write!(f, "Forest Haven"),
            Self::CliffPlateauIsles => write!(f, "Cliff Plateau Isles"),

            // row 7
            Self::HorseshoeIsland => write!(f, "Horseshoe Island"),
            Self::OutsetIsland => write!(f, "Outset Island"),
            Self::HeadstoneIsland => write!(f, "Headstone Island"),
            Self::TwoEyeReef => write!(f, "Two-Eye Reef"),
            Self::AngularIsles => write!(f, "Angular Isles"),
            Self::BoatingCourse => write!(f, "Boating Course"),
            Self::FiveStarIsles => write!(f, "Five-Star Isles"),

            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Map {
    pub grid: Vec<Vec<Quadrant>>,
}

impl Map {
    pub fn new() -> Self {
        let mut grid = vec![vec![Quadrant::Unknown; MAP_CELLS]; MAP_CELLS];
        for row in 0..7usize {
            for col in 0..7usize {
                grid[row][col] = Quadrant::new((row, col));
            }
        }

        Self { grid }
    }

    pub fn current_quadrant(
        &self,
        stage_id: &stage::StageId,
        position: &player::Position,
    ) -> Quadrant {
        match stage_id {
            stage::StageId::Sea(stage::Sea::Overworld) => {}
            stage::StageId::Sea(stage::Sea::Alt) => {}
            _ => return Quadrant::Unknown,
        };

        if position.x < MAP_RANGE.0
            || position.x > MAP_RANGE.1
            || position.z < MAP_RANGE.0
            || position.z > MAP_RANGE.1
        {
            return Quadrant::Unknown;
        }

        for rect in self.quadrant_rects() {
            if position.x >= rect.from.0
                && position.x <= rect.to.0
                && position.z >= rect.from.1
                && position.z <= rect.to.1
            {
                return rect.quadrant;
            }
        }

        return Quadrant::Unknown;
    }

    fn quadrant_rects(&self) -> Vec<Rect> {
        let mut rects = Vec::new();

        for (row, line) in self.grid.iter().enumerate() {
            for (col, quadrant) in line.iter().enumerate() {
                let from = (
                    MAP_RANGE.0 + (col as f32 * QUADRANT_SIZE),
                    MAP_RANGE.0 + (row as f32 * QUADRANT_SIZE),
                );
                let to = (from.0 + QUADRANT_SIZE, from.1 + QUADRANT_SIZE);

                rects.push(Rect {
                    quadrant: *quadrant,
                    from,
                    to,
                });
            }
        }

        rects
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    quadrant: Quadrant,
    from: (f32, f32),
    to: (f32, f32),
}
