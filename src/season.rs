#[derive(PartialEq, Copy, Clone)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Autumn
}

impl Season {
    /// The percentage of days for which there was bad weather
    /// Based of Met Office data for Northolt https://www.metoffice.gov.uk/public/weather/climate/gcptq81bc#?tab=climateTables
    /// Days in which there is over 1mm of rainfall
    pub fn percentage_bad_weather(&self) -> f32 {
        match *self {
            Season::Winter => 0.3455,
            Season::Spring => 0.3098,
            Season::Summer => 0.2696,
            Season::Autumn => 0.3275
        }
    }
}

/// Gets the season for a given day
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