extern crate itertools;
#[macro_use] extern crate maplit;
extern crate im;
extern crate rand;
extern crate rayon;

mod weather;
mod transport_mode;
mod season;
mod journey_type;
mod neighbourhood;
mod subculture;
mod scenario;
mod agent;
mod borough;

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
use borough::Borough;
use weather::Weather;
use transport_mode::TransportMode;
use season::{Season, season};
use journey_type::JourneyType;
use neighbourhood::Neighbourhood;
use subculture::Subculture;
use scenario::Scenario;


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
                Arc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.8f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Arc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::PublicTransport => 0.8f32,
                        TransportMode::Cycle => 0.6f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Arc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.4f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32
                    }
                })
            ],
            neighbourhoods: vec!(
                Arc::new(Neighbourhood{
                    id: 0,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: 1,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.5f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 1.0f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: 2,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.2f32,
                        TransportMode::Walk => 0.6f32,
                        TransportMode::PublicTransport => 0.5f32
                    }
                }),
                Arc::new(Neighbourhood{
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
                Arc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.8f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Arc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::PublicTransport => 0.8f32,
                        TransportMode::Cycle => 0.6f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Arc::new(Subculture {
                    desirability: hashmap!{
                        TransportMode::Car => 0.4f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32
                    }
                })
            ],
            neighbourhoods: vec!(
                Arc::new(Neighbourhood{
                    id: 0,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: 1,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.5f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 1.0f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: 4,
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.7f32,
                        TransportMode::Cycle => 0.8f32,
                        TransportMode::Walk => 0.6f32,
                        TransportMode::PublicTransport => 0.5f32
                    }
                }),
                Arc::new(Neighbourhood{
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
                    scenario: scenario.clone(),
                    total_years,
                    number_of_people,
                    social_connectivity,
                    subculture_connectivity,
                    neighbourhood_connectivity,
                    number_of_social_network_links,
                    number_of_neighbour_links,
                    days_in_habit_average,
                    weather_pattern: weather_pattern.clone()
                }
            );
        }
    }

//    for borough in boroughs.iter_mut() {
//        borough.run();
//    }
//    unsafe {
//        boroughs.get_unchecked_mut(0).run();
//    }
//    let mut thread_handles: Vec<JoinHandle<_>>  = Vec::new();
//
//    for borough in boroughs.iter_mut() {
//        thread_handles.push(thread::spawn(move || borough.run()));
//    }
//
//    for thread in thread_handles {
//        thread.join().unwrap();
////    }
    boroughs.par_iter_mut().for_each(|b| match b.run() {
        _ => ()
    });

    let t1 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    println!("TOTAL RUNNING TIME: {}s", t1 - t0)

//    println!("Hello, world!");
//    let x = Season::Winter.percentage_bad_weather();
//    println!("{}", x);
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