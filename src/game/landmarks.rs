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

pub fn get_landmark_description(landmark: &LandmarkKind) -> &'static str {
    match landmark {
        LandmarkKind::TrainStation => "You may roll 1 or 2 dice.",
        LandmarkKind::ShoppingMall => "Each of your Cup and Bread establishments earn +1 coin.",
        LandmarkKind::AmusementPark => "If you roll doubles, take another turn after this one.",
        LandmarkKind::RadioTower => "Once every turn, you can choose to re-roll your dice.",
    }
}
