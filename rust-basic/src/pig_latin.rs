fn main() {
    let given = String::from("first");

    println!("Converted: {}", convert(&given))
    
}

fn convert(s: &String) -> String {
    let mut converted = String::new();
    let first = s.chars().nth(0);
    if let Some(f) = first {
        match f {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                converted.push_str(&format!("{s}-hay")[..]);
            },
            _ => {
                converted.push_str(&format!("{}-{}ay", &s[1..], f)[..]);
            },
        }
    }

    return converted;
}
