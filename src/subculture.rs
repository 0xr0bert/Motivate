use std::hash::Hasher;
use std::hash::Hash;
use std::collections::HashMap;
use transport_mode::TransportMode;

/// A demographic subculture
#[derive(Clone)]
pub struct Subculture {
    /// The ID for the subculture
    pub id: String,
    /// The desirability is a score from 0 - 1 for each transport mode
    pub desirability: HashMap<TransportMode, f32>
}

impl PartialEq for Subculture {
    fn eq(&self, other: &Subculture) -> bool {
        self.id == other.id
    }
}

impl Eq for Subculture {}

impl Hash for Subculture {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
