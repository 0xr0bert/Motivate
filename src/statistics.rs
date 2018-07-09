use std::collections::HashMap;
use agent::Agent;
use transport_mode::TransportMode;
use journey_type::JourneyType;
use subculture::Subculture;
use neighbourhood::Neighbourhood;
use std::rc::Rc;
use std::cell::RefCell;
use itertools::Itertools;

pub fn count_active_mode(agents: &[Rc<RefCell<Agent>>]) -> usize {
    agents
        .iter()
        .filter(|&a| a.borrow().current_mode == TransportMode::Walk
            || a.borrow().current_mode == TransportMode::Cycle)
        .count()
}

pub fn count_active_mode_counter_to_inactive_norm(agents: &[Rc<RefCell<Agent>>]) -> usize {
    agents
        .iter()
        .filter(|&a|
            (a.borrow().current_mode == TransportMode::Walk
                || a.borrow().current_mode == TransportMode::Cycle)
            && (a.borrow().norm != TransportMode::Walk && a.borrow().norm != TransportMode::Cycle))
        .count()
}

pub fn count_inactive_mode_counter_to_active_norm(agents: &[Rc<RefCell<Agent>>]) -> usize {
    agents
        .iter()
        .filter(|&a|
            (a.borrow().current_mode != TransportMode::Walk
                && a.borrow().current_mode != TransportMode::Cycle)
            && (a.borrow().norm == TransportMode::Walk || a.borrow().norm == TransportMode::Cycle))
        .count()
}

pub fn count_active_norm(agents: &[Rc<RefCell<Agent>>]) -> usize {
    agents
        .iter()
        .filter(|&a|
            a.borrow().norm == TransportMode::Walk || a.borrow().norm == TransportMode::Cycle)
        .count()
}

pub fn count_active_mode_by_commute_length(agents: &[Rc<RefCell<Agent>>]) -> HashMap<JourneyType, usize> {
    agents
        .iter()
        .map(|agent| (agent.borrow().commute_length, Rc::clone(agent)))
        .into_group_map()
        .into_iter()
        .map(|(journey_type, grouped_agents)| (journey_type, count_active_mode(&grouped_agents)))
        .collect()
}

pub fn count_active_mode_by_subculture (agents: &[Rc<RefCell<Agent>>]) -> HashMap<Rc<Subculture>, usize> {
    agents
        .iter()
        .map(|agent| (Rc::clone(&agent.borrow().subculture), Rc::clone(agent)))
        .into_group_map()
        .into_iter()
        .map(|(subculture, grouped_agents)| (subculture, count_active_mode(&grouped_agents)))
        .collect()
}

pub fn count_active_mode_by_neighbourhood (agents: &[Rc<RefCell<Agent>>]) -> HashMap<Rc<Neighbourhood>, usize> {
    agents
        .iter()
        .map(|agent| (Rc::clone(&agent.borrow().neighbourhood), Rc::clone(agent)))
        .into_group_map()
        .into_iter()
        .map(|(neighbourhood, grouped_agents)| (neighbourhood, count_active_mode(&grouped_agents)))
        .collect()
}
