pub enum BuildingType {
    Factory,
    House,
}

pub struct Building {
    size:u32,
    building_type:BuildingType,
}

impl Building {
    pub fn new(size: u32,building_type:BuildingType) -> Self {
        Building {
            size:size,
            building_type:building_type,
        }
    }

    pub fn produce(&self) -> u32 {
        self.size
    }

    pub fn set_size(&mut self, size:u32) {
        self.size = size;
    }

    pub fn get_type(&self) -> &BuildingType {
        match &self.building_type {
            BuildingType::Factory => &self.building_type,
            BuildingType::House => &self.building_type,
        }
    }
}