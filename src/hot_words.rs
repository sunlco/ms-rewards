use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use chrono::Local;

#[derive(Debug, Serialize, Deserialize)]
pub struct HotWord {
    pub title: String,
    pub searched: bool,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotWordsData {
    pub last_update: i64,
    pub words: Vec<HotWord>,
}

impl HotWordsData {
    pub fn new() -> Self {
        Self {
            last_update: 0,
            words: Vec::new(),
        }
    }

    pub fn load_or_create() -> Result<Self, Box<dyn Error>> {
        let path = "hot_words.json";
        if Path::new(path).exists() {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("hot_words.json")?;
        
        file.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;
        Ok(())
    }

    pub fn needs_update(&self) -> bool {
        let now = Local::now().timestamp();
        now - self.last_update >= 6 * 3600 // 6 小时更新一次
    }

    pub async fn update_if_needed(&mut self) -> Result<(), Box<dyn Error>> {
        if self.needs_update() {
            let resp = reqwest::get("https://uapis.cn/api/hotlist?type=douyin")
                .await?
                .text()
                .await?;
            
            let json: serde_json::Value = serde_json::from_str(&resp)?;
            
            if let Some(data) = json["data"].as_array() {
                self.words = data.iter()
                    .filter_map(|item| item["title"].as_str())
                    .map(|title| HotWord {
                        title: title.to_string(),
                        searched: false,
                        timestamp: None,
                    })
                    .collect();
                self.last_update = Local::now().timestamp();
                self.save()?;
            }
        }
        Ok(())
    }

    pub fn get_unsearched_words(&self) -> Vec<&HotWord> {
        self.words.iter()
            .filter(|word| !word.searched)
            .collect()
    }

    pub fn mark_as_searched(&mut self, title: &str) -> Result<(), Box<dyn Error>> {
        if let Some(word) = self.words.iter_mut().find(|w| w.title == title) {
            word.searched = true;
            word.timestamp = Some(Local::now().timestamp());
            self.save()?;
        }
        Ok(())
    }
} 