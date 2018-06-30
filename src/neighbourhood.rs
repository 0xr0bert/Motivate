use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use transport_mode::TransportMode;

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
