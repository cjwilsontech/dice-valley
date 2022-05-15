pub const CARD_KIND_COUNT: usize = 15;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardKind {
    WheatField,
    Ranch,
    Forest,
    Mine,
    AppleOrchard,
    Bakery,
    ConvenienceStore,
    CheeseFactory,
    FurnitureFactory,
    FruitAndVegetableMarket,
    Cafe,
    FamilyRestaurant,
    Stadium,
    TvStation,
    BusinessCenter,
}

pub const ALL_CARDS: [CardKind; CARD_KIND_COUNT] = [
    CardKind::AppleOrchard,
    CardKind::Bakery,
    CardKind::BusinessCenter,
    CardKind::Cafe,
    CardKind::CheeseFactory,
    CardKind::ConvenienceStore,
    CardKind::FamilyRestaurant,
    CardKind::Forest,
    CardKind::FruitAndVegetableMarket,
    CardKind::FurnitureFactory,
    CardKind::Mine,
    CardKind::Ranch,
    CardKind::Stadium,
    CardKind::TvStation,
    CardKind::WheatField,
];

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CardOrder {
    Restaurants = 0,
    SecondaryIndustry = 1,
    PrimaryIndustry = 2,
    MajorEstablishments = 3,
}

#[derive(PartialEq)]
pub enum CardIcon {
    Wheat,
    Cow,
    Gear,
    Bread,
    Factory,
    Cup,
    Major,
    Fruit,
}

#[derive(Clone, Copy)]
pub struct CardStack {
    pub kind: CardKind,
    pub count: u8,
}

impl CardStack {
    pub fn test_activation(&self, roll: u8, is_current_turn: bool) -> bool {
        (match self.kind {
            CardKind::WheatField => roll == 1,
            CardKind::Ranch => roll == 2,
            CardKind::Bakery => roll == 2 || roll == 3,
            CardKind::Cafe => roll == 3,
            CardKind::ConvenienceStore => roll == 4,
            CardKind::Forest => roll == 5,
            CardKind::Stadium | CardKind::TvStation | CardKind::BusinessCenter => roll == 6,
            CardKind::CheeseFactory => roll == 7,
            CardKind::FurnitureFactory => roll == 8,
            CardKind::Mine => roll == 9,
            CardKind::FamilyRestaurant => roll == 9 || roll == 10,
            CardKind::AppleOrchard => roll == 10,
            CardKind::FruitAndVegetableMarket => roll == 11 || roll == 12,
        } && match self.get_order() {
            CardOrder::PrimaryIndustry => true,
            CardOrder::MajorEstablishments | CardOrder::SecondaryIndustry => is_current_turn,
            CardOrder::Restaurants => !is_current_turn,
        })
    }

    pub fn get_cost(&self) -> u8 {
        match self.kind {
            CardKind::WheatField | CardKind::Ranch | CardKind::Bakery => 1,
            CardKind::Cafe | CardKind::ConvenienceStore | CardKind::FruitAndVegetableMarket => 2,
            CardKind::Forest
            | CardKind::FurnitureFactory
            | CardKind::FamilyRestaurant
            | CardKind::AppleOrchard => 3,
            CardKind::CheeseFactory => 5,
            CardKind::Mine | CardKind::Stadium => 6,
            CardKind::TvStation => 7,
            CardKind::BusinessCenter => 8,
        }
    }

    pub fn get_order(&self) -> CardOrder {
        match self.kind {
            CardKind::WheatField
            | CardKind::Ranch
            | CardKind::Forest
            | CardKind::Mine
            | CardKind::AppleOrchard => CardOrder::PrimaryIndustry,
            CardKind::Bakery
            | CardKind::ConvenienceStore
            | CardKind::CheeseFactory
            | CardKind::FurnitureFactory
            | CardKind::FruitAndVegetableMarket => CardOrder::SecondaryIndustry,
            CardKind::Cafe | CardKind::FamilyRestaurant => CardOrder::Restaurants,
            CardKind::Stadium | CardKind::TvStation | CardKind::BusinessCenter => {
                CardOrder::MajorEstablishments
            }
        }
    }

    pub fn get_order_title(&self) -> &'static str {
        match self.get_order() {
            CardOrder::MajorEstablishments => "Major Establishments",
            CardOrder::PrimaryIndustry => "Primary Industry",
            CardOrder::Restaurants => "Restaurants",
            CardOrder::SecondaryIndustry => "Secondary Industry",
        }
    }

    pub fn get_icon(&self) -> CardIcon {
        match self.kind {
            CardKind::AppleOrchard | CardKind::WheatField => CardIcon::Wheat,
            CardKind::Bakery | CardKind::ConvenienceStore => CardIcon::Bread,
            CardKind::Ranch => CardIcon::Cow,
            CardKind::Cafe | CardKind::FamilyRestaurant => CardIcon::Cup,
            CardKind::Forest | CardKind::Mine => CardIcon::Gear,
            CardKind::BusinessCenter | CardKind::Stadium | CardKind::TvStation => CardIcon::Major,
            CardKind::CheeseFactory | CardKind::FurnitureFactory => CardIcon::Factory,
            CardKind::FruitAndVegetableMarket => CardIcon::Fruit,
        }
    }

    pub fn get_icon_title(&self) -> &'static str {
        match self.get_icon() {
            CardIcon::Wheat => "Wheat",
            CardIcon::Bread => "Bread",
            CardIcon::Cow => "Cow",
            CardIcon::Cup => "Cup",
            CardIcon::Gear => "Gear",
            CardIcon::Major => "Major",
            CardIcon::Factory => "Factory",
            CardIcon::Fruit => "Fruit",
        }
    }

    pub fn get_title(&self) -> &'static str {
        match self.kind {
            CardKind::WheatField => "Wheat Field",
            CardKind::Ranch => "Ranch",
            CardKind::Bakery => "Bakery",
            CardKind::Cafe => "Cafe",
            CardKind::ConvenienceStore => "Convenience Store",
            CardKind::Forest => "Forest",
            CardKind::Stadium => "Stadium",
            CardKind::TvStation => "TV Station",
            CardKind::BusinessCenter => "Business Center",
            CardKind::CheeseFactory => "Cheese Factory",
            CardKind::FurnitureFactory => "Furniture Factory",
            CardKind::Mine => "Mine",
            CardKind::FamilyRestaurant => "Family Restaurant",
            CardKind::AppleOrchard => "Apple Orchard",
            CardKind::FruitAndVegetableMarket => "Fruit and Vegetable Market",
        }
    }
}
