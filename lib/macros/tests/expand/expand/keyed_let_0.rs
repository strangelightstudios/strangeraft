fn main() {
    macros::expand!(
        KEYED,
        (K, T, V) => {let K: T = V;},
    );
}
