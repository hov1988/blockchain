pub mod blockchain;
use blockchain::BlockChain;

fn main() {
    let mut block_chain = BlockChain::new();
    block_chain.print();
    let prev_hash = block_chain.last_block().hash();

    block_chain.create_block(1, prev_hash);

    let prev_hash = block_chain.last_block().hash();
    block_chain.create_block(2, prev_hash);
    block_chain.print();
}
