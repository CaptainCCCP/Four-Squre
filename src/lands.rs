use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::sleep;

use crate::buildings::{self,Building, BuildingType};
#[derive(Debug)]
pub enum LandType {
    Grassland,
    Farmland,
    Pasture,
}

pub struct Land{
    //类型
    land_type:LandType,
    // 大小，building承载量
    size:u32,
    //肥沃度，物资产出量
    fertility: u32,
    //建筑列表
    building_list:Vec<Building>,
}

impl Land{
    //新建一个Grassland
    pub fn new(size:u32,land_type:LandType,fertility:u32,building_list:Vec<Building>) -> Self{
        //参数不带self的是new，常用来作构造函数
        //新建一片土地
        Land{
            size:size,
            land_type:land_type,
            fertility:fertility,
            building_list:building_list,
        }
    }
    //
    pub fn add_building(&mut self,value:u32){
        let mut newbuilding = Building::new(100, BuildingType::Factory);
        self.building_list.push(newbuilding);
        self.update_size();
    }
    //
    pub fn remove_building(&mut self) -> Option<Building>{
        let result = self.building_list.pop();
        match result {
            Some(value) => {
                self.update_size();
                Some(value)
            }
            None => None,
        }

    }
    //更新属性
    fn update_size(&mut self){
        self.size = self.building_list.len() as u32;
     }
    fn update_fertility(&mut self,value:u32) {
        self.fertility = value;
    }
    //产出
    pub fn produce(&self) -> u32{
        self.fertility
    }
    //展示size
    pub fn show_size(&self) -> u32{
        self.size
    }
    //新建土地
    pub fn new_land(&self) -> u32{
            // //开线程
            // let (tx, rx) = mpsc::channel();
            // thread::spawn(move || {
            //     let val = &self.fertility;
            //     tx.send(val).unwrap();
            //     thread::sleep(Duration::from_secs(1));
            // });
            // let received = rx.recv().unwrap();
            //thread::sleep(Duration::from_millis(100));
            self.fertility
        }
}

// pub struct Farmland{
//     size:u32,
// }
// pub struct Forest{

// }
// pub struct Plateau{

// }
// pub struct Wasteland{

// }