use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::sleep;

use crate::buildings::{self,Building, BuildingType};
use crate::buildings::GoodType::{Wheat,Bread};
use crate::people::People;

#[derive(Debug,PartialEq,Clone)]
pub enum LandType {
    Grassland,
    Farmland,
    Pasture,
    River,
}

pub struct Land{
    //类型
    pub land_type:LandType,
    // 大小，building承载量
    size:u32,
    //肥沃度，物资产出量
    fertility: u32,
    //建筑列表
    pub building_list:Vec<Building>,
    //人口列表
    pub people_list:Vec<People>,
    //待添加 价格表 自然情况表
}

impl Land{
    //新建一个Grassland
    pub fn new(size:u32,land_type:LandType,fertility:u32,
               building_list:Vec<Building>,people_list:Vec<People>) -> Self{
        //参数不带self的是new，常用来作构造函数
        //新建一片土地
        Land{
            size:size,
            land_type:land_type,
            fertility:fertility,
            building_list:building_list,
            people_list:people_list,
        }
    }
    //
    pub fn add_building(&mut self,value:u32){
        let mut newbuilding = Building::new(3, Wheat,1,Bread,BuildingType::BreadFactory);
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
    //更新属性TODO:有问题
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
    //开垦
    pub fn cultivate(&mut self,size:u32,land_type:LandType,fertility:u32) {
        self.size = size;
        self.land_type = land_type;
        self.fertility = fertility;
    }
    //展示size
    pub fn show_size(&self) -> u32{
        self.size
    }
    pub fn get_type(&self) -> &LandType { &self.land_type }
    pub fn get_people(&self) -> &Vec<People> { &self.people_list }
    pub fn get_building(&self) -> &Vec<Building> { &self.building_list }
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
impl Clone for Land{
    fn clone(&self) -> Self {
        Land {
            size:self.size,
            land_type:LandType::Grassland,
            fertility:self.fertility,
            building_list:Vec::new(),
            people_list:Vec::new(),
        }
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