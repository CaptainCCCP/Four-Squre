use bracket_lib::color::{BLACK, YELLOW,RED,WHITE,NAVY,GREEN};
use bracket_lib::prelude::{main_loop, BError, BTermBuilder, BTerm, VirtualKeyCode, GameState, to_cp437, RandomNumberGenerator, TextAlign, RGBA, Point};
use crate::lands::{Land, LandType};
use crate::buildings::{self,Building, BuildingType};
use crate::lands::LandType::Grassland;

const NUM_LANDS:usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
pub const SCREEN_WIDTH:i32 = 5;
pub const SCREEN_HEIGHT:i32 = 3;
pub const START_X:i32 = 10;
pub const START_Y:i32 = 3;
const GAME_WIDTH:i32 = 80;
const GAME_HEIGHT:i32 = 50;

pub struct Map{
    lands:Vec<Land>,//地块类型、大小、肥沃度、建筑列表
 }

pub fn map_idx(x:i32, y:i32) -> usize{
        ((y * SCREEN_WIDTH) + x) as usize
    }

impl Map{
    pub fn new() -> Self{
        Self{
            lands:vec![Land::new(10,Grassland,5,Vec::new(),Vec::new());NUM_LANDS]
        }
    }
    //
    pub fn render(&self, ctx:&mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x ,y);
                match self.lands[idx].get_type(){
                    LandType::Grassland => {
                        ctx.set(x + START_X ,y + START_Y,GREEN,BLACK,to_cp437('#'));
                    }
                    LandType::Farmland => {
                        ctx.set(x + START_X,y + START_Y,YELLOW,BLACK,to_cp437('#'));
                    }
                    LandType::Pasture => {
                        ctx.set(x + START_X,y + START_Y,WHITE,BLACK,to_cp437('#'));
                    }
                    LandType::River => {
                        ctx.set(x + START_X,y + START_Y,WHITE,BLACK,to_cp437('*'));
                    }
                }
            }
        }
    }
    //
    pub fn get_lands(&mut self)-> &mut Vec<Land> { &mut self.lands }
    //pub fn get_type(&self) -> &LandType { &self.land_type }
    //
    pub fn in_bounds(&self,point:Point) -> bool {
        point.x >= 0 && point.x < GAME_WIDTH
            && point.y >= 0 && point.y < GAME_HEIGHT
    }
    //
    pub fn can_enter_land(&self,x:i32, y:i32) -> bool {
        *self.lands[map_idx(x,y)].get_type() != LandType::River
    }
    //
    pub fn try_idx(&self, point:Point) -> Option<usize>{
        if !self.in_bounds(point){
            None
        }else{
            Some(map_idx(point.x,point.y))
        }
    }
}