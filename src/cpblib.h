#ifndef __CPBLIB_H
#define __CPBLIB_H

#include <stdint.h>
#include <vector>

#include "pblib/pb2cnf.h"

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct CPB2CNF CPB2CNF;

    CPB2CNF* newPB2CNF();

    int32_t* encodeLeq(
        CPB2CNF* cpb2cnf,
        int64_t* weights,
        int32_t weights_len,
        int32_t* literals,
        int32_t literals_len,
        int64_t leq,
        int32_t firstAuxiliaryVariable
    );

    int32_t* encodeGeq(
        CPB2CNF* cpb2cnf,
        int64_t* weights,
        int32_t weights_len,
        int32_t* literals,
        int32_t literals_len,
        int64_t geq,
        int32_t firstAuxiliaryVariable
    );

    int32_t* encodeBoth(
        CPB2CNF* cpb2cnf,
        int64_t* weights,
        int32_t weights_len,
        int32_t* literals,
        int32_t literals_len,
        int64_t leq,
        int64_t geq,
        int32_t firstAuxiliaryVariable
    );

    int32_t* encodeAtMostK(
        CPB2CNF* cpb2cnf,
        int32_t* literals,
        int32_t literals_len,
        int64_t k,
        int32_t firstAuxiliaryVariable
    );

    int32_t* encodeAtLeastK(
        CPB2CNF* cpb2cnf,
        int32_t* literals,
        int32_t literals_len,
        int64_t k,
        int32_t firstAuxiliaryVariable
    );

    void deletePB2CNF(CPB2CNF* cpb2cnf);

    void freePtr(int32_t* ptr);

#ifdef __cplusplus
}
#endif
#endif
