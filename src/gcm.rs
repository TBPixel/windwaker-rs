use dolphin_memory::Dolphin;
use std::fmt;
use std::io;

pub const SUPPORTED_GAME_IDS: [&'static str; 2] = ["GZLEO1", "GZLE99"];
pub const HEADER: usize = 0x80000000;

// is_supported_wind_waker returns whether the currently running emulated game is
// a supported version of TLoZ: The Wind Waker (eg. NA version) or not.
pub fn is_supported_wind_waker(d: &Dolphin) -> bool {
    let game_id = GameId::default().read(d);
    if let Ok(id) = game_id {
        return SUPPORTED_GAME_IDS.contains(&&*String::from(id));
    }

    false
}

#[derive(Default, Debug, Clone)]
pub struct GameId(String);

impl GameId {
    pub fn new(raw: &str) -> Self {
        Self(raw.to_owned())
    }

    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        self.0 = d
            .read_string(6, HEADER, None)?
            .trim_matches(char::from(0))
            .to_owned();

        Ok(self.clone())
    }
}

impl From<GameId> for String {
    fn from(game_id: GameId) -> Self {
        game_id.0
    }
}

impl fmt::Display for GameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
