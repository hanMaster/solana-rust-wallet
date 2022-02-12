extern crate libc;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::str::FromStr;
use solana_sdk::signer::{keypair, Signer};
use solana_client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::transaction::Transaction;

const URL: &str = "https://api.devnet.solana.com";
const PROGRAM_ID: &str = "8DYJ4XBH9Zrg9hEdE7wZeQKH4bGEGhPgNp5WpgQN5x82";
const ACCOUNT_ID: &str = "9ZJd6BhBrUetpM2r9MbxQzoiWN7dTzU3fauEZftGyH9v";

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
    let balance = my_client
        .get_balance(&signer.pubkey())
        .expect("Unable to get balance");

    return balance;
}

#[no_mangle]
pub extern "C" fn save_score(signer_str: *const c_char, score: u64) {
    println!("Score to save: {}", score);

    let keypair_str = c_to_str(signer_str);

    let payer = &Keypair::from_base58_string(keypair_str);
    let my_client = RpcClient::new(URL.to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let account_id = Pubkey::from_str(ACCOUNT_ID).unwrap();

    let instr = &score.to_le_bytes() as &[u8];
    println!("instr: {:?}", instr);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instr,
        vec![AccountMeta::new(account_id, false)],
    );

    let blockhash = my_client.get_latest_blockhash().expect("Unable to get latest blockhash");
    let message = Message::new(
        &[instruction],
        Some(&payer.pubkey()),
    );

    let mut tx = Transaction::new_unsigned(message);
    tx.sign(&[payer], blockhash);
    let signature = my_client
        .send_and_confirm_transaction(&tx)
        .unwrap_or_else(|err|{
            eprintln!("Failed to save score {:?}", err.kind);
            Signature::default()
        });

    println!("Transaction sent with hash: {}", signature);
}

fn c_to_str(c_pointer: *const c_char) -> &'static str {
    let c_str = unsafe { CStr::from_ptr(c_pointer) };
    let str = c_str.to_str().unwrap();
    str
}

fn string_to_c_char(str: String) -> *mut c_char {
    CString::new(str.as_bytes()).unwrap().into_raw()
}


#[cfg(test)]
mod tests {
    use solana_sdk::commitment_config::CommitmentConfig;
    use super::*;

    // #[test]
    // fn get_signer() {
    //     let seed_phrase = string_to_c_char(String::from("pitch trust globe fish fever anchor type used aunt enemy crop spatial"));
    //     let passphrase = string_to_c_char(String::from("localhost"));
    //     let signer = init_signer(seed_phrase, passphrase);
    //     // assert_eq!(signer.pubkey().to_string(), "6h21yZr5Ezvv764EzhpdqMAkVxmj99JEGX5Tvrr8AyBD");
    //     println!("Keypair: {:?}", &signer.unwrap().to_base58_string());
    // }

    // #[test]
    // fn get_balance_test() {
    //     let signer = init_signer(
    //         CString::new("pitch trust globe fish fever anchor type used aunt enemy crop spatial").unwrap(),
    //         CString::new("localhost").unwrap());
    //     let balance = get_balance(&signer);
    //     println!("Balance: {}", balance);
    // }

    #[test]
    fn get_score() {
        let account_id = Pubkey::from_str(ACCOUNT_ID).unwrap();
        let commitment_config = CommitmentConfig::processed();
        let my_client = RpcClient::new(URL.to_string());
        let account = my_client.get_account_with_commitment(
            &account_id,
            commitment_config,
        ).expect("Failed to get account info");

        let data = account.value.unwrap().data;
        println!("Account data: {:?}", &data);

        let score = data
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(()).unwrap();
        println!("Data: {}", score);
    }
}