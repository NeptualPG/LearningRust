use std::collection::HashMap;

pub fn get_emoji_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("smile", "😊");
    map.insert("love", "❤️");
    map.insert("thumbs_up", "👍");
    map.insert("star", "⭐");
    map.insert("coffee", "☕");
    map.insert("pizza", "🍕");
    map.insert("sun", "☀️");
}