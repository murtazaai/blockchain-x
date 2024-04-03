use chrono::Utc;
use sha256::digest;

#[derive(Debug, Clone)]
struct Blockchain {
    blocks: Vec<Block>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    hash: String,
    previous_hash: String,
    timestamp: i64,
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let timestamp = Utc::now().timestamp();

        let (nonce, hash) = Block::mine_block(id, &data, &previous_hash, timestamp);

        Self {
            id,
            nonce,
            data,
            hash,
            previous_hash,
            timestamp,
        }
    }

    fn mine_block(id: u64, data: &str, previous_hash: &str, timestamp: i64) -> (u64, String) {
        println!("mining block...");

        let mut nonce = 1;

        loop {
            let block_string = format!("{}{}{}{}{}", id, data, previous_hash, timestamp, nonce);

            let hash = digest(block_string);

            if hash.starts_with("0000") {
                println!("mined nonce: {}, hash: hash {}", nonce, hash);
                return (nonce, hash)
            }
            nonce += 1;
        }

    }
}

impl Blockchain {
    fn new() -> Self {
        Self {
            blocks: vec![],
        }
    }

    fn genesis_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            nonce: 1234,
            data: String::from("Genesis Block"),
            hash: String::from("0000111111111111111111111111111111111111111111111111111111111111"),
            previous_hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            timestamp: Utc::now().timestamp(),
        };

        self.blocks.push(genesis_block);
    }

    fn add_block(&mut self, block: Block) {
        match self.blocks.last() {
            None => self.genesis_block(),
            Some(last_block) => {
                if self.is_block_valid(&block, last_block) {
                    self.blocks.push(block);
                    println!("Block has been successfully added to the blockchain.")
                } else {
                    println!("Block could not be added to the blockchain. Invalid block.")
                }
            }
        }
    }

    fn is_block_valid(&self, block: &Block, last_block: &Block) -> bool {
        if block.previous_hash != last_block.hash {
            println!("Block with id {} has a wrong previous hash", block.id);
            false
        } else if !block.hash.starts_with("0000") {
            println!("Block with id {} has an invalid hash", block.id);
            false
        } else if block.id != last_block.id + 1 {
            println!("Block with id: {} is not the next block", block.id);
            false
        } else if digest(format!("{}{}{}{}{}", block.id, block.data, block.previous_hash, block.timestamp, block.nonce)) != block.hash {
            println!("Block with id: {} has an invalid hash", block.id);
            false
        } else {
            true
        }
    }

    fn is_chain_valid(&self, chain: &Vec<Block>) -> bool {
        match chain.len() {
            0 => println!("The chain is empty"),
            1 => println!("The chain contains only genesis block"),
            _ => {
                for a in 1..chain.len() {
                    let previous_block = chain.get(a-1).unwrap();
                    let current_block = chain.get(a).unwrap();
                    if !self.is_block_valid(current_block, previous_block) {
                        return false;
                    }
                }
            }
        }
        println!("The chain is valid.");
        true
    }
}

fn main() {

    let mut blockchain = Blockchain::new();

    blockchain.genesis_block();

    println!("{:?}", blockchain);

    let block = Block::new(2, blockchain.blocks[0].hash.to_owned(), String::from("Data"));

    blockchain.add_block(block);

    blockchain.is_chain_valid(&blockchain.blocks);

    println!("{:?}", blockchain);

    let block = Block::new(3, blockchain.blocks[0].hash.to_owned(), String::from("Data1"));

    blockchain.add_block(block);

    blockchain.is_chain_valid(&blockchain.blocks);

    println!("{:?}", blockchain);

    let block = Block::new(4, blockchain.blocks[0].hash.to_owned(), String::from("Data2"));

    blockchain.add_block(block);

    blockchain.is_chain_valid(&blockchain.blocks);

    println!("{:?}", blockchain);

}
