use rand;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use rand::distributions;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand::seq::sample_slice_ref;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use transport_mode::TransportMode;
use journey_type::JourneyType;
use neighbourhood::Neighbourhood;
use subculture::Subculture;
use scenario::Scenario;
use agent::Agent;
use std::cmp;
use gaussian;
use serde_yaml;

pub fn load_agents_from_file(mut file: File, subcultures: &[Rc<Subculture>], neighbourhoods: &[Rc<Neighbourhood>]) -> Vec<Rc<RefCell<Agent>>> {
        info!("Loading parameters from file");
        let mut file_contents = String::new();

        file.read_to_string(&mut file_contents)
            .expect("There was an error reading the file");

        let mut residents: Vec<Rc<RefCell<Agent>>> = serde_yaml::from_slice(file_contents.as_bytes())
            .expect("There was an error parsing the file");

        let subcultures_kvp: HashMap<String, Rc<Subculture>> = subcultures
            .iter()
            .map(|subculture| (subculture.id.clone(), Rc::clone(subculture)))
            .collect();

        let neighbourhoods_kvp: HashMap<String, Rc<Neighbourhood>> = neighbourhoods
            .iter()
            .map(|neighbourhood| (neighbourhood.id.clone(), Rc::clone(neighbourhood)))
            .collect();

        for agent in residents.iter_mut() {
            agent.borrow_mut().subculture = Rc::clone(subcultures_kvp.get(&agent.borrow().subculture_id).expect("Subculture not found"));
            agent.borrow_mut().neighbourhood = Rc::clone(neighbourhoods_kvp.get(&agent.borrow().neighbourhood_id).expect("Agent not found"));
        }

        residents
}