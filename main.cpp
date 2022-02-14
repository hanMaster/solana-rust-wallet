#include <iostream>
#include "include/wallet.h"

using namespace std;

int main()
{
    const char *seed_phrase = "pitch trust globe fish fever anchor type used aunt enemy crop spatial";
    const char *passphrase = "localhost";

    const char* signer = init_signer(seed_phrase, passphrase);
    long int balance = get_balance(signer);
    double token_balance = get_token_balance(signer);
    const char* address = get_address(signer);
    buy_token(signer, 2);
//    save_score(signer, 42);

    cout << "Address: " << address << endl;
    cout << "Balance: " << balance << endl;
    cout << "Token balance: " << token_balance << endl;

    return 0;
}
