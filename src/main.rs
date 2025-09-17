
fn next_language<'a>(languages: &'a [String], lang_to_search: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found { 
            return lang;
        }
        if lang == lang_to_search {
            found = true
        }
    }
    languages.last().unwrap()
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),        
    ];

    let result = next_language(&languages, "go");

    println!("{}", result);
}
