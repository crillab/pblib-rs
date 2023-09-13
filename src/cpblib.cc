#include "cpblib.h"

extern "C"
{
    int32_t* encode_formula(std::vector< std::vector<int32_t> > formula_vec, int32_t firstAuxiliaryVariable);

    CPB2CNF* newPB2CNF()
    {
        PB2CNF* pb2cnf = new PB2CNF();
        return reinterpret_cast<CPB2CNF*>(pb2cnf);
    }

    int32_t* encodeLeq(
        CPB2CNF* cpb2cnf,
        int64_t* weights,
        int32_t weights_len,
        int32_t* literals,
        int32_t literals_len,
        int64_t leq,
        int32_t firstAuxiliaryVariable
    ) {
        PB2CNF *pb2cnf = reinterpret_cast<PB2CNF *>(cpb2cnf);
        std::vector<int64_t> weights_vec(weights, weights + weights_len);
        std::vector<int32_t> literals_vec(literals, literals + literals_len);
        std::vector< std::vector<int32_t> > formula_vec;
        firstAuxiliaryVariable = pb2cnf->encodeLeq(weights_vec, literals_vec, leq, formula_vec, firstAuxiliaryVariable) + 1;
        return encode_formula(formula_vec, firstAuxiliaryVariable);
    }

    int32_t* encodeGeq(
        CPB2CNF* cpb2cnf,
        int64_t* weights,
        int32_t weights_len,
        int32_t* literals,
        int32_t literals_len,
        int64_t geq,
        int32_t firstAuxiliaryVariable
    ) {
        PB2CNF *pb2cnf = reinterpret_cast<PB2CNF *>(cpb2cnf);
        std::vector<int64_t> weights_vec(weights, weights + weights_len);
        std::vector<int32_t> literals_vec(literals, literals + literals_len);
        std::vector< std::vector<int32_t> > formula_vec;
        firstAuxiliaryVariable = pb2cnf->encodeGeq(weights_vec, literals_vec, geq, formula_vec, firstAuxiliaryVariable) + 1;
        return encode_formula(formula_vec, firstAuxiliaryVariable);
    }

    int32_t* encodeBoth(
        CPB2CNF* cpb2cnf,
        int64_t* weights,
        int32_t weights_len,
        int32_t* literals,
        int32_t literals_len,
        int64_t leq,
        int64_t geq,
        int32_t firstAuxiliaryVariable
    ) {
        PB2CNF *pb2cnf = reinterpret_cast<PB2CNF *>(cpb2cnf);
        std::vector<int64_t> weights_vec(weights, weights + weights_len);
        std::vector<int32_t> literals_vec(literals, literals + literals_len);
        std::vector< std::vector<int32_t> > formula_vec;
        firstAuxiliaryVariable = pb2cnf->encodeBoth(weights_vec, literals_vec, leq, geq, formula_vec, firstAuxiliaryVariable) + 1;
        return encode_formula(formula_vec, firstAuxiliaryVariable);
    }

    int32_t* encodeAtMostK(
        CPB2CNF* cpb2cnf,
        int32_t* literals,
        int32_t literals_len,
        int64_t k,
        int32_t firstAuxiliaryVariable
    ) {
        PB2CNF *pb2cnf = reinterpret_cast<PB2CNF *>(cpb2cnf);
        std::vector<int32_t> literals_vec(literals, literals + literals_len);
        std::vector< std::vector<int32_t> > formula_vec;
        firstAuxiliaryVariable = pb2cnf->encodeAtMostK(literals_vec, k, formula_vec, firstAuxiliaryVariable) + 1;
        return encode_formula(formula_vec, firstAuxiliaryVariable);
    }

    int32_t* encodeAtLeastK(
        CPB2CNF* cpb2cnf,
        int32_t* literals,
        int32_t literals_len,
        int64_t k,
        int32_t firstAuxiliaryVariable
    ) {
        PB2CNF *pb2cnf = reinterpret_cast<PB2CNF *>(cpb2cnf);
        std::vector<int32_t> literals_vec(literals, literals + literals_len);
        std::vector< std::vector<int32_t> > formula_vec;
        firstAuxiliaryVariable = pb2cnf->encodeAtLeastK(literals_vec, k, formula_vec, firstAuxiliaryVariable) + 1;
        return encode_formula(formula_vec, firstAuxiliaryVariable);
    }

    int32_t* encode_formula(std::vector< std::vector<int32_t> > formula_vec, int32_t firstAuxiliaryVariable)
    {
        int32_t formula_len = 2 + formula_vec.size();
        for(std::vector< std::vector<int32_t> >::iterator it = formula_vec.begin(); it < formula_vec.end(); it++) {
            formula_len += it->size();
        }
        int32_t* formula = (int32_t*) malloc(formula_len * sizeof(int32_t));
        int *pf = formula;
        *pf++ = formula_len;
        *pf++ = firstAuxiliaryVariable;
        for(std::vector< std::vector<int32_t> >::iterator it = formula_vec.begin(); it < formula_vec.end(); it++) {
            *pf++ = it->size();
            copy(it->begin(), it->end(), pf);
            pf += it->size();
        }
        return formula;
    }

    void deletePB2CNF(CPB2CNF* cpb2cnf)
    {
        delete reinterpret_cast<PB2CNF*>(cpb2cnf);
    }

    void freePtr(int32_t* ptr)
    {
        free(ptr);
    }
}
