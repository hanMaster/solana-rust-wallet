extern crate libc;

use libc::c_char;
use std::ffi::{CStr, CString};
use solana_sdk::signer::{keypair, Signer};
use solana_client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

const URL: &str = "https://api.devnet.solana.com";

#[no_mangle]
pub extern "C" fn init_signer(seed_phrase: *const c_char, passphrase: *const c_char) -> *mut c_char {
    let str_seed_phrase = c_to_str(seed_phrase);
    let str_passphrase = c_to_str(passphrase);
    let signer = keypair::keypair_from_seed_phrase_and_passphrase(str_seed_phrase, str_passphrase)
        .expect("Unable to init signer");
    string_to_c_char(signer.to_base58_string())
}

#[no_mangle]
pub extern "C" fn get_string() -> *mut c_char {
    string_to_c_char(String::from("Test string mother fucker"))
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
}

//  [217, 103, 211, 64, 254, 238, 114, 106, 77, 113, 212, 160, 59, 28, 89, 101, 132, 93, 252, 122, 81, 135, 50, 189, 157, 147, 77, 172, 183, 220, 22, 2]
//  [217, 103, 211, 64, 254, 238, 114, 106, 77, 113, 212, 160, 59, 28, 89, 101, 132, 93, 252, 122, 81, 135, 50, 189, 157, 147, 77, 172, 183, 220, 22, 2, 84, 138, 209, 19, 255, 9, 228, 2, 220, 118, 199, 84, 131, 37, 183, 136, 141, 152, 92, 17, 211, 57, 217, 211, 49, 201, 4, 144, 99, 54, 141, 104]