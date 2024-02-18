pub fn input_to_regex_string(input: &str) -> String {
    let str = replace_multiple(
        &input.to_lowercase(),
        vec![
            ("a", "[a,á]"),
            ("e", "[e,é,ě]"),
            ("y", "[y,ý]"),
            ("n", "[n,ň]"),
            ("c", "[c,č]"),
            ("r", "[r,ř]"),
            ("z", "[z,ž]"),
            ("s", "[s,š]"),
            ("t", "[t,ť]"),
            ("d", "[d,ď]"),
            ("u", "[u,ů,ú]"),
            ("i", "[i,í]"),
            ("o", "[o,ó]"),
        ],
    );
   str.to_string()
}
fn replace_multiple(input: &str, replacements: Vec<(&str, &str)>) -> String {
    let mut result = input.to_string();
    for (from, to) in replacements {
        result = result.replace(from, to);
    }
    result
}
