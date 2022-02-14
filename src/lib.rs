extern crate libc;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::str::FromStr;
use solana_client::{ self, rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signer::{keypair, Signer},
    signature::{Keypair, Signature},
    transaction::Transaction
};
use solana_sdk::pubkey::ParsePubkeyError;
use spl_token;

// const URL: &str = "https://api.devnet.solana.com";
const URL: &str = "http://localhost:8899";
const SCORE_PROGRAM_ID: &str = "5keeyTrvUnZD2ZCxViAhfbFfZwNFuaCKtgKydoTosAdc";
const SCORE_ACCOUNT_ID: &str = "Ew8WXnbQkQnVyaJrNgcPeHH6FQx8b1UTwSH7uNkvAJQf";

const GAME_TOKEN_PROGRAM_ID: &str = "Cf2FH5TEV6T511C4nJDyuyuaVc34vDA66rmmkwquyWeM";
const GAME_OWNER_TOKEN_ACCOUNT: &str = "G6GTsFAnYP1PaNc1g36SF4iuEiosfTZZCWWdnCNxxA8d";
const MINT: &str = "CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s";

#[no_mangle]
pub extern "C" fn init_signer(seed_phrase: *const c_char, passphrase: *const c_char) -> *mut c_char {
    let str_seed_phrase = c_to_str(seed_phrase);
    let str_passphrase = c_to_str(passphrase);
    let signer = keypair::keypair_from_seed_phrase_and_passphrase(str_seed_phrase, str_passphrase)
        .expect("Unable to init signer");
    string_to_c_char(signer.to_base58_string())
}

#[no_mangle]
pub extern "C" fn get_address(signer_str: *const c_char) -> *mut c_char {
    let keypair_str = c_to_str(signer_str);
    let signer = Keypair::from_base58_string(keypair_str);
    string_to_c_char(signer.pubkey().to_string())
}

#[no_mangle]
pub extern "C" fn get_balance(signer_str: *const c_char) -> u64 {
    let keypair_str = c_to_str(signer_str);

    let signer = Keypair::from_base58_string(keypair_str);
    let my_client = RpcClient::new(URL.to_string());

    println!("getting balance for {}", signer.pubkey());
    my_client
        .get_balance(&signer.pubkey())
        .expect("Unable to get balance")
}

#[no_mangle]
pub extern "C" fn get_token_balance(signer_str: *const c_char) -> f64 {
    let keypair_str = c_to_str(signer_str);
    let owner = Keypair::from_base58_string(keypair_str);
    let my_client = RpcClient::new(URL.to_string());

    if let Ok(token_account) = get_token_account(owner.pubkey()) {
        let token_amount = my_client
            .get_token_account_balance(&token_account)
            .expect("Failed to get account info");

        let balance = token_amount.ui_amount.expect("Failed to get token balance");
        println!("Token balance: {}", balance);
        return balance;
    };
    0.0
}

#[no_mangle]
pub extern "C" fn buy_token(signer_str: *const c_char, amount: f64) {
    println!("Trying to buy {} tokens", amount);
    let keypair_str = c_to_str(signer_str);
    let payer = &Keypair::from_base58_string(keypair_str);
    let my_client = RpcClient::new(URL.to_string());
    let program_id = Pubkey::from_str(GAME_TOKEN_PROGRAM_ID).unwrap();
    let game_owner_token_account = Pubkey::from_str(GAME_OWNER_TOKEN_ACCOUNT).unwrap();

    let native_amount = (amount * 1_000_000_000.0) as u64;

    let command: &[u8] = &[1];
    let instr = &native_amount.to_le_bytes() as &[u8];
    let instruction = [command,instr].concat();
    let (pda, _bump_seed) = Pubkey::find_program_address(&[b"flightace"], &program_id);
    let token_program_id= spl_token::ID;
    let gamer_token_account = get_token_account(payer.pubkey())
        .expect("Failed to get token account");

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instruction,
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(game_owner_token_account, false),
            AccountMeta::new(gamer_token_account, false),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(pda, false),
        ],
    );

    let blockhash = my_client.get_latest_blockhash()
        .expect("Unable to get latest blockhash");

    let message = Message::new(
        &[instruction],
        Some(&payer.pubkey()),
    );

    let mut tx = Transaction::new_unsigned(message);
    tx.sign(&[payer], blockhash);
    let signature = my_client
        .send_and_confirm_transaction(&tx)
        .unwrap_or_else(|err|{
            eprintln!("Failed to buy tokens {:?}", err.kind);
            Signature::default()
        });

    println!("Tx sent with hash: {}", signature);
}

