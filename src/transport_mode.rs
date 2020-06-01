use std::fmt;

/// The Transport Modes that can be taken by agents
#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum TransportMode {
    Car,
    PublicTransport,
    Cycle,
    Walk
}

impl fmt::Display for TransportMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}