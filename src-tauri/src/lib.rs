use rand::prelude::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn password(length: usize) -> String {
    let mut result = String::new();
    for _ in 0..length {
        result.push(random_char());
    }
    result
}

fn random_char() -> char {    
    let mut all_characters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    all_characters.extend(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']);
    all_characters.extend(vec!['å', 'ä', 'ö']);
    all_characters.extend(vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    all_characters.extend(vec!['!', '%', '-', '_']);

    let mut rng = thread_rng();
    let index = rng.gen_range(0..all_characters.len());
    all_characters[index]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password() {
        let length: usize = 8;
        let pwd = password(length);
        assert_eq!(pwd.len(), length);
    }

    #[test]
    fn test_random_char() {
        let mut counts = [0; 3];

        for _ in 0..10000 {
            let c = random_char();
            match c {
                'a' => counts[0] += 1,
                'b' => counts[1] += 1,
                'c' => counts[2] += 1,
                _ => panic!("Unexpected character returned: {}", c),
            }
        }

        for (i, count) in counts.iter().enumerate() {
            assert!(*count > 1, "Not enough instances of character {}", i);
        }
    }
}
