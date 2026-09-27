set "RUST_BACKTRACE=full" && cargo run
set "RUST_BACKTRACE=1" && cargo run

wihtout panic: 

set "RUST_BACKTRACE=0"
cargo run -q