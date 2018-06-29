extern crate itertools;
#[macro_use] extern crate maplit;
extern crate im;
extern crate rand;
extern crate rayon;

use std::collections::HashMap;
use std::io::Write;
use itertools::Itertools;
use std::time::SystemTime;
use std::fs;
use std::io;
use rand::distributions;
use rand::distributions::Distribution;
use rand::{thread_rng};
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;
use std::hash::Hasher;
use rayon::prelude::*;
use std::thread::JoinHandle;
use std::thread;

fn main() {
    let t0 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let total_years = 1;
    let number_of_people = 30000;
    let number_of_simulations_per_scenario = 4;
    let social_connectivity = 0.7f32;
    let subculture_connectivity = 0.5f32;
    let neighbourhood_connectivity = 0.3f32;
    let number_of_social_network_links = 10;
    let number_of_neighbour_links = 10;
    let days_in_habit_average = 30;
    let scenarios: Vec<Scenario> = vec![
        Scenario {
            id: String::from("pre intervention"),
            subcultures: vec![
                Rc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.8f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Rc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::PublicTransport => 0.8f32,
                        TransportMode::Cycle => 0.6f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Rc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.4f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32
                    }
                })
            ],
            neighbourhoods: vec!(
                Rc::new(Neighbourhood{
                    id: 0,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
                Rc::new(Neighbourhood{
                    id: 1,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.5f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 1.0f32
                    }
                }),
                Rc::new(Neighbourhood{
                    id: 2,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.2f32,
                        TransportMode::Walk => 0.6f32,
                        TransportMode::PublicTransport => 0.5f32
                    }
                }),
                Rc::new(Neighbourhood{
                    id: 3,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.2f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
            ),
        },
        Scenario {
            id: String::from("post intervention"),
            subcultures: vec![
                Rc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.8f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Rc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::PublicTransport => 0.8f32,
                        TransportMode::Cycle => 0.6f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Rc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.4f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32
                    }
                })
            ],
            neighbourhoods: vec!(
                Rc::new(Neighbourhood{
                    id: 0,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
                Rc::new(Neighbourhood{
                    id: 1,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.5f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 1.0f32
                    }
                }),
                Rc::new(Neighbourhood{
                    id: 4,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.7f32,
                        TransportMode::Cycle => 0.8f32,
                        TransportMode::Walk => 0.6f32,
                        TransportMode::PublicTransport => 0.5f32
                    }
                }),
                Rc::new(Neighbourhood{
                    id: 3,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.2f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
            ),
        }
    ];

    let mut weather_pattern: HashMap<u32, Weather> = HashMap::new();

    for i in 0..(total_years * 365) {
        let current_season = season(i);
        let random_float = rand::random::<f32>();
        if random_float > current_season.percentage_bad_weather() {
            weather_pattern.insert(i, Weather::Good);
        } else {
            weather_pattern.insert(i, Weather::Bad);
        }
    }

    let mut boroughs: Vec<Borough> = Vec::new();

    for scenario in scenarios.iter() {
        for i in 1..=number_of_simulations_per_scenario {
            boroughs.push(
                Borough {
                    id: String::from("".to_owned() + &scenario.id[..] + "-" + &i.to_string()),
                    scenario,
                    total_years,
                    number_of_people,
                    social_connectivity,
                    subculture_connectivity,
                    neighbourhood_connectivity,
                    number_of_social_network_links,
                    number_of_neighbour_links,
                    days_in_habit_average,
                    weather_pattern: &weather_pattern,
                    residents: Vec::new(),
                }
            );
        }
    }

//    for borough in boroughs.iter_mut() {
//        borough.run();
//    }
    unsafe {
        boroughs.get_unchecked_mut(0).run();
    }
//    let mut thread_handles: Vec<JoinHandle<_>>  = Vec::new();
//
//    for borough in boroughs.iter_mut() {
//        thread_handles.push(thread::spawn(|| borough.run()));
//    }
//
//    for thread in thread_handles {
//        thread.join().unwrap();
//    }

    let t1 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    println!("TOTAL RUNNING TIME: {}s", t1 - t0)

