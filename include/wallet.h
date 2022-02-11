#ifndef _WALLET_H
#define _WALLET_H

#ifdef __cplusplus
extern "C"{
#endif

    int get_balance(const char*);
    char* get_string();
    char* init_signer(const char*, const char*);

#ifdef __cplusplus
}
#endif
#endif