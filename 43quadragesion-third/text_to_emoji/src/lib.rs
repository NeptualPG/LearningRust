mod emoji_mappings;

// Converts words in a given sentence to their corresponding emojis.
// arguments:

/// # Arguments
/// * `sentence` - A string slice that holds the sentence to be converted.
/// # Returns
/// * A `String` with words replaced by their corresponding emojis.
/// 
/// # Example
/// ```
/// use text_to_emoji::text_to_emoji;
/// assert_eq!(text_to_emoji("I love coffee and pizza"), "I ❤️ ☕ and 🍕");
/// ```
pub fn text_to_emoji(sentence: &str) -> String {
    let emoji_map = emoji_mappings::get_emoji_map();
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut result = String::new();

    for word in words {
        if let Some(&emoji) = emoji_map.get(word) {
            result.push_str(emoji);
        } else {
            result.push_str(word);
        }
        result.push(' '); // Add a space after each word/emoji
    }

    result.trim_end().to_string() // Remove the trailing space and return
}