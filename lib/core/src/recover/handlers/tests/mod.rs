#![cfg(test)]
// Module declaration for test files
pub mod handle_chain_receive_swap_tests;
pub mod handle_chain_receive_swap_tests_integration;
pub mod handle_chain_send_swap_tests;
pub mod handle_chain_send_swap_tests_integration;
pub mod handle_receive_swap_tests;
pub mod handle_receive_swap_tests_integration;
pub mod handle_send_swap_tests;
pub mod handle_send_swap_tests_integration;

// Helper function to create a HistoryTxId for testing
mod test {
    use std::{collections::BTreeMap, str::FromStr};

    use crate::recover::model::HistoryTxId;
    use lwk_wollet::{
        elements::{self, AssetId, Transaction, TxIn, TxInWitness, Txid},
        hashes::Hash,
        WalletTx,
    };

    pub(crate) fn create_history_txid(hex_id: &str, height: i32) -> HistoryTxId {
        let txid_bytes = hex::decode(format!("{:0>64}", hex_id)).unwrap();
        let mut txid_array = [0u8; 32];
        txid_array.copy_from_slice(&txid_bytes);

        HistoryTxId {
            txid: Txid::from_slice(&txid_array).unwrap(),
            height,
        }
    }

    // Create an empty LBTC transaction
    fn create_empty_lbtc_transaction() -> Transaction {
        Transaction {
            version: 2,
            lock_time: elements::LockTime::from_height(0).unwrap(),
            input: vec![TxIn {
                previous_output: Default::default(),
                is_pegin: false,
                script_sig: elements::Script::new(),
                sequence: elements::Sequence::default(),
                asset_issuance: Default::default(),
                witness: TxInWitness::empty(),
            }],
            output: vec![],
        }
    }

    // Create a mock wallet transaction
    pub(crate) fn create_mock_wallet_tx(tx_id_hex: &str, height: u32, amount: i64) -> WalletTx {
        create_mock_lbtc_wallet_tx(tx_id_hex, height, amount)
        // let tx_id = Txid::from_str(tx_id_hex).unwrap();

        // // Create balance for the transaction
        // let mut balance = BTreeMap::new();
        // balance.insert(AssetId::default(), amount as i64);

        // WalletTx {
        //     txid: tx_id,
        //     tx: create_empty_transaction(),
        //     height: Some(height),
        //     fee: 1000,
        //     timestamp: Some(1001), // Just after swap creation time
        //     balance,
        //     outputs: Vec::new(),
        //     inputs: Vec::new(),
        //     type_: "".to_string(),
        // }
    }

    // Create a mock LBTC wallet transaction
    pub(crate) fn create_mock_lbtc_wallet_tx(
        tx_id_hex: &str,
        height: u32,
        amount: i64,
    ) -> WalletTx {
        let tx_id = Txid::from_str(tx_id_hex).unwrap();

        WalletTx {
            txid: tx_id,
            tx: create_empty_lbtc_transaction(),
            height: Some(height),
            fee: 1000,
            timestamp: Some(1001), // Just after swap creation time
            balance: {
                let mut map = BTreeMap::new();
                map.insert(
                    AssetId::from_slice(&[0; 32]).unwrap(), // Default asset ID
                    amount,
                );
                map
            },
            outputs: vec![],
            inputs: Vec::new(),
            type_: "".to_string(),
        }
    }
}
