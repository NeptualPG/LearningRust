cargo test
cargo test -- --show-output

// when you have differents CPU:

cargo test -- test-threads=1

cargo test {test_function}

cargo test {any}

// like regex: 
if the name of the function has any it work.