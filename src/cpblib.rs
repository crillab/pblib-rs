use std::ffi::c_void;

/// The entry point for the Rust bindings.
///
/// This structure is a wrapper around the underlying C structure that provides a safe interface to the pblib functions.
/// It gives access to the functions dedicated to the encoding of cardinality and Pseudo-Boolean constraints.
///
/// # Encoding cardinality constraints
///
/// Cardinality constraints are encoded thanks to the [`encode_at_least_k`](Self::encode_at_least_k) and [`encode_at_most_k`](Self::encode_at_most_k) functions.
/// They take as input the list of literals (in the DIMACS format) the constraints relies on, the bound, and the minimal variable index that can be used as an auxiliary variables must be given to the function.
/// Auxiliary variables are often used in such encodings since they allow to use less clauses to encode the constraint.
/// The preferred value for this parameter is in most cases the highest variable index in use plus 1.
///
/// The result of this function is an [`EncodingResult`] which gives both the clauses encoding the constraint (as a vector of DIMACS-encoded literals) and the new lowest variable index that is not in use.
///
/// ```
/// use pblib_rs::PB2CNF;
///
/// // we encode x1 + x2 + x3 >= 2
/// let literals = vec![1, 2, 3];
/// let pb2cnf = PB2CNF::new();
/// // the threshold is 2 and the first variable not in use is 4
/// let encoding = pb2cnf.encode_at_least_k(literals, 2, 4);
/// println!("the encoding uses {} variables", encoding.next_free_var_id() - 4);
/// println!("the encoding uses {} clauses", encoding.clauses().len());
/// encoding.clauses().iter().enumerate().for_each(|(i,c)| println!("clause {i} is {:?}", c));
/// ```
///
/// # Encoding Pseudo-Boolean constraints
///
/// The difference between cardinality and Pseudo-Boolean constraints is than weights are applied to literals.
/// Thus, the functions dedicated to this kind of constraints ([`encode_geq`](Self::encode_geq), [`encode_leq`](Self::encode_leq) and [`encode_both`](Self::encode_both)) takes a vector of weights as a supplementary parameter.
/// The vectors of weights and literals must be of same lengths, since variable at index `i` has the weight at index `i`.
///
/// ```
/// use pblib_rs::PB2CNF;
///
/// // we encode 8*x1 + 4*x2 + 2*x3 + 1*x4 >= 6
/// let weights = vec![8, 4, 2, 1];
/// let literals = vec![1, 2, 3, 4];
/// let pb2cnf = PB2CNF::new();
/// // the threshold is 6 and the first variable not in use is 5
/// let encoding = pb2cnf.encode_geq(weights.clone(), literals, 6, 5);
/// println!("the encoding uses {} variables", encoding.next_free_var_id() - 4);
/// println!("the encoding uses {} clauses", encoding.clauses().len());
/// encoding.clauses().iter().enumerate().for_each(|(i,c)| println!("clause {i} is {:?}", c));
/// ```
///
/// # Note about the encodings
///
/// Contrary to pblib, pblib-rs does not allow the user to choose the encoding use for the constraint.
/// The encoding used for the constraints are the default ones of the pblib.
/// In particular, the encodings provided by this library are not intended to match the expected model count of the formula.
#[repr(C)]
pub struct PB2CNF(*mut c_void);

/// The result of an encoding function.
///
/// This structure contains both the clauses generated to encode the constraint and the index of the next free variable id.
pub struct EncodingResult {
    clauses: Vec<Vec<i32>>,
    next_free_var_id: i32,
}

impl EncodingResult {
    /// Returns a reference to the clauses used to encode the constraint.
    #[must_use]
    pub fn clauses(&self) -> &[Vec<i32>] {
        &self.clauses
    }

    /// Returns the next free variable id.
    ///
    /// Encodings use auxiliary variables almost all the time.
    /// Functions that encode constraints ask the first variable id they can use for these auxiliary variables.
    /// In return, they tell the caller which is the lowest id that can be used after this encoding, that is, the highest auxiliary variable index plus 1.
    #[must_use]
    pub fn next_free_var_id(&self) -> i32 {
        self.next_free_var_id
    }
}

impl PB2CNF {
    /// Builds a new structure dedicated to the encoding of constraints.
    #[must_use]
    pub fn new() -> Self {
        Self(unsafe { newPB2CNF() })
    }

