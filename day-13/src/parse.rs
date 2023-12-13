pub fn parse_input(input: &str) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut inner = Vec::new();
    for line in input.lines() {
        let l = line.to_string();
        if line.is_empty() {
            result.push(inner);
            inner = Vec::new();
        } else {
            inner.push(l);
        }
    }
    result.push(inner);
    result
}
