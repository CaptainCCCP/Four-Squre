use bracket_lib::color::{BLACK, YELLOW};
use bracket_lib::prelude::{main_loop, BError, BTermBuilder, BTerm,RED,
                           VirtualKeyCode, GameState, to_cp437,NAVY,RandomNumberGenerator};

const GAME_WIDTH:i32 = 80;
const GAME_HEIGHT:i32 = 50;
const PERIOD:f32 = 30.0;

enum GameMode{
    Menu,
    Playing,
    Paused,
    End,
}
pub struct Obstacle {
    x: i32, // 世界空间
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x; // 屏幕空间
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..GAME_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

pub struct Player{
    x:i32,
    y:i32,
    velocity:f32,
}

impl Player{
    //创建玩家
    fn new(x:i32,y:i32)->Self{
        Player{
            x:0,
            y:0,
            velocity:0.0,
        }
    }
    //轮廓?
    fn render(&mut self,ctx:&mut BTerm){
        ctx.set(0,self.y,YELLOW,BLACK,to_cp437('@'))
    }
    //
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }
    //非
    fn flap(&mut self) {
        self.velocity = -2.0; // 往上飞是负的
    }
}
pub struct State{
    mode:GameMode,
    player:Player,
    frame_time:f32,
    obstacle:Obstacle,
    score:i32,
    //time:u32,
}

impl State {
    //构造函数开始的时候新建一个state
    pub fn new()->Self{
        State{
            mode:GameMode::Menu,
            player:Player::new(5,25),
            frame_time:0.0,
            obstacle: Obstacle::new(GAME_WIDTH, 0),
            score: 0,
        }
    }
    //游戏主进程
    pub fn play(&mut self,ctx:&mut BTerm){
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > PERIOD {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to Flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + GAME_WIDTH, self.score);
        }

        if self.player.y > GAME_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }
    //重启
    pub fn restart(&mut self){
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(GAME_WIDTH, 0);
        self.score = 0;
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
        ctx.print_centered(6, &format!("You earned {} points", self.score));
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