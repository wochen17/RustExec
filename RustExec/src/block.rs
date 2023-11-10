use std::time::SystemTime;
use clap::Error;

pub struct Block {
    timestamp: u128,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: u32,
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Block {
    pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block, Error> {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
        let mut block = Block {
            timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_if_work()?;
        Ok(block);
    }

    pub fn run_proof_if_work(&mut self) -> Result<(), Error> {

    }
}
