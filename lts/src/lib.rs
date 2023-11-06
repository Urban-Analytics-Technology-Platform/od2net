mod allowed;
mod bike_ottawa;
mod parse;
mod speed_limit_only;
mod tags;
#[cfg(test)]
mod tests;
#[cfg(target_arch = "wasm32")]
mod wasm;

use serde_repr::{Deserialize_repr, Serialize_repr};

pub use allowed::is_cycling_allowed;
pub use bike_ottawa::bike_ottawa;
pub use speed_limit_only::speed_limit_only;
pub use tags::Tags;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LTS {
    NotAllowed = 0,
    LTS1 = 1,
    LTS2 = 2,
    LTS3 = 3,
    LTS4 = 4,
}
