./target/debug/stderr-1.exe

just standard:

./target/debug/stderr-1.exe > output.log

just error message:

./target/debug/stderr-1.exe 2> error.log

all the logs: 

 ./target/debug/stderr-1.exe &> logs.log
