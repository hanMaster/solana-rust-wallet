#ifndef _WALLET_H
#define _WALLET_H

#ifdef __cplusplus
extern "C"{
#endif

    int get_balance(const char*);
    char* get_address(const char*);
    char* init_signer(const char*, const char*);
    void save_score(const char*, int);
    int get_score();

#ifdef __cplusplus
}
#endif
#endif