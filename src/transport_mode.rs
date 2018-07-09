#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TransportMode {
    Car,
    PublicTransport,
    Cycle,
    Walk
}

// impl TransportMode {
//     pub fn to_string(&self) -> String {
//         match *self {
//             TransportMode::Car => "Car".to_owned(),
//             TransportMode::Cycle => "Cycle".to_owned(),
//             TransportMode::PublicTransport => "PublicTransport".to_owned(),
//             TransportMode::Walk => "Walk".to_owned()
//         }
//     }
// }
