use rand::distributions;
use rand::distributions::Distribution;
use rand::{thread_rng};
use std::collections::HashMap;

/// Generate a scale free network
/// https://en.wikipedia.org/wiki/Barab%C3%A1si%E2%80%93Albert_model
/// agents: a slice of agents
/// min: the minimum number of links
/// total_number: the number of nodes in the network
/// Returns: A HashMap mapping ids, to the ids of their friends
pub fn generate_social_network(min: u32, total_number: u32) -> HashMap<u32, Vec<u32>>{
    // Create a HashMap mapping ids, to the ids of their friends
    let mut network: HashMap<u32, Vec<u32>> = HashMap::new();

    // For total_number ids
    for i in 0..total_number {
        // Create a Vector to store i's friends
        let mut i_friends: Vec<u32> = Vec::new();
        // If there aren't yet min number of people in the network, link i to everyone already in the network
        if network.len() < min as usize {
            for (&k, v) in network.iter_mut() {
                i_friends.push(k);
                v.push(i);
            }
        } else {
            // Otherwise, create a weighted distribution, where the weight of being selected is equal to the size of your network
            // This is preferential attachment
            let mut weighted: Vec<distributions::Weighted<u32>> = network
                .iter()
                .map(|(&k, v)| distributions::Weighted {weight: v.len() as u32, item: k})
                .collect();
            let weighted_choice = distributions::WeightedChoice::new(&mut weighted);

            // For min ids selected from the sample, link them together
            for _ in 0..min {
                let id: u32 = weighted_choice.sample(&mut thread_rng());
                network.get_mut(&id).unwrap().push(i);
                i_friends.push(id);
            }
        }
        // Add the newly linked id to the network
        network.insert(i, i_friends);
    }

    network
}