#[no_mangle]
pub extern "C" fn save_score(signer_str: *const c_char, score: u64) {
    println!("Start to save score: {}", score);
    let keypair_str = c_to_str(signer_str);
    let payer = &Keypair::from_base58_string(keypair_str);
    let my_client = RpcClient::new(URL.to_string());
    let program_id = Pubkey::from_str(SCORE_PROGRAM_ID).unwrap();
    let account_id = Pubkey::from_str(SCORE_ACCOUNT_ID).unwrap();

    let instr = &score.to_le_bytes() as &[u8];

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instr,
        vec![AccountMeta::new(account_id, false)],
    );

    let blockhash = my_client.get_latest_blockhash()
        .expect("Unable to get latest blockhash");

    let message = Message::new(
        &[instruction],
        Some(&payer.pubkey()),
    );

    let mut tx = Transaction::new_unsigned(message);
    tx.sign(&[payer], blockhash);
    let signature = my_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&tx, CommitmentConfig::confirmed())
        .unwrap_or_else(|err|{
            eprintln!("Failed to save score {:?}", err.kind);
            Signature::default()
        });

    println!("Tx sent with hash: {}", signature);
}

#[no_mangle]
pub extern "C" fn get_score() -> u64 {
    let account_id = Pubkey::from_str(SCORE_ACCOUNT_ID).unwrap();
    let commitment_config = CommitmentConfig::processed();
    let my_client = RpcClient::new(URL.to_string());
    let account = my_client.get_account_with_commitment(
        &account_id,
        commitment_config,
    ).expect("Failed to get account info");

    account.value.unwrap().data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(()).unwrap()
}

fn c_to_str(c_pointer: *const c_char) -> &'static str {
    let c_str = unsafe { CStr::from_ptr(c_pointer) };
    let str = c_str.to_str().unwrap();
    str
}

fn string_to_c_char(str: String) -> *mut c_char {
    CString::new(str.as_bytes()).unwrap().into_raw()
}

fn get_token_account(owner: Pubkey) -> Result<Pubkey, ParsePubkeyError> {
    let my_client = RpcClient::new(URL.to_string());

    let filter = TokenAccountsFilter::Mint(Pubkey::from_str(MINT).unwrap());
    let accounts = my_client
        .get_token_accounts_by_owner(&owner, filter)
        .expect("Failed to get accounts by owner");

    Pubkey::from_str(&accounts[0].pubkey)
}


#[cfg(test)]
mod tests {
    use solana_client::rpc_request::TokenAccountsFilter;
    use super::*;

    #[test]
    #[ignore]
    fn get_balance_test() {
        let my_client = RpcClient::new(URL.to_string());
        let pubkey = Pubkey::from_str("6h21yZr5Ezvv764EzhpdqMAkVxmj99JEGX5Tvrr8AyBD").unwrap();
        println!("getting balance for {}", pubkey.to_string());
        let balance = my_client
            .get_balance(&pubkey)
            .expect("Unable to get balance");
        println!("Balance: {}", balance);
    }

    #[test]
    fn get_token_balance_test() {
        let my_client = RpcClient::new(URL.to_string());
        let owner = Pubkey::from_str("6h21yZr5Ezvv764EzhpdqMAkVxmj99JEGX5Tvrr8AyBD").unwrap();

        let filter = TokenAccountsFilter::Mint(Pubkey::from_str(MINT).unwrap());

        let accounts = my_client.get_token_accounts_by_owner(&owner, filter).unwrap();

        let token_account = Pubkey::from_str(&accounts[0].pubkey).unwrap();

        let token_amount = my_client.get_token_account_balance(&token_account).expect("Failed to get account info");
        println!("Token balance: {}", token_amount.ui_amount.unwrap());
    }

    #[test]
    #[ignore]
    fn get_score_test() {
        println!("Score: {}", get_score());
    }

    #[test]
    #[ignore]
    fn buy_token_test() {

    }
}