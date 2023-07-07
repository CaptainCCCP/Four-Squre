use bracket_lib::color::{BLACK, YELLOW,RED,WHITE,NAVY};
use bracket_lib::prelude::{main_loop, BError, BTermBuilder, BTerm,VirtualKeyCode,
                         GameState, to_cp437,RandomNumberGenerator,TextAlign,RGBA};

use std::collections::HashMap;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};

use std::{vec, string};      

use crate::lands::{self, Grassland};

const GAME_WIDTH:i32 = 80;
const GAME_HEIGHT:i32 = 50;
const PERIOD:f32 = 1000.0;

enum GameMode{
    Menu,
    Playing,
    //Paused,
    End,
}

pub struct State{
    //游戏整体相关数据
    mode:GameMode,
    frame_time:f32,
    time:i32,
    
    world_lands:Vec<lands::Grassland>,
    world_market:HashMap<String, u32>,//数据结构存储所谓世界市场的货物数量
}
impl State {
    //构造函数开始的时候新建一个state
    pub fn new()->Self{
        State{
            mode:GameMode::Menu,
            frame_time:0.0,
            time:0,
            world_lands:Vec::new(),
            world_market:HashMap::new(),
        }
    }
    pub fn back_to_menu(&mut self){
        self.mode = GameMode::Menu;
        self.frame_time = 0.0;
        self.time = 0;
    }
    //重启
    pub fn restart(&mut self){
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.time = 0;
        //初始化土地
        self.world_lands.clear();
        self.world_lands.shrink_to_fit();
        //初始化货物
        self.world_market.insert(String::from("wheat"), 0);
        self.world_market.insert(String::from("apple"), 0);
    }
//=================================================================================================
    //游戏主进程
    pub fn play(&mut self,ctx:&mut BTerm){
    //接收货物信息
    let received_wheat:u32 = self.new_land(ctx);
    //更新货物信息
    for land in &self.world_lands{
        let good_name:String = String::from("wheat");
        let mut wheat:u32 = self.world_market.get(&good_name).copied().unwrap_or(0);
        wheat += received_wheat;
        self.world_market.insert(String::from("wheat"), wheat);
    }
        //self.world_lands
    //画面打印信息的定义
        let mut land_size_y:u32 = 13;
        let mut land_size_x:u32 = 51;
        let mut land_name_y:u32 = 13;
        let mut land_name_x:u32 = 51;
        let mut good_y:u32 = 13;
        let mut good_x:u32 = 15;
    //整体
        //背景颜色
        ctx.cls_bg(BLACK);
        //按键退出和返回菜单
        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::M => self.back_to_menu(),
                VirtualKeyCode::Q => ctx.quitting = true,
                VirtualKeyCode::L => {
                    self.world_lands.push(Grassland::new(10,5));
                }
                _ => {}
            }
        }

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
        //左侧货物列表
        ctx.draw_hollow_box(10, 10, 25,35, WHITE, BLACK);//x,y,宽,高,fg字符颜色，bg背景颜色
        ctx.print(11, 11, &format!("Worldmarket:"));
        ctx.print(15, 12, &format!("name"));
        ctx.print(25, 12, &format!("number"));
        //打印货物数量至终端
        for (key, value) in &self.world_market {
            ctx.print(good_x, good_y,&format!("{key}: {value}"));
            good_y += 1;

        }
        //右侧土地列表
        ctx.draw_hollow_box(40, 10, 25,35, WHITE, BLACK);//x,y,宽,高,fg字符颜色，bg背景颜色
        ctx.print(41, 11, &format!("Worldlands:"));
        ctx.print(41, 12, &format!("name"));
        ctx.print(51, 12, &format!("size"));
        //打印土地至终端
        for land in &self.world_lands {
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
    //新建土地
    fn new_land(&mut self, ctx: &mut BTerm) -> u32{
            //开线程
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let val:u32 = 5;
                tx.send(val).unwrap();
            });
            let received = rx.recv().unwrap();
            received
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