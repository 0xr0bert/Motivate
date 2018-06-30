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
use weather::Weather;
use transport_mode::TransportMode;
use season::{Season, season};
use journey_type::JourneyType;
use neighbourhood::Neighbourhood;
use subculture::Subculture;
use scenario::Scenario;

#[derive(PartialEq)]
pub struct Agent {
    pub subculture: Rc<Subculture>,
    pub neighbourhood: Rc<Neighbourhood>,
    pub commute_length: JourneyType,
    pub perceived_effort: HashMap<JourneyType, HashMap<TransportMode, f32>>,
    pub weather_sensitivity: f32,
    pub autonomy: f32,
    pub consistency: f32,
    pub suggestibility: f32,
    pub social_connectivity: f32,
    pub subculture_connectivity: f32,
    pub neighbourhood_connectivity: f32,
    pub average_weight: f32,
    pub habit: HashMap<TransportMode, f32>,
    pub current_mode: TransportMode,
    pub last_mode: TransportMode,
    pub norm: TransportMode,
    pub social_network: Vec<Rc<RefCell<Agent>>>,
    pub neighbours: Vec<Rc<RefCell<Agent>>>
}

impl Agent {
    pub fn update_norm(&mut self) {
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

    pub fn choose(&mut self, weather: &Weather, change_in_weather: bool) {
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