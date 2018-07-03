use std::sync::Arc;
use subculture::Subculture;
use neighbourhood::Neighbourhood;

#[derive(Clone)]
pub struct Scenario {
    pub id: String,
    pub subcultures: Vec<Arc<Subculture>>,
    pub neighbourhoods: Vec<Arc<Neighbourhood>>
}