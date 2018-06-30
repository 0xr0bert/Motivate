use std::collections::HashMap;
use transport_mode::TransportMode;

#[derive(PartialEq, Clone)]
pub struct Subculture {
    pub desirability: HashMap<TransportMode, f32>
}