use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;
use subculture::Subculture;
use serde_yaml;
use neighbourhood::Neighbourhood;
use intervention::Intervention;

/// A scenario for a simulation run
#[derive(Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// The scenario ID
    pub id: String,
    /// The subcultures in the scenario
    pub subcultures: Vec<Rc<Subculture>>,
    /// The neighbourhoods in the scenario
    pub neighbourhoods: Vec<Rc<Neighbourhood>>,

    /// The number of bikes in the scenario
    pub number_of_bikes: u32,

    /// The number of cars in the scenario
    pub number_of_cars: u32,

    /// The intervention
    pub intervention: Intervention
}

impl Scenario {
    /// Loads a scenario from a file
    /// * file: A YAML file containing the Scenario
    pub fn from_file(mut file: File) -> Self {
        info!("Loading scenario from file");
        // Create an empty String (heap allocated) to store the file's contents
        let mut file_contents = String::new();

        // Read the file into the string
        file.read_to_string(&mut file_contents)
            .expect("There was an error reading the file");

        // Deserialize the string to a Scenario
        serde_yaml::from_slice(file_contents.as_bytes())
            .expect("There was an error parsing the file")
    }
}
