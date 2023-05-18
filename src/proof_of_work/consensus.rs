/// 交易內容
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u32,
    pub fee: u32,
    pub message: String,
}

/// 區塊
pub struct Block {
    pub previous_hash: String,
    pub hash: Option<String>,
    pub difficuity: u32,
    pub timestamp: Option<u32>,
    pub nonce: Option<u32>,
    pub transactions: Vec<Transaction>,
    pub miner: Option<String>,
    pub miner_rewards: u32,
}

/// 區塊鏈
pub struct BlockChain {
    pub adjust_difficuity_unit: u32,
    pub difficuity: u32,
    pub block_time: u32,
    pub mining_rewards: u32,
    pub block_limitation: u32,
    pub chain: Vec<Block>,
    pub pending_transaction: Vec<Transaction>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u32, fee: u32, message: String) -> Self {
        Self {
            sender,
            receiver,
            amount,
            fee,
            message,
        }
    }
}

impl Block {
    pub fn new(previous_hash: &str, difficuity: u32, miner_rewards: u32) -> Block {
        Block {
            previous_hash: previous_hash.to_owned(),
            hash: None,
            difficuity,
            timestamp: None,
            nonce: None,
            transactions: vec![],
            miner: None,
            miner_rewards,
        }
    }
}

impl BlockChain {
    pub fn new() -> Self {
        Self {
            adjust_difficuity_unit: 10,
            difficuity: 1,
            block_time: 30,
            mining_rewards: 10,
            block_limitation: 32,
            chain: vec![],
            pending_transaction: vec![],
        }
    }
}

pub fn validate_block(block: &Block, target_difficulty: u32) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_block() {
        // Arrange
        let block_data = "Block Data";
        let previous_block_hash =
            "0000000000000000000000000000000000000000000000000000000000000000";
        let target_difficulty = 4;
        let nonce = 12345;

        // Act
        let block = Block::new(block_data, previous_block_hash, target_difficulty, nonce);

        // Assert
        assert!(validate_block(&block, target_difficulty));
    }
}