//    println!("Hello, world!");
//    let x = Season::Winter.percentage_bad_weather();
//    println!("{}", x);
}

fn season(day: u32) -> Season {
    let day_in_year = day % 365;
    if day_in_year < 59 || (day_in_year >= 334 && day_in_year < 365) {
        Season::Winter
    } else if day_in_year >= 59 && day_in_year < 151 {
        Season::Spring
    } else if day_in_year >= 151 && day_in_year < 243 {
        Season::Summer
    } else {
        Season::Autumn
    }
}

#[derive(Eq, Hash, PartialEq)]
enum Weather {
    Good,
    Bad
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum TransportMode {
    Car,
    PublicTransport,
    Cycle,
    Walk
}

#[derive(PartialEq, Copy, Clone)]
enum Season {
    Winter,
    Spring,
    Summer,
    Autumn
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum JourneyType {
    LocalCommute,
    CityCommute,
    DistantCommute
}

impl Season {
    fn percentage_bad_weather(&self) -> f32 {
        match *self {
            Season::Winter => 0.3455,
            Season::Spring => 0.3098,
            Season::Summer => 0.2696,
            Season::Autumn => 0.3275
        }
    }
}

struct Neighbourhood {
    id: u8,
    supportiveness: HashMap<TransportMode, f32>
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

#[derive(PartialEq, Clone)]
struct Subculture {
    desirability: HashMap<TransportMode, f32>
}

#[derive(PartialEq)]
struct Agent {
    subculture: Rc<Subculture>,
    neighbourhood: Rc<Neighbourhood>,
    commute_length: JourneyType,
    perceived_effort: HashMap<JourneyType, HashMap<TransportMode, f32>>,
    weather_sensitivity: f32,
    autonomy: f32,
    consistency: f32,
    suggestibility: f32,
    social_connectivity: f32,
    subculture_connectivity: f32,
    neighbourhood_connectivity: f32,
    average_weight: f32,
    habit: HashMap<TransportMode, f32>,
    current_mode: TransportMode,
    last_mode: TransportMode,
    norm: TransportMode,
    social_network: Vec<Rc<RefCell<Agent>>>,
    neighbours: Vec<Rc<RefCell<Agent>>>
}

impl Agent {
    fn update_norm(&mut self) {
        let social_vals =
            count_in_subgroup(&self.social_network, self.social_connectivity * self.suggestibility);
        let neighbour_vals =
            count_in_subgroup(&self.neighbours, self.neighbourhood_connectivity * self.suggestibility);
        let subculture_vals: HashMap<TransportMode, f32> = self.subculture.desirability.iter().map(
            |(&k, &v)| (k, v * self.subculture_connectivity * self.suggestibility)
        ).collect();
        let norm_vals: HashMap<TransportMode, f32> = {
            let mut x: HashMap<TransportMode, f32> = HashMap::new();
            x.insert(self.norm, self.autonomy);
            x
        };
        let habit_vals: HashMap<TransportMode, f32> = self.habit.iter().map(
            |(&k, &v)| (k, v * self.consistency)
        ).collect();
        let values_to_add: Vec<&HashMap<TransportMode, f32>> =
            vec![&social_vals, &neighbour_vals, &subculture_vals, &norm_vals,
                 &habit_vals, &self.neighbourhood.supportiveness];

        self.norm = values_to_add
            .into_iter()
            .fold(HashMap::new(), |acc, x| x.iter().map(
                |(k, &v)| (k, v + acc.get(k).unwrap_or(&0.0))
            ).collect())
            .into_iter()
            .fold((TransportMode::Walk, 0.0),
                  |(k0, v0): (TransportMode, f32), (&k1, v1): (&TransportMode, f32)| if v1 > v0 {(k1, v1)} else {(k0, v0)})
            .0
    }

