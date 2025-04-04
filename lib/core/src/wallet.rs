use std::collections::HashMap;
use std::fs::{self, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use std::{path::Path, str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use boltz_client::ElementsAddress;
use log::{debug, info, warn};
use lwk_common::Signer as LwkSigner;
use lwk_common::{singlesig_desc, Singlesig};
use lwk_wollet::elements::{AssetId, Txid};
use lwk_wollet::ElectrumOptions;
use lwk_wollet::{
    elements::{hex::ToHex, Address, Transaction},
    ElectrumClient, ElectrumUrl, ElementsNetwork, FsPersister, WalletTx, Wollet, WolletDescriptor,
};
use sdk_common::bitcoin::hashes::{sha256, Hash};
use sdk_common::bitcoin::secp256k1::PublicKey;
use sdk_common::lightning::util::message_signing::verify;
use tokio::sync::Mutex;

use crate::model::Signer;
use crate::persist::Persister;
use crate::signer::SdkLwkSigner;
use crate::{
    ensure_sdk,
    error::PaymentError,
    model::{Config, LiquidNetwork},
};
use lwk_wollet::secp256k1::Message;

static LN_MESSAGE_PREFIX: &[u8] = b"Lightning Signed Message:";

#[sdk_macros::async_trait]
pub trait OnchainWallet: Send + Sync {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError>;

    /// List all transactions in the wallet mapped by tx id
    async fn transactions_by_tx_id(&self) -> Result<HashMap<Txid, WalletTx>, PaymentError>;

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError>;

    /// Builds a drain tx.
    ///
    /// ### Arguments
    /// - `fee_rate_sats_per_kvb`: custom drain tx feerate
    /// - `recipient_address`: drain tx recipient
    /// - `enforce_amount_sat`: if set, the drain tx will only be built if the amount transferred is
    ///   this amount, otherwise it will fail with a validation error
    async fn build_drain_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        enforce_amount_sat: Option<u64>,
    ) -> Result<Transaction, PaymentError>;

    /// Build a transaction to send funds to a recipient. If building a transaction
    /// results in an InsufficientFunds error, attempt to build a drain transaction
    /// validating that the `amount_sat` matches the drain output.
    async fn build_tx_or_drain_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError>;

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError>;

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> u32;

    /// Get the public key of the wallet
    fn pubkey(&self) -> Result<String>;

    /// Get the fingerprint of the wallet
    fn fingerprint(&self) -> Result<String>;

    /// Sign given message with the wallet private key. Returns a zbase
    /// encoded signature.
    fn sign_message(&self, msg: &str) -> Result<String>;

    /// Check whether given message was signed by the given
    /// pubkey and the signature (zbase encoded) is valid.
    fn check_message(&self, message: &str, pubkey: &str, signature: &str) -> Result<bool>;

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError>;
}

pub(crate) struct LiquidOnchainWallet {
    config: Config,
    persister: Arc<Persister>,
    wallet: Arc<Mutex<Wollet>>,
    electrum_client: Mutex<Option<ElectrumClient>>,
    working_dir: String,
    pub(crate) signer: SdkLwkSigner,
}

impl LiquidOnchainWallet {
    pub(crate) fn new(
        config: Config,
        working_dir: &String,
        persister: Arc<Persister>,
        user_signer: Arc<Box<dyn Signer>>,
    ) -> Result<Self> {
        let signer = crate::signer::SdkLwkSigner::new(user_signer.clone())?;
        let wollet = Self::create_wallet(&config, working_dir, &signer)?;

        let working_dir_buf = PathBuf::from_str(working_dir)?;
        if !working_dir_buf.exists() {
            create_dir_all(&working_dir_buf)?;
        }

        Ok(Self {
            config,
            persister,
            wallet: Arc::new(Mutex::new(wollet)),
            electrum_client: Mutex::new(None),
            working_dir: working_dir.clone(),
            signer,
        })
    }

    fn create_wallet<P: AsRef<Path>>(
        config: &Config,
        working_dir: P,
        signer: &SdkLwkSigner,
    ) -> Result<Wollet> {
        let elements_network: ElementsNetwork = config.network.into();
        let descriptor = LiquidOnchainWallet::get_descriptor(signer, config.network)?;
        let mut lwk_persister =
            FsPersister::new(working_dir.as_ref(), elements_network, &descriptor)?;
        let wollet_res = Wollet::new(elements_network, lwk_persister, descriptor.clone());
        match wollet_res {
            Ok(wollet) => Ok(wollet),
            Err(
                lwk_wollet::Error::PersistError(_)
                | lwk_wollet::Error::UpdateHeightTooOld { .. }
                | lwk_wollet::Error::UpdateOnDifferentStatus { .. },
            ) => {
                warn!("Update error initialising wollet, wipping storage and retrying: {wollet_res:?}");
                let mut path = working_dir.as_ref().to_path_buf();
                path.push(elements_network.as_str());
                fs::remove_dir_all(&path)?;
                warn!("Wiping wallet in path: {:?}", path);
                lwk_persister = FsPersister::new(working_dir, elements_network, &descriptor)?;
                Ok(Wollet::new(
                    elements_network,
                    lwk_persister,
                    descriptor.clone(),
                )?)
            }
            Err(e) => Err(e.into()),
        }
    }

    fn get_descriptor(
        signer: &SdkLwkSigner,
        network: LiquidNetwork,
    ) -> Result<WolletDescriptor, PaymentError> {
        let is_mainnet = network == LiquidNetwork::Mainnet;
        let descriptor_str = singlesig_desc(
            signer,
            Singlesig::Wpkh,
            lwk_common::DescriptorBlindingKey::Slip77,
            is_mainnet,
        )
        .map_err(|e| anyhow!("Invalid descriptor: {e}"))?;
        Ok(descriptor_str.parse()?)
    }
}

#[sdk_macros::async_trait]
impl OnchainWallet for LiquidOnchainWallet {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError> {
        let wallet = self.wallet.lock().await;
        wallet.transactions().map_err(|e| PaymentError::Generic {
            err: format!("Failed to fetch wallet transactions: {e:?}"),
        })
    }

    /// List all transactions in the wallet mapped by tx id
    async fn transactions_by_tx_id(&self) -> Result<HashMap<Txid, WalletTx>, PaymentError> {
        let tx_map: HashMap<Txid, WalletTx> = self
            .transactions()
            .await?
            .iter()
            .map(|tx| (tx.txid, tx.clone()))
            .collect();
        Ok(tx_map)
    }

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        let lwk_wollet = self.wallet.lock().await;
        let address =
            ElementsAddress::from_str(recipient_address).map_err(|e| PaymentError::Generic {
                err: format!(
                    "Recipient address {recipient_address} is not a valid ElementsAddress: {e:?}"
                ),
            })?;
        let mut tx_builder = lwk_wollet::TxBuilder::new(self.config.network.into())
            .fee_rate(fee_rate_sats_per_kvb)
            .enable_ct_discount();
        if asset_id.eq(&self.config.lbtc_asset_id()) {
            tx_builder = tx_builder.add_lbtc_recipient(&address, amount_sat)?;
        } else {
            let asset = AssetId::from_str(asset_id)?;
            tx_builder = tx_builder.add_recipient(&address, amount_sat, asset)?;
        }
        let mut pset = tx_builder.finish(&lwk_wollet)?;
        self.signer
            .sign(&mut pset)
            .map_err(|e| PaymentError::Generic {
                err: format!("Failed to sign transaction: {e:?}"),
            })?;
        Ok(lwk_wollet.finalize(&mut pset)?)
    }

    async fn build_drain_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        enforce_amount_sat: Option<u64>,
    ) -> Result<Transaction, PaymentError> {
        let lwk_wollet = self.wallet.lock().await;

        let address =
            ElementsAddress::from_str(recipient_address).map_err(|e| PaymentError::Generic {
                err: format!(
                    "Recipient address {recipient_address} is not a valid ElementsAddress: {e:?}"
                ),
            })?;
        let mut pset = lwk_wollet
            .tx_builder()
            .drain_lbtc_wallet()
            .drain_lbtc_to(address)
            .fee_rate(fee_rate_sats_per_kvb)
            .enable_ct_discount()
            .finish()?;

        if let Some(enforce_amount_sat) = enforce_amount_sat {
            let pset_details = lwk_wollet.get_details(&pset)?;
            let pset_balance_sat = pset_details
                .balance
                .balances
                .get(&lwk_wollet.policy_asset())
                .unwrap_or(&0);
            let pset_fees = pset_details.balance.fee;

            ensure_sdk!(
                (*pset_balance_sat * -1) as u64 - pset_fees == enforce_amount_sat,
                PaymentError::Generic {
                    err: format!("Drain tx amount {pset_balance_sat} sat doesn't match enforce_amount_sat {enforce_amount_sat} sat")
                }
            );
        }

        self.signer
            .sign(&mut pset)
            .map_err(|e| PaymentError::Generic {
                err: format!("Failed to sign transaction: {e:?}"),
            })?;
        Ok(lwk_wollet.finalize(&mut pset)?)
    }

    async fn build_tx_or_drain_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        match self
            .build_tx(
                fee_rate_sats_per_kvb,
                recipient_address,
                asset_id,
                amount_sat,
            )
            .await
        {
            Ok(tx) => Ok(tx),
            Err(PaymentError::InsufficientFunds) if asset_id.eq(&self.config.lbtc_asset_id()) => {
                warn!("Cannot build tx due to insufficient funds, attempting to build drain tx");
                self.build_drain_tx(fee_rate_sats_per_kvb, recipient_address, Some(amount_sat))
                    .await
            }
            Err(e) => Err(e),
        }
    }

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError> {
        let tip = self.tip().await;
        let address = match self.persister.next_expired_reserved_address(tip)? {
            Some(reserved_address) => {
                debug!(
                    "Got reserved address {} that expired on block height {}",
                    reserved_address.address, reserved_address.expiry_block_height
                );
                ElementsAddress::from_str(&reserved_address.address)
                    .map_err(|e| PaymentError::Generic { err: e.to_string() })?
            }
            None => {
                let next_index = self.persister.next_derivation_index()?;
                let address_result = self.wallet.lock().await.address(next_index)?;
                let address = address_result.address().clone();
                let index = address_result.index();
                debug!(
                    "Got unused address {} with derivation index {}",
                    address, index
                );
                if next_index.is_none() {
                    self.persister.set_last_derivation_index(index)?;
                }
                address
            }
        };

        Ok(address)
    }

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> u32 {
        self.wallet.lock().await.tip().height()
    }

    /// Get the public key of the wallet
    fn pubkey(&self) -> Result<String> {
        Ok(self.signer.xpub()?.public_key.to_string())
    }

    /// Get the fingerprint of the wallet
    fn fingerprint(&self) -> Result<String> {
        Ok(self.signer.fingerprint()?.to_hex())
    }

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError> {
        let full_scan_started = Instant::now();

        // create electrum client if doesn't already exist
        let mut electrum_client = self.electrum_client.lock().await;
        if electrum_client.is_none() {
            let (tls, validate_domain) = match self.config.network {
                LiquidNetwork::Mainnet | LiquidNetwork::Testnet => (true, true),
                LiquidNetwork::Regtest => (false, false),
            };

            let electrum_url =
                ElectrumUrl::new(&self.config.liquid_electrum_url, tls, validate_domain)?;
            *electrum_client = Some(ElectrumClient::with_options(
                &electrum_url,
                ElectrumOptions { timeout: Some(3) },
            )?);
        }
        let client = electrum_client
            .as_mut()
            .ok_or_else(|| PaymentError::Generic {
                err: "Electrum client not initialized".to_string(),
            })?;

        // Use the cached derivation index with a buffer of 5 to perform the scan
        let last_derivation_index = self
            .persister
            .get_last_derivation_index()?
            .unwrap_or_default();
        let index_with_buffer = last_derivation_index + 5;
        let mut wallet = self.wallet.lock().await;

        let res = match lwk_wollet::full_scan_to_index_with_electrum_client(
            &mut wallet,
            index_with_buffer,
            client,
        ) {
            Ok(()) => Ok(()),
            Err(lwk_wollet::Error::UpdateHeightTooOld { .. }) => {
                warn!("Full scan failed with update height too old, wiping storage and retrying");
                let mut new_wallet =
                    Self::create_wallet(&self.config, &self.working_dir, &self.signer)?;
                lwk_wollet::full_scan_to_index_with_electrum_client(
                    &mut new_wallet,
                    index_with_buffer,
                    client,
                )?;
                *wallet = new_wallet;
                Ok(())
            }
            Err(e) => Err(e.into()),
        };

        self.persister
            .set_last_scanned_derivation_index(last_derivation_index)?;

        let duration_ms = Instant::now().duration_since(full_scan_started).as_millis();
        info!("lwk wallet full_scan duration: ({duration_ms} ms)");
        res
    }

    fn sign_message(&self, message: &str) -> Result<String> {
        // Prefix and double hash message
        let mut engine = sha256::HashEngine::default();
        engine.write_all(LN_MESSAGE_PREFIX)?;
        engine.write_all(message.as_bytes())?;
        let hashed_msg = sha256::Hash::from_engine(engine);
        let double_hashed_msg = Message::from_digest(sha256::Hash::hash(&hashed_msg).into_inner());
        // Get message signature and encode to zbase32
        let recoverable_sig = self.signer.sign_ecdsa_recoverable(&double_hashed_msg)?;
        Ok(zbase32::encode_full_bytes(recoverable_sig.as_slice()))
    }

    fn check_message(&self, message: &str, pubkey: &str, signature: &str) -> Result<bool> {
        let pk = PublicKey::from_str(pubkey)?;
        Ok(verify(message.as_bytes(), signature, &pk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Config;
    use crate::signer::SdkSigner;
    use crate::test_utils::persist::create_persister;
    use crate::wallet::LiquidOnchainWallet;
    use anyhow::Result;
    use tempdir::TempDir;

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_all]
    async fn test_sign_and_check_message() -> Result<()> {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let sdk_signer: Box<dyn Signer> = Box::new(SdkSigner::new(mnemonic, "", false).unwrap());
        let sdk_signer = Arc::new(sdk_signer);

        let config = Config::testnet(None);

        // Create a temporary directory for working_dir
        let temp_dir = TempDir::new("").unwrap();
        let working_dir = temp_dir.path().to_str().unwrap().to_string();

        create_persister!(storage);

        let wallet: Arc<dyn OnchainWallet> = Arc::new(
            LiquidOnchainWallet::new(config, &working_dir, storage, sdk_signer.clone()).unwrap(),
        );

        // Test message
        let message = "Hello, Liquid!";

        // Sign the message
        let signature = wallet.sign_message(message).unwrap();

        // Get the public key
        let pubkey = wallet.pubkey().unwrap();

        // Check the message
        let is_valid = wallet.check_message(message, &pubkey, &signature).unwrap();
        assert!(is_valid, "Message signature should be valid");

        // Check with an incorrect message
        let incorrect_message = "Wrong message";
        let is_invalid = wallet
            .check_message(incorrect_message, &pubkey, &signature)
            .unwrap();
        assert!(
            !is_invalid,
            "Message signature should be invalid for incorrect message"
        );

        // Check with an incorrect public key
        let incorrect_pubkey = "02a1633cafcc01ebfb6d78e39f687a1f0995c62fc95f51ead10a02ee0be551b5dc";
        let is_invalid = wallet
            .check_message(message, incorrect_pubkey, &signature)
            .unwrap();
        assert!(
            !is_invalid,
            "Message signature should be invalid for incorrect public key"
        );

        // Check with an incorrect signature
        let incorrect_signature = zbase32::encode_full_bytes(&[0; 65]);
        let is_invalid = wallet
            .check_message(message, &pubkey, &incorrect_signature)
            .unwrap();
        assert!(
            !is_invalid,
            "Message signature should be invalid for incorrect signature"
        );

        // The temporary directory will be automatically deleted when temp_dir goes out of scope
        Ok(())
    }
}
