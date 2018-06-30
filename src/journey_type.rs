#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum JourneyType {
    LocalCommute,
    CityCommute,
    DistantCommute
}