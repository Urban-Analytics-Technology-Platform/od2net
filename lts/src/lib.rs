mod bike_ottawa;
mod parse;
mod speed_limit_only;
mod tags;

pub use bike_ottawa::bike_ottawa;
pub use speed_limit_only::speed_limit_only;
pub use tags::Tags;

#[derive(PartialEq, PartialOrd)]
pub enum LTS {
    NotAllowed,
    LTS1,
    LTS2,
    LTS3,
    LTS4,
}

impl LTS {
    pub fn into_json(self) -> usize {
        match self {
            LTS::NotAllowed => 0,
            LTS::LTS1 => 1,
            LTS::LTS2 => 2,
            LTS::LTS3 => 3,
            LTS::LTS4 => 4,
        }
    }
}
