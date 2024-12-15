// 暂时不使用 enigo 相关的代码
use std::time::Duration;
use std::thread;
use rand::Rng;


pub fn random_sleep(range: std::ops::Range<usize>) {
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(range);
    thread::sleep(Duration::from_millis(sleep_time as u64));
}
