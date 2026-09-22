fn main() {
    let config_max: Option<u32> = Some(100);
    match config_max {
        Some(max: i32) => println!("The maximum is configured to be {}", max),
        _=>(),
    }
}
