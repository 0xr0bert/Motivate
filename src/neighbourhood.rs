use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use transport_mode::TransportMode;

/// A Neighbourhood
/// id: the ID for the Neighbourhood, neighbourhoods are equal if they share the same id
/// supportiveness: A score from 0-1 for each transport mode, on how supportive the environment is
#[derive(Clone)]
pub struct Neighbourhood {
    pub id: u8,
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
