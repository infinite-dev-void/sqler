use std::ops::{Deref, DerefMut};

const HTML_QUOT: &str = "&#34;";
const HTML_APOS: &str = "&#39;";
const HTML_AMP: &str = "&amp;";
const HTML_LT: &str = "&lt;";
const HTML_GT: &str = "&gt;";
const HTML_NULL: &str = "\u{FFFD}";

struct XssChar {
    position: usize,
    replace_with: &'static str,
}

pub struct XssString(pub String);

impl XssString {
    pub fn new() -> XssString {
        XssString(String::new())
    }
}

impl Deref for XssString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for XssString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::VarToSql for XssString {
    fn sql(&self) -> String {
        let mut capacity = 0;
        let mut xss_char_indices = Vec::<XssChar>::new();

        for (i, b) in self.0.as_bytes().iter().enumerate() {
            match *b {
                b'\0' => {
                    capacity += 5;
                    xss_char_indices.push(XssChar {
                        position: i,
                        replace_with: &HTML_NULL,
                    });
                }
                b'"' => {
                    capacity += 5;
                    xss_char_indices.push(XssChar {
                        position: i,
                        replace_with: &HTML_QUOT,
                    });
                }
                b'\'' => {
                    capacity += 5;
                    xss_char_indices.push(XssChar {
                        position: i,
                        replace_with: &HTML_APOS,
                    });
                }
                b'&' => {
                    capacity += 5;
                    xss_char_indices.push(XssChar {
                        position: i,
                        replace_with: &HTML_AMP,
                    });
                }
                b'<' => {
                    capacity += 5;
                    xss_char_indices.push(XssChar {
                        position: i,
                        replace_with: &HTML_LT,
                    });
                }
                b'>' => {
                    capacity += 5;
                    xss_char_indices.push(XssChar {
                        position: i,
                        replace_with: &HTML_GT,
                    });
                }
                _ => {
                    capacity += 1;
                }
            }
        }

        if xss_char_indices.len() == 0 {
            if capacity > 0 {
                let mut s = String::from('\'');
                s.push_str(&self.0.replace("'", "''"));
                s.push('\'');
                return s;
            } else {
                return String::from("''");
            }
        }

        let mut s = String::with_capacity(capacity + 2);
        s.push('\'');
        let mut last: usize = 0;
        for xss_char in xss_char_indices {
            s.push_str(&self.0[last..xss_char.position]);
            s.push_str(&xss_char.replace_with);
            last = xss_char.position + 1;
        }

        if last >= self.0.len() {
            s.push_str(&self.0[last..]);
        }
        s.push('\'');
        s
    }
}
