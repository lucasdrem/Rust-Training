pub fn run (s: &str) -> String {
    let mut words_not_repeated = String::new();

    for word in s.split_whitespace() {
        if !words_not_repeated.contains(word) {
            let spaced_word = format!("{} ", word);
            words_not_repeated.push_str(&spaced_word)
        }
    }

    return (words_not_repeated.trim()).to_string();

    //or 
    // let mut v: Vec<&str> = vec![];

    // for s in s.split_whitespace() { if !v.contains(&s) { v.push(s); } }

    // v.join(" ")

    //or
    //use itertools::Itertools;
    //  s.split_whitespace().unique().join(" ")
}