    /// Encodes an At-Most-k Pseudo-Boolean constraint.
    ///
    /// An At-Most-k constraint imposes that a weighted sum of literals is less than or equal to an integer value.
    /// The vectors of weights and literals must be of same lengths, since variable at index `i` has the weight at index `i`.
    ///
    /// In addition to this vectors and the threshold, the minimal variable index that can be used as an auxiliary variables must be given to the function.
    /// The preferred value for this parameter is in most cases the highest variable index in use plus 1.
    ///
    /// The result of this function is an [`EncodingResult`] which gives both the clauses encoding the constraint and the new lowest variable index that is not in use.
    ///
    /// # Panics
    ///
    /// In case the weights and literal vectors have not the same length, this function panics.
    #[must_use]
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
        unsafe { freePtr(formula_ptr.cast()) };
        result
    }

    /// Encodes an At-Least-k Pseudo-Boolean constraint.
    ///
    /// An At-Least-k constraint imposes that a weighted sum of literals is greater than or equal to an integer value.
    /// The vectors of weights and literals must be of same lengths, since variable at index `i` has the weight at index `i`.
    ///
    /// In addition to this vectors and the threshold, the minimal variable index that can be used as an auxiliary variables must be given to the function.
    /// The preferred value for this parameter is in most cases the highest variable index in use plus 1.
    ///
    /// The result of this function is an [`EncodingResult`] which gives both the clauses encoding the constraint and the new lowest variable index that is not in use.
    ///
    /// # Panics
    ///
    /// In case the weights and literal vectors have not the same length, this function panics.
    #[must_use]
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
        unsafe { freePtr(formula_ptr.cast()) };
        result
    }

    /// Encodes both an At-Most-k and an At-Least-p Pseudo-Boolean constraints that refers to the same variables and weights.
    ///
    /// See [`encode_leq`](Self::encode_leq) and [`encode_geq`](Self::encode_geq) for more information on At-Most-k and At-Least-p constraints, the `first_aux_var` parameter and the return type.
    /// A call to this function is intended to be more efficient in terms of encoding comparing to a call to [`encode_leq`](Self::encode_leq) and a call to [`encode_geq`](Self::encode_geq).
    ///
    /// # Panics
    ///
    /// In case the weights and literal vectors have not the same length, this function panics.
    #[must_use]
    pub fn encode_both(
        &self,
        weights: Vec<i64>,
        literals: Vec<i32>,
        less_or_eq: i64,
        greater_or_eq: i64,
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
                less_or_eq,
                greater_or_eq,
                first_aux_var,
            )
        };
        let result = decode_formula_data(formula_ptr);
        unsafe { freePtr(formula_ptr.cast()) };
        result
    }

    /// Encodes an At-Most-k cardinality constraint.
    ///
    /// An At-Most-k cardinality constraint imposes that at most k literals in a vector are set to true.
    ///
    /// In addition to this vector and the threshold, the minimal variable index that can be used as an auxiliary variables must be given to the function.
    /// The preferred value for this parameter is in most cases the highest variable index in use plus 1.
    ///
    /// The result of this function is an [`EncodingResult`] which gives both the clauses encoding the constraint and the new lowest variable index that is not in use.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
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
        unsafe { freePtr(formula_ptr.cast()) };
        result
    }

    /// Encodes an At-Least-k cardinality constraint.
    ///
    /// An At-Least-k cardinality constraint imposes that at least k literals in a vector are set to true.
    ///
    /// In addition to this vector and the threshold, the minimal variable index that can be used as an auxiliary variables must be given to the function.
    /// The preferred value for this parameter is in most cases the highest variable index in use plus 1.
    ///
    /// The result of this function is an [`EncodingResult`] which gives both the clauses encoding the constraint and the new lowest variable index that is not in use.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
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
        unsafe { freePtr(formula_ptr.cast()) };
        result
    }
}

fn decode_formula_data(formula_ptr: *mut i32) -> EncodingResult {
    let data_len =
        usize::try_from(unsafe { std::slice::from_raw_parts(formula_ptr, 1) }[0]).unwrap();
    let data = unsafe { std::slice::from_raw_parts(formula_ptr, data_len) };
    let next_free_var_id = data[1];
    let mut clauses = Vec::with_capacity(usize::try_from(data[0]).unwrap());
    let mut i = 2;
    while i < data_len {
        let len = usize::try_from(data[i]).unwrap();
        clauses.push(data[i + 1..i + 1 + len].into());
        i += len + 1;
    }
    EncodingResult {
        clauses,
        next_free_var_id,
    }
}

