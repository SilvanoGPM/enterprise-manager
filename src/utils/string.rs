pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            capitalize_next = true;
            result.push(c);
        } else {
            if capitalize_next {
                result.extend(c.to_uppercase());
            } else {
                result.push(c);
            }
            capitalize_next = false;
        }
    }

    result
}