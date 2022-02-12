#include <iostream>
#include "include/wallet.h"

using namespace std;

int main()
{
    const char *seed_phrase = "pitch trust globe fish fever anchor type used aunt enemy crop spatial";
    const char *passphrase = "localhost";

    const char* signer = init_signer(seed_phrase, passphrase);
//    int balance = get_balance(signer);
//    const char* address = get_address(signer);
    save_score(signer, 560025);

//    cout << "Signer: " << signer << endl;
//    cout << "Address: " << address << endl;
//    cout << "Balance: " << balance << endl;

    return 0;
}
