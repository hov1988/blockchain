pub mod blockchain;
use blockchain::BlockChain;

fn main() {
    let mut block_chain = BlockChain::new();
    block_chain.create_block(1, "hash 1".to_string().into_bytes());
    block_chain.create_block(2, "hash 2".to_string().into_bytes());
    block_chain.print();
}
