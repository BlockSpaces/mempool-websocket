use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MempoolMessage {
    action: String,
    data: Vec<String>,
}

impl MempoolMessage {
    pub fn new(action: String, data: Vec<String>) -> Self {
        Self { action, data }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MempoolBlockExtrasPool {
    id: u32,
    name: String,
    slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MempoolBlockExtras {
    reward: u32,
    #[serde(rename = "coinbaseRaw")]
    coinbase_raw: String,
    orphans: Vec<String>,
    #[serde(rename = "medianFee")]
    median_fee: u32,
    #[serde(rename = "feeRange")]
    fee_range: Vec<u32>,
    #[serde(rename = "totalFees")]
    total_fees: u32,
    #[serde(rename = "avgFee")]
    avg_fee: u32,
    #[serde(rename = "avgFeeRate")]
    avg_fee_rate: u32,
    #[serde(rename = "utxoSetChange")]
    utxo_set_change: u32,
    #[serde(rename = "avgTxSize")]
    avg_tx_size: u32,
    #[serde(rename = "totalInputs")]
    total_inputs: u32,
    #[serde(rename = "totalOutputs")]
    total_outputs: u32,
    #[serde(rename = "totalOutputAmt")]
    total_output_amt: u32,
    #[serde(rename = "segwitTotalTxs")]
    segwit_total_txs: u32,
    #[serde(rename = "segwitTotalSize")]
    segwit_total_size: u32,
    #[serde(rename = "segwitTotalWeight")]
    segwit_total_weight: u32,
    #[serde(rename = "virtualSize")]
    virtual_size: u32,
    #[serde(rename = "coinbaseAddress")]
    coinbase_address: String,
    #[serde(rename = "coinbaseSignature")]
    coinbase_signature: String,
    #[serde(rename = "coinbaseSignatureAscii")]
    coinbase_signature_ascii: String,
    header: String,
    #[serde(rename = "utxoSetSize")]
    utxo_set_size: Option<u32>,
    #[serde(rename = "totalInputAmt")]
    total_input_amt: Option<u32>,
    pool: MempoolBlockExtrasPool,
    #[serde(rename = "matchRate")]
    match_rate: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MempoolBlock {
    id: String,
    pub height: u32,
    version: u32,
    timestamp: u32,
    bits: u32,
    nonce: u32,
    difficulty: f32,
    merkle_root: String,
    tx_count: u32,
    size: u32,
    weight: u32,
    previousblockhash: String,
    mediantime: u32,
    extras: MempoolBlockExtras,
}

#[derive(Debug, Serialize, Deserialize)]
struct MempoolInfo {
    loaded: bool,
    size: u32,
    bytes: u32,
    usage: u32,
    total_fee: u32,
    maxmempool: u32,
    mempoolminfee: f32,
    minrelaytxfee: f32,
    incrementalrelayfee: f32,
    unbroadcastcount: u32,
    fullrbf: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct MempoolDa {
    #[serde(rename = "progressPercent")]
    progress_percent: f32,
    #[serde(rename = "difficultyChange")]
    difficulty_change: u32,
    #[serde(rename = "estimatedRetargetDate")]
    estimated_retarget_date: u64,
    #[serde(rename = "remainingBlocks")]
    remaining_blocks: u32,
    #[serde(rename = "remainingTime")]
    remaining_time: u32,
    #[serde(rename = "previousRetarget")]
    previous_retarget: u32,
    #[serde(rename = "previousTime")]
    previous_time: u32,
    #[serde(rename = "nextRetargetHeight")]
    next_retarget_height: u32,
    #[serde(rename = "timeAvg")]
    time_avg: u32,
    #[serde(rename = "timeOffset")]
    time_offset: u32,
    #[serde(rename = "expectedBlocks")]
    expected_blocks: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MempoolFees {
    #[serde(rename = "fastestFee")]
    fastest_fee: u32,
    #[serde(rename = "halfHourFee")]
    half_hour_fee: u32,
    #[serde(rename = "hourFee")]
    hour_fee: u32,
    #[serde(rename = "economyFee")]
    economy_fee: u32,
    #[serde(rename = "minimumFee")]
    minimum_fee: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MempoolBlockResponse {
    pub block: MempoolBlock,
    #[serde(rename = "mempoolInfo")]
    mempool_info: MempoolInfo,
    da: MempoolDa,
    fees: MempoolFees,
}