    fn choose(&mut self, weather: &Weather, change_in_weather: bool) {
        self.last_mode = self.current_mode;
        self.habit = self.habit
            .iter()
            .map(|(&k, v)| (k, v * (1.0 - self.average_weight)
                + if k == self.last_mode {self.average_weight} else {0.0}))
            .collect();

        let norm_vals: HashMap<TransportMode, f32> = {
            let mut x: HashMap<TransportMode, f32> = HashMap::new();
            x.insert(self.norm, self.autonomy);
            x
        };
        let habit_vals: HashMap<TransportMode, f32> = self.habit.iter().map(
            |(&k, &v)| (k, v * self.consistency)
        ).collect();

        let values_to_add: Vec<&HashMap<TransportMode, f32>> =
            vec![&norm_vals, &habit_vals, &self.neighbourhood.supportiveness];

        let intermediate = values_to_add
            .iter()
            .fold(HashMap::new(), |acc, x| x.iter().map(
                |(&k, &v)| (k, v + acc.get(&k).unwrap_or(&0.0))
            ).collect());

        let effort = self.perceived_effort.get(&self.commute_length).unwrap();

        let resolve: f32 = if !change_in_weather && (self.last_mode == TransportMode::Cycle || self.last_mode == TransportMode::Walk) {
            0.1
        } else if !change_in_weather {
            -0.1
        } else {
            0.0
        };

        let weather_modifier: HashMap<TransportMode, f32> = hashmap! {
            TransportMode::Cycle => 1.0 - self.weather_sensitivity + resolve,
            TransportMode::Walk => 1.0 - self.weather_sensitivity + resolve,
            TransportMode::Car => 1.0,
            TransportMode::PublicTransport => 1.0
        };

        let values_to_multiply: Vec<&HashMap<TransportMode, f32>> =
            if weather == &Weather::Good {vec![&intermediate, effort]}
            else {vec![&intermediate, effort, &weather_modifier]};

        self.current_mode = values_to_multiply
            .into_iter()
            .fold(HashMap::new(), |acc, x| x.iter().map(
                |(k, &v)| (k, v * acc.get(k).unwrap_or(&1.0))
            ).collect())
            .into_iter()
            .fold((TransportMode::Walk, 0.0),
                  |(k0, v0): (TransportMode, f32), (&k1, v1): (&TransportMode, f32)| if v1 > v0 {(k1, v1)} else {(k0, v0)})
            .0;
    }
}

fn count_in_subgroup(x: &Vec<Rc<RefCell<Agent>>>, weight: f32) -> HashMap<TransportMode, f32> {
        let x_size = x.len() as f32;
        x
            .iter()
            .map(|x| (x.borrow().last_mode, 1))
            .into_group_map()
            .iter()
            .map(|(&k, v)| (k, (v.len() as f32) * weight / x_size))
            .collect()
}

fn weekday(day: u32) -> bool {
    day % 7 < 4
}
fn bound(lower_bound: f32, upper_bound: f32, x: f32) -> f32 {
    upper_bound.min(lower_bound.max(x))
}
fn random_normal(mean: f32, sd: f32) -> f32 {
    rand::distributions::Normal::new(mean as f64, sd as f64)
        .sample(&mut rand::thread_rng()) as f32
}

struct Scenario {
    id: String,
    subcultures: Vec<Rc<Subculture>>,
    neighbourhoods: Vec<Rc<Neighbourhood>>
}

struct Borough<'b> {
    id: String,
    scenario: &'b Scenario,
    total_years: u32,
    number_of_people: u32,
    social_connectivity: f32,
    subculture_connectivity: f32,
    neighbourhood_connectivity: f32,
    number_of_social_network_links: u32,
    number_of_neighbour_links: u32,
    days_in_habit_average: u32,
    weather_pattern: &'b HashMap<u32, Weather>,
    residents: Vec<Rc<RefCell<Agent>>>
}

impl<'b> Borough<'b> {
    fn run(&mut self) -> Result<(), io::Error>{
        let t0 = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        self.set_up();
        let t1 = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        println!("[{}] Agents created in {}s", self.id, t1-t0);

        fs::create_dir_all("output")?;

        let mut file = fs::File::create(format!("output/output_{}.csv", self.id))?;
        file.write_all(b"Day,ActiveMode,ActiveModeCounterToInactiveNorm,InactiveModeCounterToActiveNorm,ActiveNorm,Rain\n")?;

        let mut weather = self.weather_pattern.get(&0).unwrap();

        let first_stats = self.count_stats();

        file.write_all(format!("0,{},{},{},{},{}\n",
                               first_stats.0,
                               first_stats.1,
                               first_stats.2,
                               first_stats.3,
                               if weather == &Weather::Good {0} else {1})
            .as_bytes())?;

        for i in 1..self.total_years * 365 {
            if weekday(i) {
                println!("[{}] Day: {}", self.id, i);

                let new_weather = self.weather_pattern.get(&i).unwrap();
                for resident in self.residents.iter_mut() {
                    resident.borrow_mut().choose(new_weather, weather != new_weather);
                }
                weather = new_weather;

                for resident in self.residents.iter_mut() {
                    resident.borrow_mut().update_norm();
                }

                let stats = self.count_stats();
                file.write_all(format!("0,{},{},{},{},{}\n",
                                       stats.0,
                                       stats.1,
                                       stats.2,
                                       stats.3,
                                       if weather == &Weather::Good { 0 } else { 1 })
                    .as_bytes())?;
            }
        }

        let t2 = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        println!("[{}] Total elapsed time {}s", self.id, t2 - t0);
        println!("[{}] Total elapsed time excl. set up {}s", self.id, t2 - t1);

        Ok(())

    }

