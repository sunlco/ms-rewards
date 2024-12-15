mod bot;
mod hot_words;
mod utils;

use bot::SearchBot;

#[tokio::main]
async fn main() {
    // 添加测试模式
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--test" {
        println!("Running keyboard tests...");
        return;
    }

    // 正常运行模式
    match SearchBot::new() {
        Ok(mut bot) => {
            if let Err(e) = bot.run().await {
                eprintln!("Error: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to create SearchBot: {}", e),
    }
}
