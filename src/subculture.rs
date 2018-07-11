use std::hash::Hasher;
use std::hash::Hash;
use std::cell::RefCell;
use std::collections::HashMap;
use transport_mode::TransportMode;

/// A demographic subculture
#[derive(Clone, Serialize, Deserialize)]
pub struct Subculture {
    /// The ID for the subculture
    pub id: String,
    /// The desirability is a score from 0 - 1 for each transport mode
    pub desirability: RefCell<HashMap<TransportMode, f32>>
}

impl PartialEq for Subculture {
    /// Returns true if the subcultures have the same ID
    fn eq(&self, other: &Subculture) -> bool {
        self.id == other.id
    }
}

impl Eq for Subculture {}

impl Hash for Subculture {
    /// The ID is hashed
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