    fn set_up(&mut self) {
        for _ in 0..self.number_of_people {
            let agent: Agent = self.create_unlinked_agent();
            self.residents.push(Rc::new(RefCell::new(agent)));
        }

        self.link_agents_to_social_network(&self.residents, self.number_of_social_network_links);

        let neighbourhood_residents: HashMap<Rc<Neighbourhood>, Vec<Rc<RefCell<Agent>>>> = self.residents
            .iter()
            .map(|x| (x.borrow().neighbourhood.clone(), x.clone()))
            .into_group_map();

        for (k, v) in neighbourhood_residents {
            self.link_agents_to_neighbours(&v, self.number_of_neighbour_links);
        }

    }

    fn create_unlinked_agent(&self) -> Agent {
        let perceived_effort: HashMap<JourneyType, HashMap<TransportMode, f32>> = hashmap!{
            JourneyType::LocalCommute => hashmap!{
                TransportMode::Walk => bound(0.0, 1.0, random_normal(0.2, 0.3)),
                TransportMode::Cycle => bound(0.0, 1.0, random_normal(0.2, 0.3)),
                TransportMode::PublicTransport => bound(0.0, 1.0, random_normal(0.2, 0.1)),
                TransportMode::Car => bound(0.0, 1.0, random_normal(0.2, 0.1))
            },
            JourneyType::CityCommute => hashmap!{
                TransportMode::Walk => bound(0.0, 1.0, random_normal(0.7, 0.3)),
                TransportMode::Cycle => bound(0.0, 1.0, random_normal(0.5, 0.3)),
                TransportMode::PublicTransport => bound(0.0, 1.0, random_normal(0.2, 0.1)),
                TransportMode::Car => bound(0.0, 1.0, random_normal(0.2, 0.1))
            },
            JourneyType::DistantCommute => hashmap!{
                TransportMode::Walk => 1.0,
                TransportMode::Cycle => 1.0,
                TransportMode::PublicTransport => bound(0.0, 1.0, random_normal(0.2, 0.1)),
                TransportMode::Car => bound(0.0, 1.0, random_normal(0.2, 0.1))
            }
        };

        let subculture = self.choose_subculture();
        let neighbourhood = self.choose_neighbourhood();
        let commute_length = self.choose_journey_type();

        let weather_sensitivity = 0.9;
        let autonomy = rand::random::<f32>();
        let consistency = rand::random::<f32>();

        let suggestibility = random_normal(1.0, 0.25);

        let current_mode: TransportMode = self.choose_initial_norm_and_habit(
            subculture.clone(), self.subculture_connectivity, suggestibility,
            commute_length, &perceived_effort, neighbourhood.clone()
        );
        let norm = current_mode;
        let last_mode = current_mode;

        Agent {
            subculture,
            neighbourhood,
            commute_length,
            perceived_effort,
            weather_sensitivity,
            autonomy,
            consistency,
            suggestibility,
            social_connectivity: self.social_connectivity,
            subculture_connectivity: self.subculture_connectivity,
            neighbourhood_connectivity: self.neighbourhood_connectivity,
            average_weight: 2.0 / (self.days_in_habit_average as f32 + 1.0),
            habit: hashmap!{current_mode => 1.0f32},
            current_mode,
            last_mode,
            norm,
            social_network: Vec::new(),
            neighbours: Vec::new(),
        }
    }

