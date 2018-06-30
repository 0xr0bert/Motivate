extern crate rand;

use std;
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
use std::sync::Arc;
use std::cell::RefCell;
use std::hash::Hash;
use std::hash::Hasher;
use rayon::prelude::*;
use std::thread::JoinHandle;
use std::thread;
use weather::Weather;
use transport_mode::TransportMode;
use season::{Season, season};
use journey_type::JourneyType;
use neighbourhood::Neighbourhood;
use subculture::Subculture;
use scenario::Scenario;
use agent::Agent;

pub struct Borough {
    pub id: String,
    pub scenario: Scenario,
    pub total_years: u32,
    pub number_of_people: u32,
    pub social_connectivity: f32,
    pub subculture_connectivity: f32,
    pub neighbourhood_connectivity: f32,
    pub number_of_social_network_links: u32,
    pub number_of_neighbour_links: u32,
    pub days_in_habit_average: u32,
    pub weather_pattern: HashMap<u32, Weather>
}

impl Borough {
    pub fn run(&mut self) -> Result<(), io::Error>{
        let t0 = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let mut residents: Vec<Rc<RefCell<Agent>>> = Vec::new();
        self.set_up(&mut residents);
        let t1 = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        println!("[{}] Agents created in {}s", self.id, t1-t0);

        fs::create_dir_all("output")?;

        let mut file = fs::File::create(format!("output/output_{}.csv", self.id))?;
        file.write_all(b"Day,ActiveMode,ActiveModeCounterToInactiveNorm,InactiveModeCounterToActiveNorm,ActiveNorm,Rain\n")?;

        let mut weather = self.weather_pattern.get(&0).unwrap();

        let first_stats = self.count_stats(&residents);

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
                for resident in residents.iter_mut() {
                    resident.borrow_mut().choose(new_weather, weather != new_weather);
                }
                weather = new_weather;

                for resident in residents.iter_mut() {
                    resident.borrow_mut().update_norm();
                }

                let stats = self.count_stats(&residents);
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

    fn set_up(&mut self, residents: &mut Vec<Rc<RefCell<Agent>>>) {
        for _ in 0..self.number_of_people {
            let agent: Agent = self.create_unlinked_agent();
            residents.push(Rc::new(RefCell::new(agent)));
        }

        self.link_agents_to_social_network(residents, self.number_of_social_network_links);

        let neighbourhood_residents: HashMap<u8, Vec<Rc<RefCell<Agent>>>> = residents
            .iter()
            .map(|x| (x.borrow().neighbourhood.id, x.clone()))
            .into_group_map();

        for (_, v) in neighbourhood_residents {
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
            &subculture, self.subculture_connectivity, suggestibility,
            commute_length, &perceived_effort, &neighbourhood
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

    fn choose_subculture(&self) -> Arc<Subculture> {
        let mut weighted: Vec<distributions::Weighted<Arc<Subculture>>> = self.scenario.subcultures
            .iter()
            .map(|s: &Arc<Subculture>| distributions::Weighted {weight: 1, item: Arc::clone(s)})
            .collect();
        let weighted_choice = distributions::WeightedChoice::new(&mut weighted);
        weighted_choice.sample(&mut thread_rng())
    }

    fn choose_neighbourhood(&self) -> Arc<Neighbourhood> {
        let mut weighted: Vec<distributions::Weighted<Arc<Neighbourhood>>> = self.scenario.neighbourhoods
            .iter()
            .map(|s: &Arc<Neighbourhood>| distributions::Weighted {weight: 1, item: Arc::clone(s)})
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
                                     subculture: &Arc<Subculture>,
                                     subculture_connectivity: f32,
                                     suggestibility: f32,
                                     commute_length: JourneyType,
                                     perceived_effort: &HashMap<JourneyType, HashMap<TransportMode, f32>>,
                                     neighbourhood: &Arc<Neighbourhood>
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
            }
            linked_agents.push(rc.clone());
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

    fn count_stats(&self, residents: &Vec<Rc<RefCell<Agent>>>) -> (usize, usize, usize, usize) {
        (residents.iter().filter(|&a| a.borrow().current_mode == TransportMode::Walk || a.borrow().current_mode == TransportMode::Cycle).count(),
         residents.iter().filter(|&a| (a.borrow().current_mode == TransportMode::Walk || a.borrow().current_mode == TransportMode::Cycle) && (a.borrow().norm != TransportMode::Walk && a.borrow().norm != TransportMode::Cycle)).count(),
         residents.iter().filter(|&a| (a.borrow().current_mode != TransportMode::Walk && a.borrow().current_mode != TransportMode::Cycle) && (a.borrow().norm == TransportMode::Walk || a.borrow().norm == TransportMode::Cycle)).count(),
         residents.iter().filter(|&a| a.borrow().norm == TransportMode::Walk || a.borrow().norm == TransportMode::Cycle).count()
        )
    }
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
