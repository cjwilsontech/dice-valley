pub const LANDMARK_KIND_COUNT: usize = 4;

#[derive(Clone, PartialEq)]
pub enum LandmarkKind {
    TrainStation,
    ShoppingMall,
    AmusementPark,
    RadioTower,
}

pub const ALL_LANDMARKS: [LandmarkKind; LANDMARK_KIND_COUNT] = [
    LandmarkKind::TrainStation,
    LandmarkKind::ShoppingMall,
    LandmarkKind::AmusementPark,
    LandmarkKind::RadioTower,
];

pub fn get_landmark_cost(landmark: &LandmarkKind) -> u8 {
    match landmark {
        LandmarkKind::TrainStation => 4,
        LandmarkKind::ShoppingMall => 10,
        LandmarkKind::AmusementPark => 16,
        LandmarkKind::RadioTower => 22,
    }
}

pub fn get_landmark_title(landmark: &LandmarkKind) -> &'static str {
    match landmark {
        LandmarkKind::TrainStation => "Train Station",
        LandmarkKind::ShoppingMall => "Shopping Mall",
        LandmarkKind::AmusementPark => "Amusement Park",
        LandmarkKind::RadioTower => "Radio Tower",
    }
}
