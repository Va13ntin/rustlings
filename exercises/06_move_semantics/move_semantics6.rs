// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

/// la fonction string_uppercase prend une référence constante (&String) plutôt qu'une référence mutable. De plus une nouvelle variable uppercased contenant la version en majuscules de la chaîne de caractères a été créé

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    let uppercased = data.to_uppercase();

    println!("{}", uppercased);
}
