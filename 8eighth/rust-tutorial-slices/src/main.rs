//Exercise: get the first word of a string();
fn main(){
    let s = String::from("anyword and more");
    get_word(s);
}

fn get_word(s: String) {
    let len = s.len();
    let mut cha: &str = "";
    let mut i = 1;
    
    while i <= len{
        cha = &s[i-1..i];
        if cha == " "{
            let shot: &str = &s[..i];
            println!("{}",shot);
            i = len;
        }else{
            i += 1;
        }
    }
}

