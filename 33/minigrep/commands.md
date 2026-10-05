grep -r "java" src 
cargo run -- test sample.txt
cargo run -- searchstring example-file.txt // 

output : 

C:\Users\juand\Order\programming\Rust\Class\33\minigrep>cargo run -- searchstring example-file.txt
   Compiling minigrep v0.1.0 (C:\Users\juand\Order\programming\Rust\Class\33\minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.02s                                                                         
     Running `target\debug\minigrep.exe searchstring example-file.txt`
[src\main.rs:8:5] &args = [
    "target\\debug\\minigrep.exe",
    "searchstring",
    "example-file.txt",
]