    fn choose_subculture(&self) -> Rc<Subculture> {
        let mut weighted: Vec<distributions::Weighted<Rc<Subculture>>> = self.scenario.subcultures
            .iter()
            .map(|s: &Rc<Subculture>| distributions::Weighted {weight: 1, item: Rc::clone(s)})
            .collect();
        let weighted_choice = distributions::WeightedChoice::new(&mut weighted);
        weighted_choice.sample(&mut thread_rng())
    }

    fn choose_neighbourhood(&self) -> Rc<Neighbourhood> {
        let mut weighted: Vec<distributions::Weighted<Rc<Neighbourhood>>> = self.scenario.neighbourhoods
            .iter()
            .map(|s: &Rc<Neighbourhood>| distributions::Weighted {weight: 1, item: Rc::clone(s)})
            .collect();
        let weighted_choice = distributions::WeightedChoice::new(&mut weighted);
        weighted_choice.sample(&mut thread_rng())
    }

    fn choose_journey_type(&self) -> JourneyType {
        let x = rand::random::<f32>();
        if x <= 0.33 {
            JourneyType::LocalCommute
        } else if x <= 0.66 {
            JourneyType::CityCommute
        } else {
            JourneyType::DistantCommute
        }
    }

    fn choose_initial_norm_and_habit(&self,
                                     subculture: Rc<Subculture>,
                                     subculture_connectivity: f32,
                                     suggestibility: f32,
                                     commute_length: JourneyType,
                                     perceived_effort: &HashMap<JourneyType, HashMap<TransportMode, f32>>,
                                     neighbourhood: Rc<Neighbourhood>
    ) -> TransportMode {
        let subculture_weight = subculture_connectivity * suggestibility;
        let subculture_desirability_weighted: HashMap<TransportMode, f32> = subculture
            .desirability
            .iter()
            .map(|(&k, v)|(k, v * subculture_weight))
            .collect();

        let effort_for_journey_inverted: HashMap<TransportMode, f32> = perceived_effort
            .get(&commute_length)
            .unwrap()
            .iter()
            .map(|(&k, v)| (k, 1.0 - v))
            .collect();

        let values_to_multiply: Vec<&HashMap<TransportMode, f32>> =
            vec![&subculture_desirability_weighted,
                 &effort_for_journey_inverted,
                 &neighbourhood.supportiveness];

        values_to_multiply
            .into_iter()
            .fold(HashMap::new(), |acc, x| x.iter().map(
                |(k, &v)| (k, v * acc.get(k).unwrap_or(&1.0))
            ).collect())
            .into_iter()
            .fold((TransportMode::Walk, 0.0),
                  |(k0, v0): (TransportMode, f32), (&k1, v1): (&TransportMode, f32)| if v1 > v0 {(k1, v1)} else {(k0, v0)})
            .0
    }

