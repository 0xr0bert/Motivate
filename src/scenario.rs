use std::sync::Arc;
use std::borrow::Borrow;
use subculture::Subculture;
use neighbourhood::Neighbourhood;

pub struct Scenario {
    pub id: String,
    pub subcultures: Vec<Arc<Subculture>>,
    pub neighbourhoods: Vec<Arc<Neighbourhood>>
}

impl Clone for Scenario {
    fn clone(&self) -> Self {
        Scenario {
            id: self.id.clone(),
            subcultures: self
                .subcultures
                .iter()
                .map(|rc: &Arc<Subculture>| {
                    let subculture: &Subculture = rc.borrow();
                    Arc::new(subculture.clone())
                })
                .collect(),
            neighbourhoods: self
                .neighbourhoods
                .iter()
                .map(|rc: &Arc<Neighbourhood>| {
                    let neighbourhood: &Neighbourhood = rc.borrow();
                    Arc::new(neighbourhood.clone())
                })
                .collect(),
        }
    }
}