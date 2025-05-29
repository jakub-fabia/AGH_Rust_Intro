// Funkcja do generowania obrazków pogodowych na podstawie warunków
// Wejścia: `condition` - warunek pogodowy jako string, `is_day` - czy jest dzień (bool)
// Wyjście: `Vec<String>` - wektor z liniami tekstu reprezentującymi obrazek

pub fn match_image(condition: &str, is_day: bool) -> Vec<String> {
    match (condition.trim().to_lowercase().as_str(), is_day) {
        ("sunny", true) => vec![
            centered_line(r"\ | /", 13),
            centered_line("☼", 13),
            centered_line(r"/ | \\", 13),
        ],
        ("clear", false) => vec![
            centered_line("☾", 13),
            centered_line("☾☾☾", 13),
            centered_line("~~~", 13),
        ],
        ("partly cloudy", true) => vec![
            centered_line("☁☀☁", 13),
            centered_line("☀☁☀", 13),
            centered_line(r"\\☁☀/", 13),
        ],
        ("partly cloudy", false) => vec![
            centered_line("☁☾☁", 13),
            centered_line("☾☁☾", 13),
            centered_line(r"/☾☁\\", 13),
        ],
        ("cloudy", true) => vec![
            centered_line("☁☁☁", 13),
            centered_line("☁☁☁☁☁", 13),
            centered_line("─────", 13),
        ],
        ("cloudy", false) => vec![
            centered_line("☁☁☁", 13),
            centered_line("☁☁☁☁☁", 13),
            centered_line("─────", 13),
        ],
        ("overcast", true) => vec![
            centered_line("☁☁☁☁☁", 13),
            centered_line("☁☁☁☁☁", 13),
            centered_line("~~~~~", 13),
        ],    
        ("overcast", false) => vec![
            centered_line("☁☁☁☁☁", 13),
            centered_line("☁☁☁☁☁", 13),
            centered_line("~~~~~", 13),
        ],
        ("mist", true) => vec![
            centered_line("~~~~~", 13),
            centered_line("~~~~~~~~~", 13),
            centered_line("~~~~~", 13),
        ],    
        ("mist", false) => vec![
            centered_line("~~~~~", 13),
            centered_line("~~~~~~~~~", 13),
            centered_line(".....", 13),
        ],    
        ("patchy rain possible", true) => vec![
            centered_line("☀☁🌧", 13),
            centered_line("🌧🌧🌧", 13),
            centered_line("~~~~~", 13),
        ],
        ("patchy rain possible", false) => vec![
            centered_line("☾☁🌧", 13),
            centered_line("🌧🌧🌧", 13),
            centered_line("~~~~~", 13),
        ],
        ("fog", true) => vec![
            centered_line("☁", 13),
            centered_line("~~~~~", 13),
            centered_line("~~~~~~~", 13),
        ],    
        ("fog", false) => vec![
            centered_line("☾", 13),
            centered_line("~~~~~", 13),
            centered_line("~~~~~~~", 13),
        ],
        ("patchy light drizzle", true) => vec![
            centered_line("☁𓄼☁", 13),
            centered_line("𓄼𓄼𓄼", 13),
            centered_line(". .", 13),
        ],    
        ("patchy light drizzle", false) => vec![
            centered_line("☁𓄼☁", 13),
            centered_line("𓄼𓄼𓄼", 13),
            centered_line(". .", 13),
        ],
        ("light drizzle", true) => vec![
            centered_line("☁𓄼☁", 13),
            centered_line("𓄼𓄼𓄼", 13),
            centered_line("...", 13),
        ],    
        ("light drizzle", false) => vec![
            centered_line("☁𓄼☁", 13),
            centered_line("𓄼𓄼𓄼", 13),
            centered_line("...", 13),
        ],    
        ("patchy light rain", true) => vec![
            centered_line("☀☁🌦", 13),
            centered_line("🌦𓄼🌦", 13),
            centered_line(". .", 13),
        ],    
        ("patchy light rain", false) => vec![
            centered_line("☾☁🌦", 13),
            centered_line("🌦𓄼🌦", 13),
            centered_line(". .", 13),
        ],
        ("light rain", true) => vec![
            centered_line("☁🌧︎☁", 13),
            centered_line("🌧︎︎☁︎🌧", 13),
            centered_line(". .", 13),
        ],    
        ("light rain", false) => vec![
            centered_line("☾🌧︎☁", 13),
            centered_line("🌧︎︎☁︎🌧", 13),
            centered_line(". .", 13),
        ],
        ("moderate rain at times", true) => vec![
            centered_line("☀☁🌧", 13),
            centered_line("🌧🌧🌧", 13),
            centered_line("~~~", 13),
        ],    
        ("moderate rain at times", false) => vec![
            centered_line("☾☁🌧", 13),
            centered_line("🌧🌧🌧", 13),
            centered_line("~~~", 13),
        ],
        ("moderate rain", true) => vec![
            centered_line("☁🌧☁", 13),
            centered_line("🌧🌧🌧", 13),
            centered_line(":::", 13),
        ],    
        ("moderate rain", false) => vec![
            centered_line("☾🌧︎☁", 13),
            centered_line("🌧🌧︎🌧", 13),
            centered_line(":::", 13),
        ],
        ("heavy rain at times", true) => vec![
            centered_line("☀☁⛈", 13),
            centered_line("⛈⛈⛈", 13),
            centered_line("///", 13),
        ],    
        ("heavy rain at times", false) => vec![
            centered_line("☾☁⛈", 13),
            centered_line("⛈⛈⛈", 13),
            centered_line("///", 13),
        ],
        ("heavy rain", true) => vec![
            centered_line("☁⛈☁", 13),
            centered_line("⛈⛈⛈", 13),
            centered_line("≡≡≡", 13),
        ],    
        ("heavy rain", false) => vec![
            centered_line("☾⛈☁", 13),
            centered_line("⛈⛈⛈", 13),
            centered_line("≡≡≡", 13),
        ],
        ("light rain shower", true) => vec![
            centered_line("☁𓄼☁", 13),
            centered_line("𓄼🌧︎︎𓄼", 13),
            centered_line(".:.", 13),
        ],    
        ("light rain shower", false) => vec![
            centered_line("☾𓄼☁", 13),
            centered_line("𓄼🌧︎𓄼", 13),
            centered_line(".:.", 13),
        ],
        ("patchy rain nearby", true) => vec![
            centered_line("☀☁🌧", 13),
            centered_line("🌧𓄼🌧", 13),
            centered_line("~~~", 13),
        ],    
        ("patchy rain nearby", false) => vec![
            centered_line("☾☁🌧", 13),
            centered_line("🌧𓄼🌧", 13),
            centered_line("~~~", 13),
        ],
        _ => vec![
            "[no art]".to_string(),
            "<unknown condition>".to_string(),
            "¯\\_(ツ)_/¯".to_string(),
        ],
    }
}

fn centered_line(content: &str, width: usize) -> String {
    let pad = width.saturating_sub(content.chars().count());
    let left = pad / 2;
    let right = pad - left;
    format!("{}{}{}", " ".repeat(left), content, " ".repeat(right))
}