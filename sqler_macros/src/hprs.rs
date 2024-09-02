// Remove the suffix at the end of the number if exists
// Example: (15i32 -> 15, 101usize -> 101, 7 -> 7)

pub fn remove_num_suffix(n: String) -> String {
    match n.find('i') {
        Some(idx) => {
            return n[0..idx].to_string();
        }
        _ => {}
    };

    match n.find('u') {
        Some(idx) => {
            return n[0..idx].to_string();
        }
        _ => {}
    };

    n
}

// Remove prefix and suffix from the number if exist.
// As a side effect of removing the prefix from the
// number, it will be converted back to decimal.
// example: (0x1fusize -> 31, 0o77 -> 63, 10 -> 10)

pub fn remove_num_prefix_suffix(n: String) -> String {
    if n.starts_with("0x") {
        return i128::from_str_radix(&remove_num_suffix(n)[2..], 16)
            .unwrap()
            .to_string();
    };

    if n.starts_with("0o") {
        return i128::from_str_radix(&remove_num_suffix(n)[2..], 8)
            .unwrap()
            .to_string();
    };

    if n.starts_with("0b") {
        return i128::from_str_radix(&remove_num_suffix(n)[2..], 2)
            .unwrap()
            .to_string();
    };

    return remove_num_suffix(n);
}

/* #[inline]
pub fn remove_space_at_end(s: &mut String) {
    if s.ends_with(' ') {
        *s = String::from(&s[0..s.len() - 1]);
    }
}

#[inline]
pub fn add_space_at_end(s: &mut String) {
    if s.ends_with(' ') {
        return;
    }
    s.push(' ');
}
 */
