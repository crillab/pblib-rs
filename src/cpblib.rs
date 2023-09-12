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
        threshold: i64,
        first_aux_var: i32,
    ) -> EncodingResult {
        let formula_ptr = unsafe {
            encodeLeq(
                self.0,
                weights.as_ptr(),
                weights.len().try_into().unwrap(),
                literals.as_ptr(),
                literals.len().try_into().unwrap(),
                threshold,
                first_aux_var,
            )
        };
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
        unsafe { freePtr(formula_ptr as *mut _) };
        EncodingResult {
            clauses,
            next_free_var_id,
        }
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
        assert_eq!(3, encoding.next_free_var_id());
        let mut clauses = encoding.clauses().to_vec();
        clauses.iter_mut().for_each(|cl| cl.sort_unstable());
        clauses.sort_unstable();
        assert_eq!(vec![vec![-2, -1]], clauses)
    }
}
