pub fn cn(inputs: &[&str]) -> String {
    inputs
        .iter()
        .filter(|&&s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
