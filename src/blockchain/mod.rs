use std::time::SystemTime;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    nonce: u32,
    previous_hash: Vec<u8>,
    time_stamp: u128,
    transactions: Vec<Vec<u8>>,
}

impl Block {
    fn new (nonce: u32, previous_hash: Vec<u8>) -> Self {
        let time_new = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

        Block {
            nonce,
            previous_hash,
            time_stamp: time_new.as_millis(),
            transactions: Vec::<Vec<u8>>::new()
        }
    }

    fn print(&self) {
        println!("timestamp: {:x}", self.time_stamp);
        println!("nonce: {}", self.nonce);
        println!("previous_hash: {:?}", self.previous_hash);
        println!("transactions: {:?}", self.transactions)
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();

        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamp.to_be_bytes());

        for tx in self.transactions.iter() {
            bin.extend(tx.clone());
        }

        let mut hasher = Sha256::new();
        hasher.update(bin);
        hasher.finalize().to_vec()
    }
}

#[derive(Debug)]
pub struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>
}

impl BlockChain {
    pub fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };

        bc.create_block(0, vec![0u8; 32]);
        bc
    }

    pub fn create_block(&mut self, nonce: u32, previous_hash:Vec<u8>) {
        let b = Block::new(nonce, previous_hash);
        self.chain.push(b);
    }

    pub fn print(&self) {
        for (i, block) in self.chain.iter().enumerate() {
            println!("{} Chain {} {}", "=".repeat(25), i, "=".repeat(25));
            block.print();
        }
        println!("{}", "*".repeat(25));
    }

    pub fn last_block(&self) -> &Block {
        if (&self.chain.len() > &1) {
            return &self.chain[&self.chain.len() - 1];
        }

        &self.chain[0]
    }
}