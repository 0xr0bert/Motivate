#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum TransportMode {
    Car,
    PublicTransport,
    Cycle,
    Walk
}