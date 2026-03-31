use learn_rust2::rand_even;

#[test]
fn test_rand_even() {
    for _ in 0..100 {
        let result = rand_even();
        assert_eq!(result % 2, 0);
    }
}
