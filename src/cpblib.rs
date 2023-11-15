use std::ffi::c_void;

#[repr(C)]
pub struct PB2CNF(*mut c_void);

pub struct EncodingResult {
    clauses: Vec<Vec<i32>>,
    next_free_var_id: i32,
}

impl EncodingResult {
    pub fn clauses(&self) -> &[Vec<i32>] {
        &self.clauses
    }

    pub fn next_free_var_id(&self) -> i32 {
        self.next_free_var_id
    }
}

impl PB2CNF {
    pub fn new() -> Self {
        Self(unsafe { newPB2CNF() })
    }

    pub fn encode_leq(
        &self,
        weights: Vec<i64>,
        literals: Vec<i32>,
        leq: i64,
        first_aux_var: i32,
    ) -> EncodingResult {
        assert_len_eq(&weights, &literals);
        let formula_ptr = unsafe {
            encodeLeq(
                self.0,
                weights.as_ptr(),
                weights.len().try_into().unwrap(),
                literals.as_ptr(),
                literals.len().try_into().unwrap(),
                leq,
                first_aux_var,
            )
        };
        let result = decode_formula_data(formula_ptr);
        unsafe { freePtr(formula_ptr as *mut _) };
        result
    }

    pub fn encode_geq(
        &self,
        weights: Vec<i64>,
        literals: Vec<i32>,
        geq: i64,
        first_aux_var: i32,
    ) -> EncodingResult {
        assert_len_eq(&weights, &literals);
        let formula_ptr = unsafe {
            encodeGeq(
                self.0,
                weights.as_ptr(),
                weights.len().try_into().unwrap(),
                literals.as_ptr(),
                literals.len().try_into().unwrap(),
                geq,
                first_aux_var,
            )
        };
        let result = decode_formula_data(formula_ptr);
        unsafe { freePtr(formula_ptr as *mut _) };
        result
    }

    pub fn encode_both(
        &self,
        weights: Vec<i64>,
        literals: Vec<i32>,
        leq: i64,
        geq: i64,
        first_aux_var: i32,
    ) -> EncodingResult {
        assert_len_eq(&weights, &literals);
        let formula_ptr = unsafe {
            encodeBoth(
                self.0,
                weights.as_ptr(),
                weights.len().try_into().unwrap(),
                literals.as_ptr(),
                literals.len().try_into().unwrap(),
                leq,
                geq,
                first_aux_var,
            )
        };
        let result = decode_formula_data(formula_ptr);
        unsafe { freePtr(formula_ptr as *mut _) };
        result
    }

    pub fn encode_at_most_k(
        &self,
        literals: Vec<i32>,
        k: i64,
        first_aux_var: i32,
    ) -> EncodingResult {
        let formula_ptr = unsafe {
            encodeAtMostK(
                self.0,
                literals.as_ptr(),
                literals.len().try_into().unwrap(),
                k,
                first_aux_var,
            )
        };
        let result = decode_formula_data(formula_ptr);
        unsafe { freePtr(formula_ptr as *mut _) };
        result
    }

    pub fn encode_at_least_k(
        &self,
        literals: Vec<i32>,
        k: i64,
        first_aux_var: i32,
    ) -> EncodingResult {
        let formula_ptr = unsafe {
            encodeAtLeastK(
                self.0,
                literals.as_ptr(),
                literals.len().try_into().unwrap(),
                k,
                first_aux_var,
            )
        };
        let result = decode_formula_data(formula_ptr);
        unsafe { freePtr(formula_ptr as *mut _) };
        result
    }
}

fn decode_formula_data(formula_ptr: *mut i32) -> EncodingResult {
    let data_len = unsafe { std::slice::from_raw_parts(formula_ptr, 1) }[0] as usize;
    let data = unsafe { std::slice::from_raw_parts(formula_ptr, data_len) };
    let next_free_var_id = data[1];
    let mut clauses = Vec::with_capacity(data[0] as usize);
    let mut i = 2;
    while i < data_len {
        let len = data[i] as usize;
        clauses.push(data[i + 1..i + 1 + len].into());
        i += len + 1
    }
    EncodingResult {
        clauses,
        next_free_var_id,
    }
}

fn assert_len_eq(weights: &[i64], literals: &[i32]) {
    if weights.len() != literals.len() {
        panic!(
            "weights len ({}) and literals len ({}) must be equal",
            weights.len(),
            literals.len()
        );
    }
}

impl Default for PB2CNF {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PB2CNF {
    fn drop(&mut self) {
        unsafe { deletePB2CNF(self.0) }
    }
}

extern "C" {
    pub fn newPB2CNF() -> *mut c_void;

    pub fn encodeLeq(
        ptr: *mut c_void,
        weights: *const i64,
        weights_len: i32,
        literals: *const i32,
        literals_len: i32,
        leq: i64,
        firstAuxiliaryVariable: i32,
    ) -> *mut i32;

    pub fn encodeGeq(
        ptr: *mut c_void,
        weights: *const i64,
        weights_len: i32,
        literals: *const i32,
        literals_len: i32,
        geq: i64,
        firstAuxiliaryVariable: i32,
    ) -> *mut i32;

    pub fn encodeBoth(
        ptr: *mut c_void,
        weights: *const i64,
        weights_len: i32,
        literals: *const i32,
        literals_len: i32,
        leq: i64,
        geq: i64,
        firstAuxiliaryVariable: i32,
    ) -> *mut i32;

    pub fn encodeAtMostK(
        ptr: *mut c_void,
        literals: *const i32,
        literals_len: i32,
        k: i64,
        firstAuxiliaryVariable: i32,
    ) -> *mut i32;

    pub fn encodeAtLeastK(
        ptr: *mut c_void,
        literals: *const i32,
        literals_len: i32,
        k: i64,
        firstAuxiliaryVariable: i32,
    ) -> *mut i32;

    pub fn deletePB2CNF(ptr: *mut c_void);

    pub fn freePtr(ptr: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leq_clause() {
        let weights = vec![1, 1];
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_leq(weights, literals, 1, 3);
        assert_encoding_eq(vec![vec![-2, -1]], 3, encoding);
    }

    #[test]
    fn test_geq_clause() {
        let weights = vec![1, 1];
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_geq(weights, literals, 1, 3);
        assert_encoding_eq(vec![vec![1, 2]], 3, encoding);
    }

    #[test]
    fn test_both_xor() {
        let weights = vec![1, 1];
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_both(weights, literals, 1, 1, 3);
        assert_encoding_eq(vec![vec![-2, -1], vec![1, 2]], 3, encoding);
    }

    #[test]
    fn test_at_most_one() {
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_at_most_k(literals, 1, 3);
        assert_encoding_eq(vec![vec![-2, -1]], 3, encoding);
    }

    #[test]
    fn test_at_least_one() {
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_at_least_k(literals, 1, 3);
        assert_encoding_eq(vec![vec![1, 2]], 3, encoding);
    }

    fn assert_encoding_eq(
        expected_formula: Vec<Vec<i32>>,
        expected_next_free_var_id: i32,
        encoding: EncodingResult,
    ) {
        assert_eq!(expected_next_free_var_id, encoding.next_free_var_id());
        let mut clauses = encoding.clauses().to_vec();
        clauses.iter_mut().for_each(|cl| cl.sort_unstable());
        clauses.sort_unstable();
        assert_eq!(expected_formula, clauses)
    }

    #[test]
    #[should_panic]
    fn test_weights_and_literals_len_mismatch() {
        let weights = vec![1];
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        pb2cnf.encode_leq(weights, literals, 1, 3);
    }
}
