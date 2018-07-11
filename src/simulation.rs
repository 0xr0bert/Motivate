extern crate rand;

use std;
use std::collections::HashMap;
use std::io::Write;
use std::io::BufWriter;
use itertools::Itertools;
use std::time::SystemTime;
use std::fs;
use std::io;
use rand::thread_rng;
use rand::seq::sample_slice_ref;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use weather::Weather;
use scenario::Scenario;
use journey_type::JourneyType;
use agent::Agent;
use statistics;
use union_with::union_of;
use social_network;
use agent_generation;

/// Run the simulation
/// scenario: The scenario of the simulation
/// number_of_people: The number of agents to generate
/// social_connectivity: How connected the agent is to its social network
/// subculture_connectivity: How connected the agent is to its subculture
/// neighbourhood_connectivity: How connected the agent is to its neighbourhood
/// number_of_neighbour_links: The minimum number of links each agent should have in the neighbourhood network
/// days_in_habit_average: How many days should be used in the habit average
/// distributions: The distributions of the commute length
/// weather_pattern: A HashMap from day number to Weather
/// network: The social network
/// Returns: Result, nothing if successful, io:Error if output could not be written
pub fn run(id: String,
           scenario_file: File,
           total_years: u32,
           number_of_people: u32,
           social_connectivity: f32,
           subculture_connectivity: f32,
           neighbourhood_connectivity: f32,
           number_of_neighbour_links: u32,
           days_in_habit_average: u32,
           distributions: Vec<(f64, f64, f64)>,
           weather_pattern: &HashMap<u32, Weather>,
           network: HashMap<u32, Vec<u32>>,
           generate: bool) -> Result<(), io::Error> {
    // Used for monitoring running time
    let t0 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Load scenario
    let scenario = Scenario::from_file(scenario_file);

    // Create the agents
    let mut residents: Vec<Rc<RefCell<Agent>>> = set_up(
        &scenario, social_connectivity, subculture_connectivity,
        neighbourhood_connectivity, days_in_habit_average,
        number_of_neighbour_links, number_of_people, 
        distributions, network, generate, &id);

    // Report the setup running time
    let t1 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    info!("[{}] Agents created in {}s", id, t1-t0);

    // Create the output directory if it does not already exist
    fs::create_dir_all("output")?;

    // Create the output file, and write the header to it
    let mut file = BufWriter::new(fs::File::create(format!("output/output_{}.csv", id))?);
    file.write(generate_csv_header(&scenario).as_bytes())?;

    // Get the weather at day 0
    let mut weather = weather_pattern.get(&0).unwrap();

    // Write the first set of statistics to the file
    file.write(generate_csv_output(0, &weather, &scenario, &residents).as_bytes())?;

    // For each day in the simulation
    for day in 1..total_years * 365 {
        if day == scenario.intervention.day {
            intervene(&scenario, &residents)
        }

        // Only consider weekdays
        if weekday(day) {
            // Log the day to the terminal
            info!("[{}] Day: {}", id, day);

            // Get the new weather
            let new_weather = weather_pattern.get(&day).unwrap();

            // For each resident update the habit
            for resident in residents.iter_mut() {
                resident.borrow_mut().update_habit();
            }

            // For each resident, choose a travel mode
            for resident in residents.iter_mut() {
                resident.borrow_mut().choose(new_weather, weather != new_weather);
            }

            // Update the weather
            weather = new_weather;

            // Log the stats to the file
            file.write(generate_csv_output(day, &weather, &scenario, &residents).as_bytes())?;
        }
    }

    // Output the running time to the terminal
    let t2 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    info!("[{}] Total elapsed time {}s", id, t2 - t0);
    info!("[{}] Total elapsed time excl. set up {}s", id, t2 - t1);

    Ok(())

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
/// number_of_neighbour_links: The minimum number of links each agent should have in the neighbourhood network
/// number_of_people: The number of agents to generate
/// network: The social network
/// Returns: The created agents
fn set_up(scenario: &Scenario,
          social_connectivity: f32,
          subculture_connectivity: f32,
          neighbourhood_connectivity: f32,
          days_in_habit_average: u32,
          number_of_neighbour_links: u32,
          number_of_people: u32,
          distributions: Vec<(f64, f64, f64)>,
          network: HashMap<u32, Vec<u32>>,
          generate: bool,
          id: &str) -> Vec<Rc<RefCell<Agent>>> {

    let residents = if generate {
        // Create agents directory if it does not exist
        std::fs::create_dir_all("config/agents")
            .expect("Failed to create config/agents directory");


        let file = File::create(format!("config/agents/{}.yaml", id)).expect("Failed to create agents file");

        agent_generation::generate_and_save_agents(
            file, scenario, social_connectivity, subculture_connectivity, 
            neighbourhood_connectivity, days_in_habit_average, number_of_people, distributions)
    } else {
        agent_generation::load_agents(File::open(format!("config/agents/{}.yaml", id)).expect("Failed to open agents file"))
    };
    
    // Generate social network
    // self.link_agents_to_social_network(&residents, self.number_of_social_network_links);
    link_agents_from_predefined_network(&residents, network, |agent, friends| agent.social_network.append(friends));
    residents.iter()
        .for_each(|a| debug!("Social network size for agent: {}", a.borrow().social_network.len()));

    // Group agents by neighbourhood
    let neighbourhood_residents: HashMap<String, Vec<Rc<RefCell<Agent>>>> = residents
        .iter()
        .map(|x| (x.borrow().neighbourhood.id.clone(), x.clone()))
        .into_group_map();

    // For each neighbourhood create a social network
    for (_, v) in neighbourhood_residents {
        link_agents_to_neighbours(&v, number_of_neighbour_links);
    }

    // Return the created agents
    residents
}

/// Link agents to a social network
/// https://en.wikipedia.org/wiki/Barab%C3%A1si%E2%80%93Albert_model
/// agents: a slice of agents
/// n: the minimum number of links
fn link_agents_to_neighbours(agents: &[Rc<RefCell<Agent>>], n: u32) {
    // Create network of ids
    let network = social_network::generate_social_network(n, agents.len() as u32);
    // Create the neighbourhood network from the network of ids
    link_agents_from_predefined_network(agents, network, |agent, friends| agent.neighbours.append(friends));

    agents.iter()
        .for_each(|a| debug!("Neighbourhood network size for agent: {}", a.borrow().neighbours.len()))
}

/// Generate the header for the csv file
/// scenario: The scenario for this simulation
/// Returns: The header for the csv file
fn generate_csv_header(scenario: &Scenario) -> String {
    let subculture_ids: Vec<String> = scenario
        .subcultures
        .iter()
        .map(|subculture| subculture.id.clone())
        .collect();

    let neighbourhood_ids: Vec<String> = scenario
        .neighbourhoods
        .iter()
        .map(|neighbourhood| neighbourhood.id.clone())
        .collect();

    format!(
        "Day,Rain,ActiveMode,ActiveNorm,ActiveModeCounterToInactiveNorm,InactiveModeCounterToActiveNorm,LocalCommute,CityCommute,DistantCommute,{},{}\n",
        subculture_ids.join(","),
        neighbourhood_ids.join(",")
    )
}

/// Generate CSV output that conforms to the header generated in generate_csv_header(...)
/// day: The day number
/// weather: The current weather
/// scenario: The current scenario
/// agents: The agents in the network
/// Returns: The csv output for the day
fn generate_csv_output(day: u32, weather: &Weather, scenario: &Scenario, agents: &[Rc<RefCell<Agent>>]) -> String {
    let rain = if weather == &Weather::Good { 0 } else { 1 };

    let active_mode = statistics::count_active_mode(agents);
    let active_norm = statistics::count_active_norm(agents);
    let active_mode_counter_to_inactive_norm =
        statistics::count_active_mode_counter_to_inactive_norm(agents);

    let inactive_mode_counter_to_active_norm =
        statistics::count_inactive_mode_counter_to_active_norm(agents);

    let active_mode_by_commute_length = statistics::count_active_mode_by_commute_length(agents);
    let local_commute = active_mode_by_commute_length.get(&JourneyType::LocalCommute).unwrap();
    let city_commute = active_mode_by_commute_length.get(&JourneyType::CityCommute).unwrap();
    let distant_commute = active_mode_by_commute_length.get(&JourneyType::DistantCommute).unwrap();

    let active_mode_by_subculture =
        statistics::count_active_mode_by_subculture(agents);

    let active_mode_by_subculture_in_correct_order: Vec<String> = scenario
        .subcultures
        .iter()
        .map(|subculture| active_mode_by_subculture.get(subculture).unwrap_or(&0usize).to_string())
        .collect();

    let active_mode_by_neighbourhood =
        statistics::count_active_mode_by_neighbourhood(agents);

    let active_mode_by_neighbourhood_in_correct_order: Vec<String> = scenario
        .neighbourhoods
        .iter()
        .map(|neighbourhood| active_mode_by_neighbourhood.get(neighbourhood).unwrap_or(&0usize).to_string())
        .collect();

    format!(
        "{},{},{},{},{},{},{},{},{},{},{}\n",
        day,
        rain,
        active_mode,
        active_norm,
        active_mode_counter_to_inactive_norm,
        inactive_mode_counter_to_active_norm,
        local_commute,
        city_commute,
        distant_commute,
        active_mode_by_subculture_in_correct_order.join(","),
        active_mode_by_neighbourhood_in_correct_order.join(",")
    )
}

/// Link agents to a predefined social network
/// agents: a slice of agents
/// network: A map from agent id, to a vector of friend ids
/// f: Should add friends to a network of agent.
fn link_agents_from_predefined_network(
    agents: &[Rc<RefCell<Agent>>], 
    network: HashMap<u32, Vec<u32>>, 
    f: fn(agent: &mut Agent, friends: &mut Vec<Rc<RefCell<Agent>>>))
{
    network
        .iter()
        .for_each(|(&k, v)| {
            // Load the agent's freinds
            let mut friends: Vec<Rc<RefCell<Agent>>> = v
                .iter()
                .map(|&id| agents[id as usize].clone())
                .collect();
            
            // Add the agent's friends to the network of the agent
            f(&mut agents[k as usize].borrow_mut(), &mut friends);
        });
}


/// Calculate whether a given day is a weekday
/// day: The day number
/// Returns: true iff day is a weekday
fn weekday(day: u32) -> bool {
    day % 7 < 5
}

fn intervene(scenario: &Scenario, agents: &[Rc<RefCell<Agent>>]) {
    // This adds Intervention.neighbourhood_changes.increase_in_supportiveness 
    // to Neighbourhood.supportiveness
    scenario
        .intervention
        .neighbourhood_changes
        .iter()
        .for_each(
            |change| {
                let neighbourhood_to_change = scenario
                    .neighbourhoods
                    .iter()
                    .filter(|neighbourhood| neighbourhood.id == change.id)
                    .next()
                    .expect("A neighbourhood in your intervention was not found");

                let new_supportiveness = union_of(
                    &neighbourhood_to_change.supportiveness.borrow(),
                    &change.increase_in_supportiveness, 
                    |v1, v2| v1 + v2);

                neighbourhood_to_change
                    .supportiveness
                    .borrow_mut()
                    .iter_mut()
                    .for_each(|(k, v)| *v = *new_supportiveness.get(&k).unwrap());
            }
        );

    scenario
        .intervention
        .subculture_changes
        .iter()
        .for_each(
            |change| {
                let subculture_to_change = scenario
                    .subcultures
                    .iter()
                    .filter(|subculture| subculture.id == change.id)
                    .next()
                    .expect("A subculture in your intervention was not found");

                let new_desirability = union_of(
                    &subculture_to_change.desirability.borrow(),
                    &change.increase_in_desirability, 
                    |v1, v2| v1 + v2);    

                subculture_to_change
                    .desirability
                    .borrow_mut()
                    .iter_mut()
                    .for_each(|(k, v)| *v = *new_desirability.get(&k).unwrap());
            }
        );

    println!("PRE - BIKES: {}; CARS: {}",
        agents
            .iter()
            .filter(|agent| agent.borrow().owns_bike)
            .count(),
        agents
            .iter()
            .filter(|agent| agent.borrow().owns_car)
            .count()
    );
    
    if scenario.intervention.change_in_number_of_bikes > 0 {
        // Give people bikes
        let agents_without_bikes: Vec<&Rc<RefCell<Agent>>> = agents
            .iter()
            .filter(|agent| !agent.borrow().owns_bike)
            .collect();

        let mut rng = thread_rng();
        let sample = sample_slice_ref(
            &mut rng, 
            &agents_without_bikes, 
            scenario.intervention.change_in_number_of_bikes as usize);
        
        sample
            .iter()
            .for_each(|agent| agent.borrow_mut().owns_bike = true);
    } else if scenario.intervention.change_in_number_of_bikes < 0 {
        // Take away some bikes
        let agents_with_bikes: Vec<&Rc<RefCell<Agent>>> = agents
            .iter()
            .filter(|agent| agent.borrow().owns_bike)
            .collect();

        let mut rng = thread_rng();
        let decrease_in_bikes = (scenario.intervention.change_in_number_of_bikes * -1) as usize;
        let sample = sample_slice_ref(
            &mut rng, 
            &agents_with_bikes, 
            decrease_in_bikes);
        
        sample
            .iter()
            .for_each(|agent| agent.borrow_mut().owns_bike = false);
    }

    if scenario.intervention.change_in_number_of_cars > 0 {
        // Give people cars
        let agents_without_cars: Vec<&Rc<RefCell<Agent>>> = agents
            .iter()
            .filter(|agent| !agent.borrow().owns_car)
            .collect();

        let mut rng = thread_rng();
        let sample = sample_slice_ref(
            &mut rng, 
            &agents_without_cars, 
            scenario.intervention.change_in_number_of_cars as usize);
        
        sample
            .iter()
            .for_each(|agent| agent.borrow_mut().owns_car = true);
    } else if scenario.intervention.change_in_number_of_cars < 0 {
        // Take away some cars
        let agents_with_cars: Vec<&Rc<RefCell<Agent>>> = agents
            .iter()
            .filter(|agent| agent.borrow().owns_car)
            .collect();

        let mut rng = thread_rng();
        let decrease_in_cars = (scenario.intervention.change_in_number_of_cars * -1) as usize;
        let sample = sample_slice_ref(
            &mut rng, 
            &agents_with_cars, 
            decrease_in_cars);
        
        sample
            .iter()
            .for_each(|agent| agent.borrow_mut().owns_car = false);
    }

    println!("POST - BIKES: {}; CARS: {}",
        agents
            .iter()
            .filter(|agent| agent.borrow().owns_bike)
            .count(),
        agents
            .iter()
            .filter(|agent| agent.borrow().owns_car)
            .count()
    );
}
