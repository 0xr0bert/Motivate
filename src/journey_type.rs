use std::collections::HashMap;
use std::fmt;
use transport_mode::TransportMode;

/// A categorical distance for commute
#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum JourneyType {
    LocalCommute,
    CityCommute,
    DistantCommute
}

impl JourneyType {
    /// Gets the (economic) cost for different TransportModes for the JourneyType
    /// Where there is no entry the journey is impossible
    /// * Returns: A HashMap from TransportMode to the cost of transporting by that mode, for the supplied JourneyType
    pub fn cost(&self) -> HashMap<TransportMode, f32> {
        match *self {
            JourneyType::LocalCommute => hashmap!{
                TransportMode::Car => 0.2f32,
                TransportMode::PublicTransport => 0.2f32,
                TransportMode::Cycle => 0.2f32,
                TransportMode::Walk => 0.2f32
            },
            JourneyType::CityCommute => hashmap!{
                TransportMode::Car => 0.3f32,
                TransportMode::PublicTransport => 0.3f32,
                TransportMode::Cycle => 0.6f32,
                TransportMode::Walk => 0.9f32
            },
            JourneyType::DistantCommute => hashmap!{
                TransportMode::Car => 0.1f32,
                TransportMode::PublicTransport => 0.1f32
            }
        }
    }
}

impl fmt::Display for JourneyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
