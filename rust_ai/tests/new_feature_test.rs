use rust_ai::new_feature::new_rust_function;

#[test]
fn test_new_rust_function() {
    assert_eq!(new_rust_function(), "Hello from Rust feature!");
}
