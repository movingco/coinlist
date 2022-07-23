//! Parses coin lists according to the coin list standard.
#![deny(missing_docs)]

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use struct_tag::StructTagData;
use url::Url;

/// ID of a chain supporting coins.
#[derive(Clone, Copy, Debug, JsonSchema, Serialize, Deserialize)]
#[repr(u32)]
pub enum ChainID {
    /// Aptos mainnet.
    AptosMainnet = 200,
    /// Aptos devnet.
    AptosDevnet = 201,
    /// Sui mainnet.
    SuiMainnet = 300,
    /// Sui devnet.
    SuiDevnet = 301,
}

/// Extra information about a coin.
#[derive(Clone, Debug, Default, JsonSchema, Serialize, Deserialize)]
pub struct CoinExtensions {
    /// Website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// The bridge contract.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bridgeContract")]
    pub bridge_contract: Option<String>,
    /// The asset contract.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetContract")]
    pub asset_contract: Option<String>,
    /// Explorer link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explorer: Option<String>,
    /// Twitter link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    /// GitHub link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    /// Medium link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// Telegram announcement link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgann: Option<String>,
    /// Telegram group link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tggroup: Option<String>,
    /// Discord link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    /// Serum V3 USDT market.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serumV3Usdt")]
    pub serum_v3_usdt: Option<String>,
    /// Serum V3 USDC market.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serumV3Usdc")]
    pub serum_v3_usdc: Option<String>,
    /// Coingecko API ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coingeckoId")]
    pub coingecko_id: Option<String>,
    /// URL of the image representing this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    /// Brief description of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Underlying coins backing this coin.
    /// For example: if this is a Uniswap LP coin, this would be the two coins.
    #[serde(skip_serializing_if = "Option::is_none", rename = "underlyingCoins")]
    pub underlying_coins: Option<Vec<StructTagData>>,

    /// The protocol or app that this coin originates from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Unknown extensions.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Coin information.
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct CoinInfo {
    /// Name of the coin.
    pub name: String,
    /// Symbol of the coin.
    pub symbol: String,
    /// Logo of the coin. Highly recommended.
    /// If the provided logo is invalid, this value is discarded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logoURI")]
    pub logo_uri: Option<Url>,
    /// Number of decimals of the coin.
    pub decimals: u8,
    /// Coin struct tag.
    pub address: StructTagData,
    /// Chain ID of the coin.
    #[serde(rename = "chainId")]
    pub chain_id: u32,
    /// Tags of the coin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Coin extensions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<CoinExtensions>,
}

impl Eq for CoinInfo {}

impl PartialEq for CoinInfo {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.chain_id == other.chain_id
    }
}

impl PartialOrd for CoinInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CoinInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.chain_id == other.chain_id {
            self.address.to_string().cmp(&other.address.to_string())
        } else {
            self.chain_id.cmp(&other.chain_id)
        }
    }
}

impl CoinInfo {
    /// Removes the tags and extensions from the [CoinInfo].
    /// This is useful for making smaller coin lists.
    pub fn simplify(&mut self) {
        self.tags = None;
        self.extensions = None;
    }
}

/// Details about what a tag is.
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct TagDetails {
    /// Name of the tag.
    pub name: String,
    /// Description of what the tag is.
    pub description: String,
}

/// Semver version of the coin list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, Serialize, Deserialize)]
pub struct Version {
    /// Major version.
    pub major: u32,
    /// Minor version.
    pub minor: u32,
    /// Patch version.
    pub patch: u32,
}

/// A list of coins.
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct CoinList {
    /// Name of the coin list.
    pub name: String,
    /// Logo URI of the coin list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<Url>,
    /// All tags that may be referenced in the coin list.
    pub tags: BTreeMap<String, TagDetails>,
    /// When the coin list was last updated.
    pub timestamp: DateTime<Utc>,
    /// The coins in the coin list.
    /// This is named `tokens` to keep compatibility with the Uniswap spec.
    pub tokens: Vec<CoinInfo>,
    /// The semver version.
    pub version: Version,
}

impl CoinList {
    /// Creates a new [CoinList] with the current timestamp.
    pub fn new(name: &str) -> CoinList {
        CoinList {
            name: name.to_string(),
            logo_uri: None,
            tags: BTreeMap::new(),
            timestamp: Utc::now(),
            tokens: vec![],
            version: Version::default(),
        }
    }

    /// Filters the coins in the coin list by the given chain ID.
    pub fn filter_chain(&self, chain_id: u32) -> CoinList {
        CoinList {
            tokens: self
                .tokens
                .clone()
                .into_iter()
                .filter(|t| t.chain_id == chain_id)
                .collect(),
            ..self.clone()
        }
    }

    /// Strips extraneous metadata from the coin list.
    pub fn simplify(&mut self) {
        self.tags = BTreeMap::new();
        self.tokens = self
            .tokens
            .iter()
            .map(|coin| {
                let mut coin = coin.clone();
                coin.simplify();
                coin
            })
            .collect();
    }
}
