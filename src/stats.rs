use crate::game::Stats;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Statistic {
    pub total_statistic: Stats,
}
impl Statistic {
    pub fn load() -> Result<Self> {
        let path = get_path();
        if !path.exists() {
            let stats = Statistic::default();
            stats.save()?;
            Ok(stats)
        } else {
            let text_content = fs::read_to_string(path)?;
            let stats = serde_json::from_str(&text_content)?;
            Ok(stats)
        }
    }
    pub fn save(&self) -> Result<()> {
        let path = get_path();
        fs::create_dir_all(path.parent().unwrap() )?;
        let text_content = serde_json::to_string_pretty(&self)?;
        fs::write(&path, &text_content)?;
        Ok(())
    }
    pub fn add_stats(&mut self, stat: &Stats) {
        self.total_statistic.total_amount += stat.total_amount;
        self.total_statistic.total_correct += stat.total_correct;
        self.total_statistic.total_wrong += stat.total_wrong;
        self.total_statistic.total_time += stat.total_time;
    }
}

fn get_path() -> PathBuf {
    dirs::data_local_dir().unwrap().join("math_thing/data.json")
}
