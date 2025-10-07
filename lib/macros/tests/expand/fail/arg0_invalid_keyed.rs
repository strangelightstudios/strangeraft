fn main() {
    macros::expand!(
        !FOO,
        (K, T, V) => {K; T; V;},
    );

    macros::expand!(
        FOO,
        (K, T, V) => {K; T; V;},
    );
}
