juand@Neptual MINGW64 ~/Order/programming/Rust/Class/37thirty-seventh/envlive (main)
$ CLI_ARG=TEST cargo run -q
couldn't interpret BB: environment variable not found
CLI_ARG: "TEST"


(base) PS C:\Users\juand\Order\programming\Rust\Class\37thirty-seventh\envlive> $env:CLI_ARG="TEST"; cargo run -q
couldn't interpret BB: environment variable not found
CLI_ARG: "TEST"


cargo add dotenv