#[derive(Debug)]
pub struct Config {
    pub difficulty: usize,
    pub sender: &'static str,
    pub reward: u64,
}

impl Config {
    pub fn default() -> Self {
        Config {
            difficulty: 3,
            sender: "THE BLOCKCHAIN",
            reward: 1,
        }
    }
}
