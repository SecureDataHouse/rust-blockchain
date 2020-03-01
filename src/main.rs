pub mod basic_prototype;
pub mod proof_of_work;

use std::thread;
use crate::basic_prototype::Block;
use crate::basic_prototype::Blockchain;
use std::time::Duration;
use num_bigint::BigUint;
use crate::proof_of_work::ProofOfWork;

fn main() {
    let mut bc: Blockchain = Blockchain {
        blocks: Vec::<Block>::new(),
    };
    bc.new_blockchain();
    thread::sleep(Duration::from_secs(1));
    bc.add_block("Send 1 BTC to Ivan".to_string());
    thread::sleep(Duration::from_secs(1));
    bc.add_block("Send 2 more BTC to Ivan".to_string());

    for mut block in bc.blocks {
        let mut pow = ProofOfWork {
            block: Block {
                timestamp: 0,
                data: "".to_string(),
                prev_block_hash: [0; 32],
                hash: [0; 32],
                nonce: 0,
            },
            target: BigUint::new(Vec::new()),
        };
        pow.new_proof_of_work();
        let (nonce, hash) = pow.run();
        block.hash = hash;
        block.nonce = nonce;
        pow.block.timestamp = block.timestamp;
        pow.block.data = block.data;
        pow.block.prev_block_hash = block.prev_block_hash;
        pow.block.hash = block.hash;
        pow.block.nonce = block.nonce;
        let is_valid: bool = pow.validate();
        println!("{:?}", pow.block);
        println!("{}", is_valid);
    }
}
