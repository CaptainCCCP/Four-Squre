//github repo name:github;
use bracket_lib::prelude::{main_loop, BError, BTermBuilder};//游戏引擎
//use std::{cmp::Ordering, io};
//use std::time::{Duration, Instant};
//use rand::Rng;

pub mod buildings;
pub mod lands;
pub mod people;
pub mod game;
pub mod stimulation;

fn main()->BError {
    //初始化app,window,graphics
    let context = BTermBuilder::simple80x50() //窗口
            .with_title("market_stimulate")
            .build()?;

    //let game = game::State::new();//flappy游戏
    let game = stimulation::State::new();

    //刷新数据更新状态          //多线程监听用户输入,根据事件、动作来改变状态及状态
    //tick function
    //渲染游戏画面
    main_loop(context,game)
}
