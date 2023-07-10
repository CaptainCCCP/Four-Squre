pub enum GoodType {
    Wheat,
    Bread,
}

pub enum BuildingType {
    BreadFactory,
    House,
}

pub struct Building {
    input_num:u32,
    input_type:GoodType,
    output_num:u32,
    output_type:GoodType,
    building_type:BuildingType,
}

impl Building {
    pub fn new(input_num: u32,input_type: GoodType,output_num: u32,output_type: GoodType,
               building_type:BuildingType) -> Self {
        Building {
            input_num:input_num,
            input_type:input_type,
            output_num:output_num,
            output_type:output_type,
            building_type:building_type,
        }
    }

    pub fn produce(&self) -> u32 {
        self.output_num
    }

    // pub fn set_size(&mut self, size:u32) {
    //     self.size = size;
    // }

    pub fn get_type(&self) -> &BuildingType {
        match &self.building_type {
            BuildingType::BreadFactory => &self.building_type,
            BuildingType::House => &self.building_type,
        }
    }
}