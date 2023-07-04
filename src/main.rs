//github repo name:github;
use bracket_lib::prelude::*;//游戏引擎
//use std::{cmp::Ordering, io};
//use std::time::{Duration, Instant};
//use rand::Rng;

// pub mod buildings;
// pub mod lands;
// pub mod people;

// enum GameMode{
//     Menu,
//     Playing,
//     End,
// }

// struct State{
//     mode:GameMode,
// }

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1,1,"Hello Bracket Terminal!");
    }
}

fn main()->BError {
    println!("Hello, world!");
    //初始化app,window,graphics
    let context = BTermBuilder::simple80x50()
            .with_title("market_stimulate")
            .build()?;
    //刷新数据更新状态          //多线程监听用户输入,根据事件、动作来改变状态及状态
    //tick function
    //渲染游戏画面
    main_loop(context,State{})
}
