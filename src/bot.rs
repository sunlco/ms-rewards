#![allow(dead_code)]
#![allow(unused_variables)]
use crate::hot_words::HotWordsData;
use std::error::Error;
use std::thread;
use std::time::Duration;
use rand::Rng;
use enigo::{
    Button, Coordinate, Axis, 
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
    {Coordinate::Abs, Coordinate::Rel},
};
use crate::utils::random_sleep;

const COMMON_WAIT_TIME: Duration = Duration::from_secs(3);

pub struct SearchBot {
    hot_words: HotWordsData,
    search_count: u32,
    enigo: Enigo,
}

impl SearchBot {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SearchBot {
            hot_words: HotWordsData::load_or_create()?,
            search_count: 0,
            enigo: Enigo::new(&Settings::default()).unwrap(),
        })
    }

    // 模拟人类输入
    pub fn human_typing(&mut self, text: &str) {
        // 将文本拆分为字符并逐个输入
        for c in text.chars() {
            // 输入单个字符
            self.enigo.text(&c.to_string());
            // 添加随机延迟，模拟人类输入速度
            let delay = rand::thread_rng().gen_range(100..300);
            thread::sleep(Duration::from_millis(delay));
        }
    }

    // 模拟人类滚动页面
    pub fn human_scroll(&mut self, height: i32) {

        let sign = if height > 0 { 1 } else { -1 };

        for _ in 0..height.abs() {
            self.enigo.scroll(1 * sign, Axis::Vertical);
            // 每次滚动间隔30-80毫秒,让滚动看起来更自然
            random_sleep(30..80);
        }

        random_sleep(500..1000);
    }

    // 打开浏览器
    fn open_browser(&mut self) -> Result<(), Box<dyn Error>> {
       #[cfg(target_os = "windows")]
       std::process::Command::new("cmd")
           .args(&["/C", "start", "msedge"])
           .output()?;

       #[cfg(target_os = "macos")]
       std::process::Command::new("open")
           .args(&["-a", "Microsoft Edge"])
           .output()?;
        
        Ok(())
    }

    // 关闭标签页
    fn close_tab(&mut self) {
        self.enigo.key(Key::Control, Press);
        self.enigo.key(Key::Unicode('w'), Click);
        self.enigo.key(Key::Control, Release);
        thread::sleep(COMMON_WAIT_TIME);
    }

    // 打开新标签页
    fn open_new_tab(&mut self) {
        self.enigo.key(Key::Control, Press);
        self.enigo.key(Key::Unicode('t'), Click);
        self.enigo.key(Key::Control, Release);
        thread::sleep(COMMON_WAIT_TIME);
    }

    // 模拟滚动页面
    fn scroll_page(&mut self, if_read: Option<bool>) {
        let mut rng = rand::thread_rng();
        // 随机滚动3-5次
        let scroll_times = rng.gen_range(3..5);
        
        let if_read = if_read.unwrap_or(false);
        
        for _ in 0..scroll_times {
            // 滚动距离1-5个单位
            let length = rng.gen_range(1..5);
            
            // 根据if_read参数决定滚动方向
            // if_read为true时只向下滚动,false时随机上下滚动
            let sign = if if_read {
                1
            } else {
                if rng.gen_bool(0.5) { 1 } else { -1 }
            };

            let scroll_distance = length * sign;
            self.human_scroll(scroll_distance);
            if if_read {
                thread::sleep(Duration::from_secs(3));
            }
            println!("滚动: {}", scroll_distance);
        }
    }

    // 随机点击搜索结果
    fn click_search_results(&mut self) {
        let mut rng = rand::thread_rng();

        // 假设搜索结果在页面的某个区域内
        // 这些值需要根据实际情况调整
        let x_min = 100;
        let x_max = 800;
        let y_min = 200;
        let y_max = 600;

        // 随机选择1到2个结果进行点击
        let click_count = rng.gen_range(1..=2);

        for _ in 0..click_count {
            let x = rng.gen_range(x_min..x_max);
            let y = rng.gen_range(y_min..y_max);

            // 移动鼠标到随机位置并点击
            self.enigo.move_mouse(x, y, Abs).unwrap();
            println!("点击: {} {}", x, y);
            random_sleep(1000..3000);
            self.enigo.button(Button::Left, Click).unwrap();
            // 等待页面加载完成
            random_sleep(2000..5000);
            // 滚动页面，模拟阅读
            self.scroll_page(Some(true));
            // 结尾处阅读一定时间
            random_sleep(5000..10000);
            // 关闭标签页
            self.close_tab();
        }
    }

    fn search_and_read(&mut self, word: &str) {
        // 输入搜索词
        self.human_typing(word);
        // 按下确认键
        self.enigo.key(Key::Return, Click);
        println!("当前搜索词为 {}", word);
        // 等待3-5秒，确保页面加载完成
        random_sleep(3000..5000);
        // 滚动页面
        self.scroll_page(None);
        // 点击搜索结果
        self.click_search_results();
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // 更新热搜词
        self.hot_words.update_if_needed().await?;
        
        // 打开浏览器
        self.open_browser();
        thread::sleep(COMMON_WAIT_TIME);

        // 获取未搜索的词列表
        let unsearched_words: Vec<_> = self.hot_words.get_unsearched_words().iter().map(|w| w.title.clone()).collect();
        for word in unsearched_words {
            if self.search_count >= 30 {
                break;
            }

            self.open_new_tab();
            self.search_and_read(&word);

            // 标记为已搜索
            self.hot_words.mark_as_searched(&word)?;
            self.search_count += 1;
            
            // 使用 rand::thread_rng() 生成随机数
            let wait_time = rand::thread_rng().gen_range(3..8);
            thread::sleep(Duration::from_secs(wait_time));
        }

        Ok(())
    }
} 