use adder::add_two;

mod common;

// when we have a function test into a folder test We don't need to use #[cfg(test)] because the test is only compiled when we run cargo test
#[test]
fn it_adds_two() {
    // we can't se the output of the setup function because it is not returned, but we can see the output in the console when we run 
    common::setup();
    assert_eq!(4, add_two(2));
}