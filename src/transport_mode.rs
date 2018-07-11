/// The Transport Modes that can be taken by agents
#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TransportMode {
    Car,
    PublicTransport,
    Cycle,
    Walk
}
