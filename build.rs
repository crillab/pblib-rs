use cc::Build;

const PBLIB_DIR: &str = "pblib";
const PBLIB_FILES: [&str; 30] = [
    "pblib/preencoder.cpp",
    "pblib/IncSimplePBConstraint.cpp",
    "pblib/PBConfig.cpp",
    "pblib/PBParser.cpp",
    "pblib/SimplePBConstraint.cpp",
    "pblib/encoder/BDD_Seq_Amo.cpp",
    "pblib/encoder/bimander_amo_encoding.cpp",
    "pblib/encoder/k-product.cpp",
    "pblib/encoder/BinaryMerge.cpp",
    "pblib/encoder/naive_amo_encoder.cpp",
    "pblib/encoder/sorting_merging.cpp",
    "pblib/encoder/SortingNetworks.cpp",
    "pblib/encoder/SWC.cpp",
    "pblib/encoder/Encoder.cpp",
    "pblib/encoder/cardencoding.cpp",
    "pblib/encoder/amo.cpp",
    "pblib/encoder/binary_amo.cpp",
    "pblib/encoder/commander_encoding.cpp",
    "pblib/encoder/adderencoding.cpp",
    "pblib/encoder/bdd.cpp",
    "pblib/pbconstraint.cpp",
    "pblib/IncrementalData.cpp",
    "pblib/PBFuzzer.cpp",
    "pblib/clausedatabase.cpp",
    "pblib/formula.cpp",
    "pblib/pb2cnf.cpp",
    "pblib/incpbconstraint.cpp",
    "pblib/VectorClauseDatabase.cpp",
    "pblib/auxvarmanager.cpp",
    "pblib/helper.cpp",
];

const MINISAT_DIR: &str = "pblib/cli/minisat";
const MINISAT_FILES: [&str; 5] = [
    "minisat/utils/Options.cc",
    "minisat/utils/System.cc",
    "minisat/core/Solver.cc",
    "minisat/core/SolverTypes.cc",
    "minisat/simp/SimpSolver.cc",
];

fn build_dep<T, U>(flags: &[&str], includes: &[T], files: &[U], output: &str)
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let mut build = Build::new();
    build
        .cpp(true)
        .warnings(true)
        .opt_level(3)
        .define("NDEBUG", "1");
    flags.iter().for_each(|f| {
        build.flag_if_supported(f);
    });
    build.includes(includes.iter().map(|i| i.as_ref()));
    build.files(files.iter().map(|f| f.as_ref()));
    println!("cargo:rerun-if-changed=build.rs");
    build.compile(output);
}

fn main() {
    build_dep(
        &["-std=c++11", "-Wno-class-memaccess"],
        &[MINISAT_DIR],
        MINISAT_FILES
            .iter()
            .map(|f| format!("{}/{}", MINISAT_DIR, f))
            .collect::<Vec<String>>()
            .as_slice(),
        "libminisat.a",
    );

    build_dep(
        &[
            "-std=c++11",
            "-Wno-sign-compare",
            "-Wno-unused-variable",
            "-Wno-unused-but-set-variable",
            "-Wno-unused-parameter",
            "-Wno-unused-function",
        ],
        &[PBLIB_DIR, MINISAT_DIR],
        PBLIB_FILES
            .iter()
            .map(|f| format!("{}/{}", PBLIB_DIR, f))
            .collect::<Vec<String>>()
            .as_slice(),
        "libpb.a",
    );

    build_dep(
        &["-std=c++11", "-Wno-sign-compare"],
        &[PBLIB_DIR],
        &["src/cpblib.cc"],
        "libcpblib.a",
    )
}
