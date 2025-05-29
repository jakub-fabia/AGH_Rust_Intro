pub fn match_image(condition: &str, is_day: bool) -> Vec<&'static str> {
    match (condition.trim().to_lowercase().as_str(), is_day) {
        ("sunny", true) => vec![
            "   \\ | /   ",
            "    ☀️     ",
            "   / | \\   ",
        ],
        ("clear", false) => vec![
            "     🌙     ",
            "   🌙🌙🌙  ",
            "    ~~~     ",
        ],
        ("partly cloudy", true) => vec![
            "    ☁☀☁    ",
            "   ☀☁☀   ",
            "   \\☁☀/   ",
        ],
        ("partly cloudy", false) => vec![
            "    ☁🌙☁    ",
            "   🌙☁🌙   ",
            "   /🌙☁\\   ",
        ],
        ("cloudy", true) => vec![
            "    ☁☁☁    ",
            "   ☁☁☁☁   ",
            "   ────   ",
        ],
        ("cloudy", false) => vec![
            "    ☁☁☁    ",
            "   ☁☁☁☁   ",
            "   ────   ",
        ],
        ("overcast", true) => vec![
            "   ☁☁☁☁   ",
            "  ☁☁☁☁☁  ",
            "   ~~~~~   ",
        ],    
        ("overcast", false) => vec![
            "   ☁☁☁☁   ",
            "  ☁☁☁☁☁  ",
            "   ~~~~~   ",
        ],
        ("mist", true) => vec![
            "   ~~~~~   ",
            "  ~~~~~~~  ",
            "   .....   ",
        ],    
        ("mist", false) => vec![
            "   ~~~~~   ",
            "  ~~~~~~~  ",
            "   .....   ",
        ],    
        ("patchy rain possible", true) => vec![
            "    ☀☁🌧    ",
            "    🌧🌧🌧    ",
            "   ~~~~~   ",
        ],
        ("patchy rain possible", false) => vec![
            "    🌙☁🌧    ",
            "    🌧🌧🌧    ",
            "   ~~~~~   ",
        ],
        ("fog", true) => vec![
            "    ☁     ",
            "   ~~~~~   ",
            "  ~~~~~~~  ",
        ],    
        ("fog", false) => vec![
            "    🌙     ",
            "   ~~~~~   ",
            "  ~~~~~~~  ",
        ],
        ("patchy light drizzle", true) => vec![
            "    ☁💧☁    ",
            "    💧💧💧    ",
            "   . . .   ",
        ],    
        ("patchy light drizzle", false) => vec![
            "    ☁💧☁    ",
            "    💧💧💧    ",
            "   . . .   ",
        ],
        ("light drizzle", true) => vec![
            "    ☁💧☁    ",
            "    💧💧💧    ",
            "   .....   ",
        ],    
        ("light drizzle", false) => vec![
            "    ☁💧☁    ",
            "    💧💧💧    ",
            "   .....   ",
        ],    
        ("patchy light rain", true) => vec![
            "    ☀☁🌦    ",
            "    🌦💧🌦    ",
            "   . . .   ",
        ],    
        ("patchy light rain", false) => vec![
            "    🌙☁🌦    ",
            "    🌦💧🌦    ",
            "   . . .   ",
        ],
        ("light rain", true) => vec![
            "    ☁☔☁    ",
            "    ☔☔☔    ",
            "   . . .   ",
        ],    
        ("light rain", false) => vec![
            "    🌙☔☁    ",
            "    ☔☔☔    ",
            "   . . .   ",
        ],
        ("moderate rain at times", true) => vec![
            "    ☀☁🌧    ",
            "    🌧🌧🌧    ",
            "   ~~~~~   ",
        ],    
        ("moderate rain at times", false) => vec![
            "    🌙☁🌧    ",
            "    🌧🌧🌧    ",
            "   ~~~~~   ",
        ],
        ("moderate rain", true) => vec![
            "    ☁🌧☁    ",
            "    🌧☔🌧    ",
            "   :::::   ",
        ],    
        ("moderate rain", false) => vec![
            "    🌙☔☁    ",
            "    🌧☔🌧    ",
            "   :::::   ",
        ],
        ("heavy rain at times", true) => vec![
            "    ☀☁⛈️    ",
            "    ⛈️⛈️⛈️    ",
            "   /////   ",
        ],    
        ("heavy rain at times", false) => vec![
            "    🌙☁⛈️    ",
            "    ⛈️⛈️⛈️    ",
            "   /////   ",
        ],
        ("heavy rain", true) => vec![
            "    ☁⛈️☁    ",
            "    ⛈️⛈️⛈️    ",
            "   ≡≡≡≡≡   ",
        ],    
        ("heavy rain", false) => vec![
            "    🌙⛈️☁    ",
            "    ⛈️⛈️⛈️    ",
            "   ≡≡≡≡≡   ",
        ],
        ("light rain shower", true) => vec![
            "    ☁💧☁    ",
            "  💧☔☔💧  ",
            "   .:.:.   ",
        ],    
        ("light rain shower", false) => vec![
            "    🌙💧☁    ",
            "  💧☔☔💧  ",
            "   .:.:.   ",
        ],
        ("patchy rain nearby", true) => vec![
            "   ☀☁🌧   ",
            "  🌧💧🌧  ",
            "   ~~~~~   ",
        ],    
        ("patchy rain nearby", false) => vec![
            "   🌙☁🌧   ",
            "  🌧💧🌧  ",
            "   ~~~~~   ",
        ],
        _ => vec![
            "[no art]",
            "<unknown condition>",
            "¯\\_(ツ)_/¯",
        ],
    }
}