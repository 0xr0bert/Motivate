use transport_mode::TransportMode;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Intervention {
    /// The day number of the intervention
    pub day: u32,
    
    /// Changes in the neighbourhood
    pub neighbourhood_changes: Vec<NeighbourhoodChange>,

    /// Changes in the subculture
    pub subculture_changes: Vec<SubcultureChange>,

    /// Change in the number of bikes
    pub change_in_number_of_bikes: i32,

    /// Change in the number of cars
    pub change_in_number_of_cars: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NeighbourhoodChange {
    /// Neighbourhood ID
    pub id: String,

    /// Be very careful that this does not make the supportiveness > 1 or < 0
    pub increase_in_supportiveness: HashMap<TransportMode, f32>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SubcultureChange {
    /// Subculture ID
    pub id: String,

    /// Be very careful that this does not make the desirability > 1 or < 0
    pub increase_in_supportiveness: HashMap<TransportMode, f32>
}