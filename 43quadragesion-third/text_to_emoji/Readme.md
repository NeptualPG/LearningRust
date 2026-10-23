# Text to Emoji 

A Rust library to convert words in a sentence into emojis! This library prodvides a fun and simple way to add emoji representations to text.

## Installation

Add this to yout `Cargo.toml`:

```toml
[dependencies]
text_to_emoji = "0.1.0"

```

### Example 

```rust 
use text_to_emoji::text_to_emoji;

assert_eq!(text_to_emoji("I love coffee and pizza"), "I ❤️ ☕ and 🍕");
```

## License

This project is licensed under the MIT License.