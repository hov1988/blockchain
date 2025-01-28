use crate::config::Config;
use transaction::*;
pub mod transaction;

use crate::core::wallet::{Wallet, WalletTransaction};
use std::ops::Index;
use std::time::Instant;
pub mod block;

use block::Block;
pub trait Serialization<T> {
    fn serialization(&self) -> Vec<u8>;
    fn deserialization(bytes: Vec<u8>) -> T;
}

pub enum BlockSearch {
    SearchByIndex(usize),
    SearchByPreviousHash(Vec<u8>),
    SearchByBlockHash(Vec<u8>),
    SearchByNonce(i32),
    SearchByTimeStamp(u128),
    SearchByTransaction(Vec<u8>),
}

pub enum BlockSearchResult<'a> {
    Success(&'a Block),
    FailOfEmptyBlocks,
    FailOfIndex(usize),
    FailOfPreviousHash(Vec<u8>),
    FailOfBlockHash(Vec<u8>),
    FailOfNonce(i32),
    FailOfTimeStamp(u128),
    FailOfTransaction(Vec<u8>),
}

#[derive(Debug)]
pub struct BlockChain {
    pub config: Config,
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
    blockchain_address: String,
}

impl Index<usize> for BlockChain {
    type Output = Block;
    fn index(&self, index: usize) -> &Self::Output {
        self.chain
            .get(index)
            .expect("index out of range for the chain")
    }
}

impl BlockChain {
    pub fn new(config: Config, address: String) -> Self {
        let mut bc = BlockChain {
            config,
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
            blockchain_address: address,
        };

        let b = Block::new(0, vec![0 as u8; 32]);
        bc.chain.push(b);
        bc.mining();

        bc
    }

    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let mut b = Block::new(nonce, previous_hash);
        for tx in self.transaction_pool.iter() {
            b.transactions.push(tx.clone());
        }
        self.transaction_pool.clear();
        let now = Instant::now();
        let proof_hash = BlockChain::do_proof_of_work(&mut b, self.config.difficulty);
        let elapsed = now.elapsed();
        println!(
            "compute time: {:?}\nproof for the current block is :{:?}",
            elapsed, proof_hash
        );
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
        if self.chain.len() > 1 {
            return &self.chain[self.chain.len() - 1];
        }

        &self.chain[0]
    }

    pub fn search_block(&self, search: BlockSearch) -> BlockSearchResult {
        for (idx, block) in self.chain.iter().enumerate() {
            match search {
                BlockSearch::SearchByIndex(index) => {
                    if idx == index {
                        return BlockSearchResult::Success(block);
                    }

                    if index >= self.chain.len() {
                        return BlockSearchResult::FailOfIndex(index);
                    }
                }
                BlockSearch::SearchByPreviousHash(ref hash) => {
                    if block.previous_hash == *hash {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfPreviousHash(hash.to_vec());
                    }
                }

                BlockSearch::SearchByBlockHash(ref hash) => {
                    if block.hash() == *hash {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfBlockHash(hash.to_vec());
                    }
                }

                BlockSearch::SearchByNonce(nonce) => {
                    if block.nonce == nonce {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfNonce(nonce);
                    }
                }

                BlockSearch::SearchByTimeStamp(time_stamp) => {
                    if block.time_stamp == time_stamp {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfTimeStamp(time_stamp);
                    }
                }

                BlockSearch::SearchByTransaction(ref transaction) => {
                    for tx in block.transactions.iter() {
                        if tx == transaction {
                            return BlockSearchResult::Success(block);
                        }

                        if idx >= self.chain.len() {
                            return BlockSearchResult::FailOfTransaction(transaction.to_vec());
                        }
                    }
                }
            }
        }

        BlockSearchResult::FailOfEmptyBlocks
    }

    pub fn add_transaction(&mut self, tx: &WalletTransaction) -> bool {
        if tx.sender == self.blockchain_address {
            println!("minner cannot send money to himself");
            return false;
        }

        if tx.sender != self.config.sender && !Wallet::verify_transaction(tx) {
            println!("invalid transaction");
            return false;
        }

        if tx.sender != self.config.sender
            && self.calculate_total_amount(tx.sender.clone()) < tx.amount as i64
        {
            println!("sender dose not have enough balance");
            return false;
        }

        let transaction = Transaction::new(
            tx.sender.as_bytes().to_vec(),
            tx.recipient.as_bytes().to_vec(),
            tx.amount,
        );

        for tx_in_pool in self.transaction_pool.iter() {
            if *tx_in_pool == transaction.serialization() {
                break;
            }
        }

        self.transaction_pool.push(transaction.serialization());
        true
    }

    fn do_proof_of_work(block: &mut Block, difficulty: usize) -> String {
        loop {
            let hash = block.hash();
            let hash_str = hex::encode(&hash);
            if hash_str[0..difficulty] == "0".repeat(difficulty) {
                return hash_str;
            }

            *block += 1;
        }
    }

    pub fn mining(&mut self) -> bool {
        let tx = WalletTransaction {
            sender: self.config.sender.into(),
            recipient: self.blockchain_address.clone().into(),
            amount: self.config.reward,
            public_key: "".to_string(),
            signature: "".to_string(),
        };

        self.add_transaction(&tx);
        self.create_block(0, self.last_block().hash());

        true
    }

    pub fn calculate_total_amount(&self, address: String) -> i64 {
        let mut total_amount: i64 = 0;
        for i in 0..self.chain.len() {
            let block = &self[i];
            for t in block.transactions.iter() {
                let tx = Transaction::deserialization(t.clone());
                let value = tx.value;

                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.recipient_address {
                    total_amount += value as i64;
                }

                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.sender_address {
                    total_amount -= value as i64;
                }
            }
        }

        total_amount
    }
}
