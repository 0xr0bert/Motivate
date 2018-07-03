use std::collections::HashMap;
use transport_mode::TransportMode;

/// A categorical distance for commute
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum JourneyType {
    LocalCommute,
    CityCommute,
    DistantCommute
}

impl JourneyType {
    /// Gets the cost for different TransportModes for the JourneyType
    pub fn cost(&self) -> HashMap<TransportMode, f32> {
        match *self {
            JourneyType::LocalCommute => hashmap!{
                TransportMode::Car => 0.1f32,
                TransportMode::PublicTransport => 0.1f32,
                TransportMode::Cycle => 0.4f32,
                TransportMode::Walk => 0.2f32
            },
            JourneyType::CityCommute => hashmap!{
                TransportMode::Car => 0.1f32,
                TransportMode::PublicTransport => 0.1f32,
                TransportMode::Cycle => 0.6f32,
                TransportMode::Walk => 0.9f32
            },
            JourneyType::DistantCommute => hashmap!{
                TransportMode::Car => 0.1f32,
                TransportMode::PublicTransport => 0.1f32,
                TransportMode::Cycle => 1.1f32,
                TransportMode::Walk => 1.1f32
            }
        }
    }
}
