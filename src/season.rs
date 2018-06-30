#[derive(PartialEq, Copy, Clone)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Autumn
}

impl Season {
    pub fn percentage_bad_weather(&self) -> f32 {
        match *self {
            Season::Winter => 0.3455,
            Season::Spring => 0.3098,
            Season::Summer => 0.2696,
            Season::Autumn => 0.3275
        }
    }
}

pub fn season(day: u32) -> Season {
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