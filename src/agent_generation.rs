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
use union_with::union_of;
use std::cmp;
use gaussian;
use serde_yaml;

/// Generates unlinked agents and saves them to the supplied file
/// file: The file to save to
/// scenario: The scenario of the simulation
/// social_connectivity: How connected the agent is to its social network
/// subculture_connectivity: How connected the agent is to its subculture
/// neighbourhood_connectivity: How connected the agent is to its neighbourhood
/// days_in_habit_average: How many days should be used in the habit average
/// number_of_people: The number of agents to generate
/// distributions: The distributions of the commute length
/// Returns: The created agents
pub fn generate_and_save_agents(
    mut file: File,
    scenario: &Scenario,
    social_connectivity: f32,
    subculture_connectivity: f32,
    neighbourhood_connectivity: f32,
    days_in_habit_average: u32,
    number_of_people: u32,
    distributions: Vec<(f64, f64, f64)>) -> Vec<Rc<RefCell<Agent>>>
{
    let agents = set_up(
        scenario, social_connectivity, subculture_connectivity, 
        neighbourhood_connectivity, days_in_habit_average, number_of_people, 
        distributions);

    let agents_string = serde_yaml::to_string(&agents).unwrap();
    file.write_all(agents_string.as_bytes()).unwrap();
    agents
}

pub fn load_agents(mut file: File) -> Vec<Rc<RefCell<Agent>>> {
    info!("Loading agents from file");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)
        .expect("There was an error reading the file");

    serde_yaml::from_slice(file_contents.as_bytes())
        .expect("There was an error parsing the file")
}

/// Create the agents
/// Returns a set of Rc pointers, storing RefCells of Agents
/// Rc are reference counter pointers, that are store immutable data,
/// RefCells are immutable but have mutable contents
/// Meaning that Agents are mutable
/// scenario: The scenario of the simulation
/// social_connectivity: How connected the agent is to its social network
/// subculture_connectivity: How connected the agent is to its subculture
/// neighbourhood_connectivity: How connected the agent is to its neighbourhood
/// days_in_habit_average: How many days should be used in the habit average
/// number_of_people: The number of agents to generate
/// distributions: The distributions of the commute length
/// Returns: The created agents
fn set_up(scenario: &Scenario,
          social_connectivity: f32,
          subculture_connectivity: f32,
          neighbourhood_connectivity: f32,
          days_in_habit_average: u32,
          number_of_people: u32,
          distributions: Vec<(f64, f64, f64)>) -> Vec<Rc<RefCell<Agent>>> {
    // Create an empty vec to store agents
    let mut residents: Vec<Rc<RefCell<Agent>>> = Vec::new();
    // Create self.number_of_people unlinked agents
    for _ in 0..number_of_people {
        let agent: Agent = create_unlinked_agent(scenario, social_connectivity,
            subculture_connectivity, neighbourhood_connectivity, days_in_habit_average);

        residents.push(Rc::new(RefCell::new(agent)));
    }

    // This needs to be in a different scope, so that residents can be borrowed as immutable
    // later on, when called at link_agents_from_predefined_network. Rust allows multiple
    // immutable borrows, or one mutable borrow at any given time, if this is not in a different
    // scope then &residents and &mut residents are in scope at the same time.
    {
        // Give people cars
        let mut rng = thread_rng();
        let sample = sample_slice_ref(&mut rng, &residents, scenario.number_of_cars as usize);
        sample
            .iter()
            .for_each(|agent| agent.borrow_mut().owns_car = true);
    }
    {
        // Give people bikes
        let mut rng = thread_rng();
        let sample = sample_slice_ref(&mut rng, &residents, scenario.number_of_cars as usize);
        sample
            .iter()
            .for_each(|agent| agent.borrow_mut().owns_bike = true);
    }

    let commute_distances: Vec<f64> = gaussian::get_samples_from_gmm(number_of_people as usize, distributions)
        .into_iter()
        .map(|x| x.abs())
        .collect();

    // Assign commute distances
    commute_distances
        .iter()
        .zip(residents.iter())
        .for_each(|(distance, agent)| {
            let agent_ref = &mut agent.borrow_mut();

            agent_ref.commute_length_continuous = *distance;

            // Assign categorical distance
            if *distance < 4241.0 {
                agent_ref.commute_length = JourneyType::LocalCommute
            } else if *distance < 19457.0 {
                agent_ref.commute_length = JourneyType::CityCommute
            } else {
                agent_ref.commute_length = JourneyType::DistantCommute
            }
        });
    
    for agent in residents.iter() {
        let mut borrowed_agent = agent.borrow_mut();
        let new_mode = choose_initial_norm_and_habit(
            &borrowed_agent.subculture, 
            borrowed_agent.social_connectivity, 
            borrowed_agent.suggestibility, 
            borrowed_agent.commute_length, 
            &borrowed_agent.neighbourhood, 
            borrowed_agent.owns_car, 
            borrowed_agent.owns_bike);
        
        borrowed_agent.current_mode = new_mode;
        borrowed_agent.habit = hashmap!{new_mode => 1.0f32};
        borrowed_agent.norm = new_mode;
    }

    residents

}

