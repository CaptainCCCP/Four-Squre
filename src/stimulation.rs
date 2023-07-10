use bracket_lib::color::{BLACK, YELLOW,RED,WHITE,NAVY};
use bracket_lib::prelude::{main_loop, BError, BTermBuilder,
                           BTerm, VirtualKeyCode, GameState,
                           to_cp437, RandomNumberGenerator, TextAlign, RGBA, Point};

use std::collections::HashMap;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};

use std::{vec, string};      

use crate::map::{Map,map_idx};
use crate::currentland::Currentland;
use crate::lands::{self, LandType};
use crate::buildings::{self,Building, BuildingType};
use crate::buildings::BuildingType::BreadFactory;
use crate::buildings::GoodType::{Bread, Wheat};
use crate::people::{PersonType,People};

const GAME_WIDTH:i32 = 80;
const GAME_HEIGHT:i32 = 50;
const PERIOD:f32 = 1000.0;
const START_X:i32 = 10;
const START_Y:i32 = 3;

enum GameMode{
    Menu,
    Playing,
    End,
}

pub struct State{
    //游戏整体相关数据
    mode:GameMode,
    frame_time:f32,
    time:i32,

    map:Map,
    currentland:Currentland,
    world_market:HashMap<String, u32>,//数据结构存储所谓世界市场的货物数量
}
impl State {
    //构造函数开始的时候新建一个state
    pub fn new()->Self{
        State{
            mode:GameMode::Menu,
            frame_time:0.0,
            time:0,

            map:Map::new(),
            currentland:Currentland::new(Point::new(START_X,START_Y)),
            world_market:HashMap::new(),
        }
    }
    pub fn back_to_menu(&mut self){
        self.mode = GameMode::Menu;
        self.frame_time = 0.0;
        self.time = 0;
        self.map = Map::new();
    }
    //重启
    pub fn restart(&mut self){
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.time = 0;

        //初始化货物
        self.world_market.insert(String::from("wheat"), 0);
        self.world_market.insert(String::from("apple"), 0);
    }
//=================================================================================================
    //游戏主进程
    pub fn play(&mut self,ctx:&mut BTerm){
        ctx.cls();
        //硬件操作中断
        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::M => self.back_to_menu(),
                VirtualKeyCode::Q => ctx.quitting = true,
                VirtualKeyCode::L => {//开垦土地
                    self.map.get_lands()[map_idx(self.currentland.position.x-START_X,
                        self.currentland.position.y-START_Y)].cultivate(3,LandType::Farmland,5);
                }
                VirtualKeyCode::P => {//添加人口
                    let land = self.map.get_lands();
                    land[map_idx(self.currentland.position.x-START_X,
                        self.currentland.position.y-START_Y)]
                        .people_list.push(People::new(1,500,PersonType::Farmer));
                }
                VirtualKeyCode::B => {//添加建筑
                    let land = self.map.get_lands();
                    land[map_idx(self.currentland.position.x-START_X,
                        self.currentland.position.y-START_Y)]
                        .building_list.push(Building::new(3,Wheat,1,Bread,BreadFactory));
                }
                _ => {}
            }
        }
    //接收货物信息
    for land in self.map.get_lands(){
        let received_wheat:u32 = land.produce();
        let good_name:String = String::from("wheat");
        //更新货物信息
        let mut wheat:u32 = self.world_market.get(&good_name).copied().unwrap_or(0);
        wheat += received_wheat;
        self.world_market.insert(String::from("wheat"), wheat);
    }
    //居民消费
    for land in self.map.get_lands().iter(){
        for people in land.people_list.iter(){
            let consumed_wheat:u32 = people.consume();
            let good_name:String = String::from("wheat");
            //更新货物信息
            let mut wheat:u32 = self.world_market.get(&good_name).copied().unwrap_or(0);
            wheat -= consumed_wheat;
            self.world_market.insert(String::from("wheat"), wheat);
        }
    }
    //画面打印信息的定义
        let mut land_size_y:u32 = 13;
        let mut land_size_x:u32 = 35;
        let mut good_y:u32 = 13;
        let mut good_x:u32 = 2;
        let mut pop_x:u32 = 45;
        let mut pop_y:u32 = 12;
    //整体
        //背景颜色
        ctx.cls_bg(BLACK);
        self.currentland.update(ctx,&self.map);
        self.map.render(ctx);
        self.currentland.render(ctx);
    //左上角：
        //显示时间
        self.frame_time += ctx.frame_time_ms;//计时
        if self.frame_time > PERIOD {
            self.frame_time = 0.0;
            self.time += 1;
        }
        ctx.print(0, 1, &format!("Time: {}", self.time));
 
        
    //右上角：
        //按键提示
        ctx.print(60,2, "(M) Back to Menu");
        ctx.print(60,1, "(Q) Quit");
    //左下角：
    //右下角：
    //中间：
        //中上人口列表
        ctx.draw_hollow_box(44, 10, 20,35, WHITE, BLACK);//x,y,宽,高,fg字符颜色，bg背景颜色
        ctx.print(45, 11, &format!("pop:"));
        ctx.print(55, 11, &format!("area:"));
        //打印人口和地区至终端
        for people in &self.map.get_lands()[map_idx(self.currentland.position.x-START_X,
            self.currentland.position.y-START_Y)].people_list{
            let person_type = &people.people_type;
            let person_type_str = match person_type {
                PersonType::Farmer => "Farmer",
                PersonType::Worker => "Worker",
                PersonType::Trader => "Trader",
            };
            ctx.print(pop_x,pop_y,&format!("{}",person_type_str));
            pop_y += 1;

        }
        //左侧货物列表
        ctx.draw_hollow_box(1, 10, 20,35, WHITE, BLACK);//x,y,宽,高,fg字符颜色，bg背景颜色
        ctx.print(2, 11, &format!("World market:"));
        ctx.print(2, 12, &format!("name"));
        ctx.print(10, 12, &format!("number"));
        //打印货物数量至终端
        for (key, value) in &self.world_market {
            ctx.print(good_x, good_y,&format!("{key}:   {value}"));
            good_y += 1;

        }
        //右侧土地列表
        ctx.draw_hollow_box(22, 10, 20,35, WHITE, BLACK);//x,y,宽,高,fg字符颜色，bg背景颜色
        ctx.print(23, 11, &format!("Worldlands:"));
        ctx.print(23, 12, &format!("name"));
        ctx.print(33, 12, &format!("size"));
        //打印土地至终端
        for land in self.map.get_lands() {
            ctx.print(land_size_x, land_size_y,&format!("{}",land.show_size()));
            land_size_y += 1;
        }
    }
//=================================================================================================
    //主菜单
    pub fn main_menu(&mut self,ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to Market Stimulator");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    pub fn dead(&mut self,ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "GAME OVER");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

}
impl GameState for State{
    //tick每一帧(rendered frame)都调用，实时监听所有状态变化
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode{
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
            _ => {}
        }
    }
}