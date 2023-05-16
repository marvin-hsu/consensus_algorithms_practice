pub struct Block {}

impl Block {
    pub fn new(
        block_data: &str,
        previous_block_hash: &str,
        target_difficulty: u32,
        nonce: usize,
    ) -> Block {
        Block {}
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
