use sha2::{Digest, Sha256};
use std::ops::AddAssign;
use std::time::SystemTime;
use crate::blockchain::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub nonce: i32,
    pub previous_hash: Vec<u8>,
    pub time_stamp: u128,
    pub transactions: Vec<Vec<u8>>,
}

impl AddAssign<i32> for Block {
    fn add_assign(&mut self, rhs: i32) {
        self.nonce += rhs;
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl Block {
    pub fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
        let time_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Block {
            nonce,
            previous_hash,
            time_stamp: time_now.as_nanos(),
            transactions: Vec::new(),
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::new();
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(&self.previous_hash);
        bin.extend(self.time_stamp.to_be_bytes());
        for tx in &self.transactions {
            bin.extend(tx.clone());
        }
        let mut hasher = Sha256::new();
        hasher.update(bin);
        hasher.finalize().to_vec()
    }

    pub fn print(&self) {
        println!("timestamp: {:x}", self.time_stamp);
        println!("nonce: {}", self.nonce);
        println!("previous_hash: {:?}", self.previous_hash);
    }
}
