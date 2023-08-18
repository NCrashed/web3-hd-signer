use std::{str::FromStr, thread, collections::HashMap};


use bip39::Mnemonic;
use config::Config;
use serde_derive::Deserialize;
use web3::types::{U256, H160, H256};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone)]
pub struct WalletAddress {
    pub id : u32,
    pub address : String,
    pub balance : U256,
    pub balance_token : (String, U256),
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub sweeper : String, 
    pub hd_phrase : String, 
    pub eth_token : String,
    pub eth_safe : String,
    pub eth_provider : String,
    pub tron_token : String,
    pub tron_safe : String,
    pub tron_provider : String,
    pub plg_token : String,
    pub plg_safe : String,
    pub plg_provider : String,
    pub bsc_token : String,
    pub bsc_safe : String,
    pub bsc_provider : String,
    pub stl_token : String,
    pub stl_safe : String,
    pub stl_provider : String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Balance {c_from: Option<u32>, c_to: Option<u32>},
    Refill,
    Sweep
}


#[derive(ValueEnum,Debug,Clone)]
pub enum Crypto {
    Eth,
    Tron,
    Polygon,
    BSC,
    Stellar,
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {     
    #[arg(short,long)]
    crypto: Crypto,
    #[arg(default_value = "./config.toml")]
    path: String,
    #[command(subcommand)]
    command: Commands
}
#[allow(non_snake_case)]
#[derive(Debug,Deserialize)]
pub struct RatesRaw{
    pub ETH: f64,
    pub TRX: f64,
    pub MATIC: f64,
    pub BNB: f64,
    pub XLM: f64,
}

#[derive(Debug,Deserialize)]
pub struct Rates{
    pub eth: f64,
    pub trx: f64,
    pub mtc: f64,
    pub bnb: f64,
    pub xlm: f64,
}