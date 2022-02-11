#include <iostream>
#include "include/wallet.h"

using namespace std;

int main()
{
    const char *seed_phrase = "pitch trust globe fish fever anchor type used aunt enemy crop spatial";
    const char *passphrase = "localhost";

    const char* signer = init_signer(seed_phrase, passphrase);
    int balance = get_balance(signer);

//    const char* signer = "test";
//    int balance = 5;

    cout << "Signer: " << signer << endl;
    cout << "Balance: " << balance << endl;

    return 0;
}
