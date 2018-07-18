use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::cell::RefCell;
use transport_mode::TransportMode;

// TODO: Is supportiveness the same as capacity
/// A Neighbourhood
#[derive(Clone, Serialize, Deserialize)]
pub struct Neighbourhood {
    /// The ID for the Neighbourhood, neighbourhoods are equal if they share the same id
    pub id: String,
    /// A score from 0-1 for each transport mode, on how supportive the environment is
    pub supportiveness: RefCell<HashMap<TransportMode, f32>>,
    /// The maximum capacity for a transport mode, at which there is no congestion
    pub capacity: RefCell<HashMap<TransportMode, u32>>

}

impl PartialEq for Neighbourhood {
    /// Tests equality of neighbourhood's only equal if they have the same id
    fn eq(&self, other: &Neighbourhood) -> bool {
        self.id == other.id
    }
}

impl Eq for Neighbourhood {}

impl Hash for Neighbourhood {
    /// Returns the has of the id
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
