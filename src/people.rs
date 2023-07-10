pub enum PersonType {
    Farmer,
    Worker,
}

pub struct People {
    wheat_consume:u32,
    pub people_type:PersonType,
}

impl People {
    pub fn new(wheat_consume: u32,people_type:PersonType) -> Self {
        People {
            wheat_consume:wheat_consume,
            people_type:people_type,
        }
    }

    pub fn consume(&self) -> u32 {
        self.wheat_consume
    }


    fn get_type(&self) -> &PersonType {
        match &self.people_type {
            PersonType::Farmer => &self.people_type,
            PersonType::Worker => &self.people_type,
        }
    }
}