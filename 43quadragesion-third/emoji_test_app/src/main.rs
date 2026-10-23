use text_to_emoji::convert_to_emojis;

fn main() {
    let sentence = "I love  coffee and pizza on a sun day";
    let emoji_sentence = convert_to_emojis(sentence);
    println!("{}", emoji_sentence);    
}
