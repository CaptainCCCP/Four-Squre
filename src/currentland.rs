use bracket_lib::color::{BLACK, YELLOW,RED,WHITE,NAVY,GREEN};
use bracket_lib::prelude::{main_loop, BError, BTermBuilder, BTerm, VirtualKeyCode, GameState,
                           to_cp437, RandomNumberGenerator, TextAlign, RGBA, Point};
use crate::lands::LandType;
use crate::map::{Map, map_idx,SCREEN_WIDTH,SCREEN_HEIGHT,START_Y,START_X};
//指向的土地
pub struct Currentland{
    pub position:Point
}
impl Currentland{
    pub fn new(position:Point) -> Self{
        Self{
            position
        }
    }
    //
    pub fn render(&self,ctx:&mut BTerm){
        ctx.set(self.position.x,self.position.y,
            RED,
            BLACK,
            to_cp437('@'),
        )
    }
    //
    pub fn update(&mut self,ctx: &mut BTerm,map:&Map){
        if let Some(key) = ctx.key{
            let delta = match key{
                VirtualKeyCode::Left => Point::new(-1,0),
                VirtualKeyCode::Right => Point::new(1,0),
                VirtualKeyCode::Up => Point::new(0,-1),
                VirtualKeyCode::Down => Point::new(0,1),
                _ => Point::zero()
            };
            let new_position = self.position + delta;
            if  new_position.x >= START_X && new_position.y >= START_Y
                && new_position.x < START_X+SCREEN_WIDTH && new_position.y < START_Y+SCREEN_HEIGHT
            {
                if map.can_enter_land(new_position.x - START_X,new_position.y - START_Y)
                {
                    self.position = new_position;
                }
            }


        }
    }
}
