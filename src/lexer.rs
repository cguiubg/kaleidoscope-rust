use getchar::getchar;

enum Token {
    EOF=-1,
    Def=-2,
    Extern=-3,
    Identifier=-4,
    Number=-5
}

fn isspace(character: Option<char>) -> bool {
    match character {
        Some(' ') => { return true; }
        Some('\t') => { return true; }
        _ => { return false; }
    }
}

fn isalpha(character: Option<char>) -> bool {
    match character {
        Some(c) => { return c.is_alphabetic(); }
        _ => { return false; }
    }
}

fn isnum(character: Option<char>) -> bool {
    match character {
        Some(c) => { return c.is_numeric(); }
        _ => { return false; }
    }
}

fn isalphanum(character: Option<char>) -> bool {
    return isalpha(character) || isnum(character);
}


pub fn consume_chars() { 
    let mut num_value = 0;
    let mut identifier_name = String::from("");
    let mut character_opt = getchar();

    // Consume white space
    while let Some(character) = character_opt {
        match isspace(Some(character)) {
            true => { character_opt = getchar(); }
            false => { break; }
        }
    }

    if isalpha(character_opt) && let Some(c) = character_opt {
        identifier_name.clear();
        identifier_name.push(c);
        while isalphanum(character_opt) {
            match character_opt {
                Some(character) => { identifier_name.push(character); }
                _ => { break; }
            }
            character_opt = getchar();
        }
        println!("{}", identifier_name);
    }
}