    fn link_agents_to_social_network(&self, agents: &Vec<Rc<RefCell<Agent>>>, n: u32) {
        let mut linked_agents: Vec<Rc<RefCell<Agent>>> = Vec::new();

        for rc in agents.iter() {
            if linked_agents.len() < n as usize {
                for linked_agent in linked_agents.iter() {
                    rc.borrow_mut().social_network.push(linked_agent.clone());
                   linked_agent.borrow_mut().social_network.push(rc.clone())
                }
                linked_agents.push(rc.clone())
            } else {
                let mut weighted: Vec<distributions::Weighted<Rc<RefCell<Agent>>>> = linked_agents
                    .iter()
                    .map(|a| distributions::Weighted {weight: a.borrow().social_network.len() as u32, item: a.clone()})
                    .collect();
                let weighted_choice = distributions::WeightedChoice::new(&mut weighted);
                for _ in 0..n {
                    let friend = &mut weighted_choice.sample(&mut thread_rng());
                    friend.borrow_mut().social_network.push(rc.clone());
                    rc.borrow_mut().social_network.push(friend.clone());
                }

                linked_agents.push(rc.clone());
            }
        }
    }

    fn link_agents_to_neighbours(&self, agents: &Vec<Rc<RefCell<Agent>>>, n: u32) {
        let mut linked_agents: Vec<Rc<RefCell<Agent>>> = Vec::new();

        for rc in agents.iter() {
            if linked_agents.len() < n as usize {
                for linked_agent in linked_agents.iter() {
                    rc.borrow_mut().neighbours.push(linked_agent.clone());
                    linked_agent.borrow_mut().neighbours.push(rc.clone())
                }
                linked_agents.push(rc.clone())
            } else {
                let mut weighted: Vec<distributions::Weighted<Rc<RefCell<Agent>>>> = linked_agents
                    .iter()
                    .map(|a| distributions::Weighted {weight: a.borrow().neighbours.len() as u32, item: a.clone()})
                    .collect();
                let weighted_choice = distributions::WeightedChoice::new(&mut weighted);
                for _ in 0..n {
                    let friend = &mut weighted_choice.sample(&mut thread_rng());
                    friend.borrow_mut().neighbours.push(rc.clone());
                    rc.borrow_mut().neighbours.push(friend.clone());
                }

                linked_agents.push(rc.clone());
            }
        }
    }

    fn count_stats(&self) -> (usize, usize, usize, usize) {
        (self.residents.iter().filter(|&a| a.borrow().current_mode == TransportMode::Walk || a.borrow().current_mode == TransportMode::Cycle).count(),
         self.residents.iter().filter(|&a| (a.borrow().current_mode == TransportMode::Walk || a.borrow().current_mode == TransportMode::Cycle) && (a.borrow().norm != TransportMode::Walk && a.borrow().norm != TransportMode::Cycle)).count(),
         self.residents.iter().filter(|&a| (a.borrow().current_mode != TransportMode::Walk && a.borrow().current_mode != TransportMode::Cycle) && (a.borrow().norm == TransportMode::Walk || a.borrow().norm == TransportMode::Cycle)).count(),
         self.residents.iter().filter(|&a| a.borrow().norm == TransportMode::Walk || a.borrow().norm == TransportMode::Cycle).count()
        )
    }
}

//const SUBCULTURE_A: Subculture = Subculture {
//    desirability: hashmap!{
//        TransportMode::Car => 0.8,
//        TransportMode::PublicTransport => 0.5,
//        TransportMode::Cycle => 0.9,
//        TransportMode::Walk => 0.7,
//    }
//};
//
//const SUBCULTURE_B: Subculture = Subculture {
//    desirability: hashmap!{
//        TransportMode::Car => 0.9,
//        TransportMode::PublicTransport => 0.8,
//        TransportMode::Cycle => 0.6,
//        TransportMode::Walk => 0.7,
//    }
//};
//
//const SUBCULTURE_C: Subculture = Subculture {
//    desirability: hashmap!{
//        TransportMode::Car => 0.4,
//        TransportMode::PublicTransport => 0.5,
//        TransportMode::Cycle => 0.9,
//        TransportMode::Walk => 0.9,
//    }
//};