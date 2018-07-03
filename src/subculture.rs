use std::collections::HashMap;
use transport_mode::TransportMode;

/// A demographic subculture
#[derive(PartialEq, Clone)]
pub struct Subculture {
    /// The desirability is a score from 0 - 1 for each transport mode
    pub desirability: HashMap<TransportMode, f32>
}
