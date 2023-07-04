use bracket_lib::color::{BLACK, YELLOW};
use bracket_lib::prelude::{main_loop, BError, BTermBuilder, BTerm,RED,
                           VirtualKeyCode, GameState, to_cp437,NAVY,RandomNumberGenerator};

const GAME_WIDTH:i32 = 80;
const GAME_HEIGHT:i32 = 50;
const PERIOD:f32 = 1000.0;

enum GameMode{
    Menu,
    Playing,
    Paused,
    End,
}

pub struct State{
    mode:GameMode,
    frame_time:f32,
    time:i32,
}
impl State {
    //构造函数开始的时候新建一个state
    pub fn new()->Self{
        State{
            mode:GameMode::Menu,
            frame_time:0.0,
            time:0,
        }
    }
    //游戏主进程
    pub fn play(&mut self,ctx:&mut BTerm){
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;//计时
        if self.frame_time > PERIOD {
            self.frame_time = 0.0;
            self.time += 1;
            println!("+1");
        }

        if self.time > 10 {
            self.mode = GameMode::End;
        }
    }
    //重启
    pub fn restart(&mut self){
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.time = 0;
    }
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
    //tick汉书实时监听所有状态变化
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode{
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
            _ => {}
        }
    }
}