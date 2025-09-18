
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

fn last_language(languages: &[String] ) -> &str {
    languages.last().unwrap()
}

fn longest_lang<'a>(lang1: &'a str, lang2: &'a str) -> &'a str {
    if lang1.len() > lang2.len()  {
        lang1
    } else {
        lang2
    }
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),        
    ];

    //let result = next_language(&languages, "go");
    let result = longest_lang("Python", "COBOL");

    println!("{}", result);
}
