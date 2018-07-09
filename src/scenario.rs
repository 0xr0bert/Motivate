use std::sync::Arc;
use subculture::Subculture;
use neighbourhood::Neighbourhood;

/// A scenario for a simulation run
#[derive(Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// The scenario ID
    pub id: String,
    /// The subcultures in the scenario
    pub subcultures: Vec<Arc<Subculture>>,
    /// The neighbourhoods in the scenario
    pub neighbourhoods: Vec<Arc<Neighbourhood>>
}
