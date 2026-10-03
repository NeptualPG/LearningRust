use adder::add_two;

// when we have a function test into a folder test We don't need to use #[cfg(test)] because the test is only compiled when we run cargo test
#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}