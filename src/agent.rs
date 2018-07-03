use union_with::union_of;
use std::collections::HashMap;
use itertools::Itertools;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use weather::Weather;
use transport_mode::TransportMode;
use journey_type::JourneyType;
use neighbourhood::Neighbourhood;
use subculture::Subculture;
use union_with::union_of;

#[derive(PartialEq)]
pub struct Agent {
    pub subculture: Arc<Subculture>,
    pub neighbourhood: Arc<Neighbourhood>,
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
    fn calculate_mode_budget(&mut self) -> HashMap<TransportMode, f32> {
        let social_vals =
            count_in_subgroup(&self.social_network, self.social_connectivity);
        let neighbour_vals =
            count_in_subgroup(&self.neighbours, self.neighbourhood_connectivity);
        let subculture_vals: HashMap<TransportMode, f32> = self.subculture.desirability.iter().map(
            |(&k, &v)| (k, v * self.subculture_connectivity)
        ).collect();

        let values_to_add: Vec<&HashMap<TransportMode, f32>> =
            vec![&social_vals, &neighbour_vals, &subculture_vals];

        self.norm = values_to_add
                .iter()
                .fold(HashMap::new(), |acc, x|
                    union_of(&acc, x, |v1, v2| v1 + v2)
                )
                .iter()
                .fold((TransportMode::Walk, 0.0),
                      |(k0, v0): (TransportMode, f32), (&k1, &v1): (&TransportMode, &f32)| if v1 > v0 { (k1, v1) } else { (k0, v0) })
                .0;

        let habit_vals: HashMap<TransportMode, f32> = self.habit.iter().map(
            |(&k, &v)| (k, v)
        ).collect();

        let values_to_average: Vec<&HashMap<TransportMode, f32>> =
            vec![&social_vals, &neighbour_vals, &subculture_vals, &habit_vals];

        let intermediate: HashMap<TransportMode, f32> = values_to_average
            .iter()
            .fold(HashMap::new(), |acc, x|
                union_of(&acc, x, |v1, v2| v1 + v2)
            )
            .into_iter()
            .map(|(k, v)| (k, v / (values_to_average.len() as f32)))
            .collect();


        let max = intermediate
            .iter()
            .fold((TransportMode::Walk, 0.0),
                  |(k0, v0): (TransportMode, f32), (&k1, &v1): (&TransportMode, &f32)| if v1 > v0 { (k1, v1) } else { (k0, v0) })
            .1;

        // Make it so the the max mode has a budget of 1, therefore at least one mode is always possible
        let budget: HashMap<TransportMode, f32> = intermediate
            .into_iter()
            .map(|(k, v)| (k, v / max))
            .collect();
        //
        // self.norm = budget
        //     .iter()
        //     .fold((TransportMode::Walk, 0.0),
        //           |(k0, v0): (TransportMode, f32), (&k1, &v1): (&TransportMode, &f32)| if v1 > v0 { (k1, v1) } else { (k0, v0) })
        //     .0;

        budget
    }

    fn calculate_cost(&self, weather: &Weather, change_in_weather: bool) -> HashMap<TransportMode, f32> {
        let neighbourhood_vals: HashMap<TransportMode, f32> = self.neighbourhood
            .supportiveness
            .iter()
            .map(|(&k, v)| (k, 1.0f32 - v))
            .collect();

        let commute_cost = self.commute_length.cost();

        let values_to_average =
            vec!(&commute_cost, &neighbourhood_vals);

        let mut intermediate: HashMap<TransportMode, f32> = values_to_average
            .iter()
            .fold(HashMap::new(), |acc, x| union_of(&acc, x, |v1, v2| v1 + v2))
            .iter()
            .map(|(&k, v)| (k, v / (values_to_average.len()) as f32))
            .collect();

        if weather == &Weather::Bad {
            let resolve = if !change_in_weather &&
                (self.last_mode == TransportMode::Walk || self.last_mode == TransportMode::Cycle) {
                -0.1f32
            } else if !change_in_weather {
                0.1f32
            } else {
                0.0f32
            };

            let weather_modifier = hashmap!{
                TransportMode::Cycle => 1.0f32 + self.weather_sensitivity + resolve,
                TransportMode::Walk => 1.0f32 + self.weather_sensitivity + resolve
            };

            let values_to_multiply = vec![intermediate.clone(), weather_modifier];

            intermediate = values_to_multiply
                .iter()
                .fold(HashMap::new(), |acc, x| union_of(&acc, x, |v1, v2| v1 * v2));
        }

        let intermediate_min = intermediate
            .iter()
            .fold((TransportMode::Walk, 0.0),
                  |(k0, v0): (TransportMode, f32), (&k1, &v1): (&TransportMode, &f32)| if v1 < v0 { (k1, v1) } else { (k0, v0) })
            .1;

        if intermediate_min > 1.0 {
            intermediate
                .into_iter()
                .map(|(k, v)| (k, v / intermediate_min))
                .collect()
        } else {
            intermediate
        }
    }

    pub fn update_habit(&mut self) {
        self.last_mode = self.current_mode;
        let last_mode_map: HashMap<TransportMode, f32> = hashmap!{
            self.last_mode => self.average_weight
        };
        let old_habit: HashMap<TransportMode, f32> = self.habit
            .iter()
            .map(|(&k, v)| (k, v * (1.0f32 - self.average_weight)))
            .collect();
        let values_to_add = vec![old_habit, last_mode_map];

        self.habit = values_to_add
            .iter()
            .fold(HashMap::new(), |acc, x| union_of(&acc, x, |v1, v2| v1 + v2));
    }

    pub fn choose(&mut self, weather: &Weather, change_in_weather: bool) {
        let budget = self.calculate_mode_budget();
        let cost = self.calculate_cost(weather, change_in_weather);
        let diff: HashMap<TransportMode, f32> = budget
            .iter()
            .filter_map(|(&k, &v)| {
                let cost_val: f32 = *cost.get(&k).unwrap_or(&99999999.0f32);
                if v >= cost_val {
                    Some((k, v - cost_val))
                } else {
                    None
                }
            })
            .collect();
        // println!("{}", diff.len());
        self.current_mode = diff
            .into_iter()
            .fold((TransportMode::Walk, 0.0),
                  |(k0, v0): (TransportMode, f32), (k1, v1): (TransportMode, f32)| if v1 > v0 { (k1, v1) } else { (k0, v0) })
            .0;

        // if weather == &Weather::Good && change_in_weather {
        //     println!("{},{}", budget.len(), cost.len());
        //     // print!("(\n    ");
        //     // budget.iter().for_each(|(k, v)| print!("({}, {})", k.to_string(), *v ));
        //     // print!("\n    ");
        //     // cost.iter().for_each(|(k, v)| print!("({}, {})", k.to_string(), *v ));
        //     // println!(")");
        // }
    }
}

fn count_in_subgroup(x: &[Rc<RefCell<Agent>>], weight: f32) -> HashMap<TransportMode, f32> {
    let x_size = x.len() as f32;
    x
        .iter()
        .map(|x| (x.borrow().last_mode, 1))
        .into_group_map()
        .iter()
        .map(|(&k, v)| (k, (v.len() as f32) * weight / x_size))
        .collect()
}
