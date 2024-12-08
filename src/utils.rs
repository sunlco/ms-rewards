// 暂时不使用 enigo 相关的代码
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use crate::hot_words::HotWordsData;
use crate::bot::SearchBot;
use std::time::Duration;
use std::thread;
use rand::Rng;


pub fn random_sleep(range: std::ops::Range<usize>) {
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(range);
    thread::sleep(Duration::from_millis(sleep_time as u64));
}


pub fn test_enigo() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    // 暂时注释掉 enigo 的测试代码
    // println!("Testing Ctrl+K...");
    // enigo.key(Key::Control, Direction::Press);
    // enigo.key(Key::Unicode('k'), Direction::Click);
    // enigo.key(Key::Control, Direction::Release);
    // thread::sleep(Duration::from_secs(1));

    println!("Testing Ctrl+L...");
    enigo.key(Key::Control, Press);
    enigo.key(Key::Unicode('l'), Click);
    enigo.key(Key::Control, Release);
    thread::sleep(Duration::from_secs(1));

    println!("Testing text input...");
    enigo.text("中文");
    thread::sleep(Duration::from_secs(1));

    println!("Keyboard shortcuts test is currently disabled.");
} 

pub fn test_hot_words() {
    let hot_words = HotWordsData::load_or_create().unwrap();
    println!("{:?}", hot_words);
}

pub fn test_human_typing() {
    let mut bot = SearchBot::new().unwrap();
    bot.human_typing("一段非常长的测试文本，用于测试人类输入速度");
}