/// Create an unlinked agent, that does not own a bike or a car, without a current mode, and without a commute length
/// scenario: The scenario of the simulation
/// social_connectivity: How connected the agent is to its social network
/// subculture_connectivity: How connected the agent is to its subculture
/// neighbourhood_connectivity: How connected the agent is to its neighbourhood
/// days_in_habit_average: How many days should be used in the habit average
/// Returns: The created agent
fn create_unlinked_agent(scenario: &Scenario,
                         social_connectivity: f32,
                         subculture_connectivity: f32,
                         neighbourhood_connectivity: f32,
                         days_in_habit_average: u32) -> Agent {
    // Choose a subculture and, neighbourhood
    let subculture = choose_subculture(scenario);
    let neighbourhood = choose_neighbourhood(scenario);

    // Weather sensitivity is currently fixed
    let weather_sensitivity = 0.9f32;
    // TODO: Should consistency be used
    let consistency = rand::random::<f32>();

    // TODO: Should suggestibility be used
    let suggestibility = random_normal(1.0, 0.25);

    // Use a placeholder transport mode
    let current_mode: TransportMode = TransportMode::PublicTransport;
    let norm = current_mode;
    let last_mode = current_mode;

    // Create and return the agent
    Agent {
        subculture,
        neighbourhood,
        commute_length: JourneyType::LocalCommute,
        commute_length_continuous: 0.0,
        weather_sensitivity,
        consistency,
        suggestibility,
        social_connectivity: social_connectivity,
        subculture_connectivity: subculture_connectivity,
        neighbourhood_connectivity: neighbourhood_connectivity,
        average_weight: 2.0 / (days_in_habit_average as f32 + 1.0),
        habit: hashmap!{current_mode => 1.0f32},
        current_mode,
        last_mode,
        norm,
        owns_bike: false,
        owns_car: false,
        social_network: Vec::new(),
        neighbours: Vec::new(),
    }
}

/// Choose a random neighbourhood, equal chance of each
/// scenario: The scenario of the simulation
/// Returns: The chosen neighbourhood
fn choose_neighbourhood(scenario: &Scenario) -> Rc<Neighbourhood> {
    let mut weighted: Vec<distributions::Weighted<Rc<Neighbourhood>>> = scenario.neighbourhoods
        .iter()
        .map(|s: &Rc<Neighbourhood>| distributions::Weighted {weight: 1, item: Rc::clone(s)})
        .collect();
    let weighted_choice = distributions::WeightedChoice::new(&mut weighted);
    weighted_choice.sample(&mut thread_rng())
}

