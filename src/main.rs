pub mod blockchain;
use blockchain::BlockChain;
use crate::blockchain::BlockSearchResult;

fn get_block_search_result(result: BlockSearchResult) {
    match result {
        BlockSearchResult::Success(block) => {
            println!("find given block: {:?}", block);
        }

        BlockSearchResult::FailOfEmptyBlocks => {
            println!("no block in the chain");
        }

        BlockSearchResult::FailOfIndex(idx) => {
            println!("fail to find block with index: {}", idx);
        }

        BlockSearchResult::FailOfPreviousHash(hash) => {
            println!("not block hash given previous hash: {:?}", hash);
        }

        BlockSearchResult::FailOfBlockHash(hash) => {
            println!("not block has hash as :{:?}", hash);
        }

        BlockSearchResult::FailOfNonce(nonce) => {
            println!("not block has nonce with value: {}", nonce);
        }

        BlockSearchResult::FailOfTimestamp(time_stamp) => {
            println!("not block has given time stamp: {}", time_stamp);
        }

        BlockSearchResult::FailOfTransaction(tx) => {
            println!("not block contains given trasaction: {:?}", tx);
        }
        _ => {}
    }
}

fn main() {
    let mut block_chain = BlockChain::new();
    block_chain.print();
    let prev_hash = block_chain.last_block().hash();

    block_chain.create_block(1, prev_hash);

    let prev_hash = block_chain.last_block().hash();
    block_chain.create_block(2, prev_hash);
    block_chain.print();
}
