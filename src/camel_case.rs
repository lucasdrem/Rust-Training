pub fn run(str: &str) -> String {
    let mut camel_case_word = String::new();
    for word in str.split_whitespace() {
        let mut c = word.chars();
        let uppercase_first_letter_word = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };
        
        camel_case_word.push_str(&uppercase_first_letter_word);
    }
    
    return camel_case_word;

    //or
    // str.split_whitespace()
    // .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
    // .collect()
}