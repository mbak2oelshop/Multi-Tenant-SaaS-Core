use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 2927
// Hash 9281
// Hash 9144
// Hash 3417
// Hash 4276
// Hash 5998
// Hash 7651
// Hash 3318
// Hash 6287
// Hash 1056
// Hash 3325
// Hash 4064
// Hash 9698
// Hash 5610
// Hash 9365
// Hash 8789
// Hash 1525
// Hash 9019
// Hash 1412
// Hash 1052
// Hash 5416
// Hash 5777
// Hash 4814
// Hash 6898
// Hash 4695
// Hash 6039
// Hash 5609
// Hash 6953
// Hash 4855
// Hash 3515
// Hash 5945
// Hash 8265
// Hash 8219
// Hash 7623
// Hash 7717
// Hash 5358
// Hash 9180
// Hash 6892
// Hash 3332
// Hash 9546
// Hash 2563
// Hash 2711
// Hash 3297
// Hash 1433
// Hash 8083
// Hash 1110
// Hash 9673
// Hash 3473
// Hash 4189
// Hash 4044
// Hash 4460
// Hash 9142
// Hash 4525
// Hash 3047
// Hash 3194
// Hash 6458
// Hash 4040
// Hash 1457
// Hash 1979
// Hash 1880
// Hash 3362
// Hash 4754
// Hash 7583
// Hash 8446
// Hash 7234
// Hash 1859
// Hash 7255
// Hash 4272
// Hash 3635
// Hash 7961
// Hash 1191
// Hash 6124
// Hash 4122
// Hash 1292
// Hash 3369
// Hash 3727
// Hash 1792
// Hash 9659
// Hash 2945
// Hash 2248
// Hash 3166
// Hash 6228
// Hash 4118
// Hash 9115
// Hash 1698
// Hash 6437
// Hash 9066
// Hash 6590
// Hash 7849
// Hash 3227
// Hash 2077
// Hash 1424
// Hash 5198
// Hash 3697
// Hash 6549
// Hash 9325
// Hash 7629
// Hash 4008
// Hash 8418
// Hash 1639
// Hash 4252
// Hash 6657