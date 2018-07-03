use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use transport_mode::TransportMode;

/// A Neighbourhood
#[derive(Clone)]
pub struct Neighbourhood {
    /// The ID for the Neighbourhood, neighbourhoods are equal if they share the same id
    pub id: u8,
    /// A score from 0-1 for each transport mode, on how supportive the environment is
    pub supportiveness: HashMap<TransportMode, f32>
}

impl PartialEq for Neighbourhood {
    fn eq(&self, other: &Neighbourhood) -> bool {
        self.id == other.id
    }
}

impl Eq for Neighbourhood {}

impl Hash for Neighbourhood {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
