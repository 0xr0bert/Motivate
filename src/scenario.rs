use std::rc::Rc;
use subculture::Subculture;
use neighbourhood::Neighbourhood;

pub struct Scenario {
    pub id: String,
    pub subcultures: Vec<Rc<Subculture>>,
    pub neighbourhoods: Vec<Rc<Neighbourhood>>
}