pub fn twofer(name: &str) -> String {
    let me = if name.is_empty() {
        "you"
    } else {
        name
    };
    format!("One for {}, one for me.", me)
}