/// Choose a random subculture, equal chance of each
/// scenario: The scenario of the simulation
/// Returns: The chosen subculture
fn choose_subculture(scenario: &Scenario) -> Rc<Subculture> {
    let mut weighted: Vec<distributions::Weighted<Rc<Subculture>>> = scenario.subcultures
        .iter()
        .map(|s: &Rc<Subculture>| distributions::Weighted {weight: 1, item: Rc::clone(s)})
        .collect();
    let weighted_choice = distributions::WeightedChoice::new(&mut weighted);
    weighted_choice.sample(&mut thread_rng())
}

/// Gets a random number from a normal distribution
/// mean: The mean of the normal distribution
/// sd: The standard deviation of the normal distributions
/// Returns: A random float from N(mean, sd)
fn random_normal(mean: f32, sd: f32) -> f32 {
    rand::distributions::Normal::new(mean as f64, sd as f64)
        .sample(&mut rand::thread_rng()) as f32
}

/// Choose an initial norm and habit
/// subculture: The subculture of the agent
/// subculture_connectivity: How connected the agent is to the subculture
/// _suggestibility: Not currently used
/// commute_length: The distance of the agent's commute
/// neighbourhood: The neighbourhood of the agent
/// Returns: The chosen transport mode
fn choose_initial_norm_and_habit(subculture: &Rc<Subculture>,
                                 subculture_connectivity: f32,
                                 _sugestibility: f32,
                                 commute_length: JourneyType,
                                 neighbourhood: &Rc<Neighbourhood>,
                                 owns_car: bool,
                                 owns_bike: bool
) -> TransportMode {
    let subculture_weight = subculture_connectivity;
    let mut initial_budget: HashMap<TransportMode, f32> = subculture
        .desirability
        .borrow()
        .iter()
        .map(|(&k, v)|(k, v * subculture_weight))
        .collect();

    // Find the key-value-pair in initial_budget with the highest value, and store the value
    let max: f32 = *initial_budget
        .iter()
        .max_by(|v1, v2| v1.1.partial_cmp(&v2.1).unwrap_or(cmp::Ordering::Equal))
        .unwrap()
        .1;

    // Make it so the the max mode has a budget of 1, therefore at least one mode is always possible
    initial_budget = initial_budget
        .into_iter()
        .map(|(k, v)| (k, v / max))
        .collect();

    // Take car / bike ownership into account
    let ownership_modifier = hashmap! {
        TransportMode::Car => if owns_car {1.0f32} else {0.0f32},
        TransportMode::Cycle => if owns_bike {1.0f32} else {0.0f32},
    };

    initial_budget = union_of(&initial_budget, &ownership_modifier, |v1, v2| v1 * v2);


    let commute_length_cost = commute_length.cost();

    // Take the supportiveness for each mode, away from 1, so a lower supportiveness = higher cost
    let neighbourhood_vals: HashMap<TransportMode, f32> = neighbourhood
        .supportiveness
        .borrow()
        .iter()
        .map(|(&k, v)| (k, 1.0f32 - v))
        .collect();

    let values_to_average: Vec<&HashMap<TransportMode, f32>> =
        vec![&neighbourhood_vals,
             &commute_length_cost];

    let initial_cost: HashMap<TransportMode, f32> = values_to_average
        .iter()
        .fold(HashMap::new(), |acc, x|
            union_of(&acc, x, |v1, v2| v1 + v2)
        )
        .into_iter()
        .map(|(k, v)| (k, v / (values_to_average.len() as f32)))
        .collect();

    // Filter out values where the budget is not greater than or equal to the cost
    // Calculate the difference between the budget and the cost
    // Get the maximum key-value pair (by value)
    // Set the current_mode equal to the key
    initial_budget
        .iter()
        .filter_map(|(&k, &v)| {
            let cost_val: f32 = *initial_cost.get(&k).unwrap_or(&9999.0f32);
            if v >= cost_val {
                Some((k, v - cost_val))
            } else {
                None
            }
        })
        .fold((TransportMode::Walk, -0.1),
              |(k0, v0): (TransportMode, f32), (k1, v1): (TransportMode, f32)| if v1 > v0 { (k1, v1) } else { (k0, v0) })
        .0
}