pub enum PersonType {
    Farmer,
    Worker,
    Trader,
}

pub struct People {
    food_consume:u32,
    storage:u32,
    pub people_type:PersonType,
}

impl People {
    pub fn new(food_consume: u32,storage:u32,people_type:PersonType) -> Self {
        People {
            food_consume:food_consume,
            storage:storage,
            people_type:people_type,
        }
    }

    pub fn consume(&self) -> u32 {
        self.food_consume
    }


    fn get_type(&self) -> &PersonType {
        match &self.people_type {
            PersonType::Farmer => &self.people_type,
            PersonType::Worker => &self.people_type,
            PersonType::Trader => &self.people_type,
        }
    }
}