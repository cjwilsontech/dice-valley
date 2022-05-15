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
    MajorEstabalishments = 3,
}

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

fn get_icon_title(icon: CardIcon) -> String {
    match icon {
        CardIcon::Wheat => String::from("Wheat"),
        CardIcon::Bread => String::from("Bread"),
        CardIcon::Cow => String::from("Cow"),
        CardIcon::Cup => String::from("Cup"),
        CardIcon::Gear => String::from("Gear"),
        CardIcon::Major => String::from("Major"),
        CardIcon::Factory => String::from("Factory"),
        CardIcon::Fruit => String::from("Fruit"),
    }
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
            CardOrder::MajorEstabalishments | CardOrder::SecondaryIndustry => is_current_turn,
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
                CardOrder::MajorEstabalishments
            }
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

    pub fn get_icon_title(&self) -> String {
        get_icon_title(self.get_icon())
    }

    pub fn get_title(&self) -> String {
        match self.kind {
            CardKind::WheatField => String::from("Wheat Field"),
            CardKind::Ranch => String::from("Ranch"),
            CardKind::Bakery => String::from("Bakery"),
            CardKind::Cafe => String::from("Cafe"),
            CardKind::ConvenienceStore => String::from("Convenience Store"),
            CardKind::Forest => String::from("Forest"),
            CardKind::Stadium => String::from("Stadium"),
            CardKind::TvStation => String::from("TV Station"),
            CardKind::BusinessCenter => String::from("Business Center"),
            CardKind::CheeseFactory => String::from("Cheese Factory"),
            CardKind::FurnitureFactory => String::from("Furniture Factory"),
            CardKind::Mine => String::from("Mine"),
            CardKind::FamilyRestaurant => String::from("Family Restaurant"),
            CardKind::AppleOrchard => String::from("Apple Orchard"),
            CardKind::FruitAndVegetableMarket => String::from("Fruit and Vegetable Market"),
        }
    }
}