fn assert_len_eq(weights: &[i64], literals: &[i32]) {
    assert_eq!(
        weights.len(),
        literals.len(),
        "weights len ({}) and literals len ({}) must be equal",
        weights.len(),
        literals.len()
    );
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
    use splr::{Certificate, Config, SolveIF, Solver, SolverError};

    #[test]
    fn test_leq_clause() {
        let weights = vec![1, 1];
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_leq(weights, literals, 1, 3);
        assert_encoding_eq(&[vec![-2, -1]], 3, &encoding);
    }

    #[test]
    fn test_geq_clause() {
        let weights = vec![1, 1];
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_geq(weights, literals, 1, 3);
        assert_encoding_eq(&[vec![1, 2]], 3, &encoding);
    }

    #[test]
    fn test_both_xor_clauses() {
        let weights = vec![1, 1];
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_both(weights, literals, 1, 1, 3);
        assert_encoding_eq(&[vec![-2, -1], vec![1, 2]], 3, &encoding);
    }

    #[test]
    fn test_at_most_one_clause() {
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_at_most_k(literals, 1, 3);
        assert_encoding_eq(&[vec![-2, -1]], 3, &encoding);
    }

    #[test]
    fn test_at_least_one_clause() {
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_at_least_k(literals, 1, 3);
        assert_encoding_eq(&[vec![1, 2]], 3, &encoding);
    }

    fn assert_encoding_eq(
        expected_formula: &[Vec<i32>],
        expected_next_free_var_id: i32,
        encoding: &EncodingResult,
    ) {
        assert_eq!(expected_next_free_var_id, encoding.next_free_var_id());
        let mut clauses = encoding.clauses().to_vec();
        clauses.iter_mut().for_each(|cl| cl.sort_unstable());
        clauses.sort_unstable();
        assert_eq!(expected_formula, clauses);
    }

    #[test]
    #[should_panic(expected = "weights len (1) and literals len (2) must be equal")]
    fn test_weights_and_literals_len_mismatch() {
        let weights = vec![1];
        let literals = vec![1, 2];
        let pb2cnf = PB2CNF::new();
        let _ = pb2cnf.encode_leq(weights, literals, 1, 3);
    }

    fn check_models(
        encoding: &EncodingResult,
        init_n_vars: usize,
        property: &dyn Fn(&[i32]) -> bool,
        n_models: usize,
    ) {
        let mut solver = Solver::try_from((Config::default(), encoding.clauses())).unwrap();
        let mut models = solver
            .iter()
            .map(|m| {
                let mut c = m.clone();
                c.truncate(init_n_vars);
                c
            })
            .collect::<Vec<_>>();
        models.sort_unstable();
        models.dedup();
        assert_eq!(n_models, models.len());
        for m in &models {
            assert!(property(m));
        }
    }

    fn check_unsat(encoding: &EncodingResult) {
        if encoding.clauses().iter().any(Vec::is_empty) {
            return;
        }
        let mut solver = match Solver::try_from((Config::default(), encoding.clauses())) {
            Ok(s) => s,
            Err(r) => match r {
                Ok(Certificate::SAT(_)) => panic!(),
                Ok(Certificate::UNSAT)
                | Err(SolverError::EmptyClause | SolverError::Inconsistent) => return,
                _ => panic!("internal error"),
            },
        };
        assert_eq!(Ok(Certificate::UNSAT), solver.solve());
    }

    fn model_cost(weights: &[i64], model: &[i32]) -> i64 {
        model
            .iter()
            .map(|l| {
                if *l > 0 {
                    weights[usize::try_from(*l - 1).unwrap()]
                } else {
                    0
                }
            })
            .sum()
    }

    #[test]
    fn test_leq() {
        let weights = vec![8, 4, 2, 1];
        let literals = vec![1, 2, 3, 4];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_leq(weights.clone(), literals, 6, 5);
        check_models(&encoding, 4, &|m| model_cost(&weights, m) <= 6, 7);
    }

    #[test]
    fn test_geq() {
        let weights = vec![8, 4, 2, 1];
        let literals = vec![1, 2, 3, 4];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_geq(weights.clone(), literals, 6, 5);
        check_models(&encoding, 4, &|m| model_cost(&weights, m) >= 6, 10);
    }

    #[test]
    fn test_both() {
        let weights = vec![8, 4, 2, 1];
        let literals = vec![1, 2, 3, 4];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_both(weights.clone(), literals, 7, 5, 5);
        check_models(
            &encoding,
            4,
            &|m| model_cost(&weights, m) >= 5 && model_cost(&weights, m) <= 7,
            3,
        );
    }

    #[test]
    fn test_both_unsat() {
        let weights = vec![8, 4, 2, 1];
        let literals = vec![1, 2, 3, 4];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_both(weights.clone(), literals, 5, 7, 5);
        check_unsat(&encoding);
    }

    #[test]
    fn test_at_least() {
        let literals = vec![1, 2, 3];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_at_least_k(literals, 2, 4);
        let weights = vec![1; 3];
        check_models(&encoding, 3, &|m| model_cost(&weights, m) >= 2, 4);
    }

    #[test]
    fn test_at_most() {
        let literals = vec![1, 2, 3];
        let pb2cnf = PB2CNF::new();
        let encoding = pb2cnf.encode_at_most_k(literals, 2, 4);
        let weights = vec![1; 3];
        check_models(&encoding, 3, &|m| model_cost(&weights, m) <= 2, 7);
    }
}
