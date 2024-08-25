//! # SQLer Macros
//!
//! `sqler_macors` is a collection of procedure macros used by
//! `sqler` crate

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

/* fn count_hash_in_raw_string(s: &String) -> usize {
    let mut hash_count: usize = 0;

    for c in s[1..].chars() {
        if c == '#' {
            hash_count += 1;
        } else {
            break;
        }
    }

    hash_count
} */
struct Sqler {
    stmt_sg: String,
    stmt_len: usize,
    stmt_sgs: Vec<String>,
    vars: Vars,
}

impl Sqler {
    fn new() -> Sqler {
        Sqler {
            stmt_sg: String::new(),
            stmt_len: 0,
            stmt_sgs: Vec::new(),
            vars: Vars::new(),
        }
    }

    fn construct_from_rust(&mut self, ts: TokenStream) -> TokenStream {
        for tt in ts {
            self.match_token_tree(tt);
        }

        self.stmt_sg = self.stmt_sg.trim().to_owned();
        if self.stmt_sg.len() > 0 {
            self.stmt_len += self.stmt_sg.len();
            self.stmt_sgs
                .push(String::from("\"") + &self.stmt_sg + "\"");
        }

        self.construct()
    }

    fn match_token_tree(&mut self, tt: TokenTree) {
        match tt {
            TokenTree::Literal(l) => {
                self.handle_literal(l);
            }

            TokenTree::Group(g) => {
                self.handle_group(g);
            }

            TokenTree::Ident(i) => {
                self.handle_ident(i);
            }

            TokenTree::Punct(p) => {
                self.handle_punct(p);
            }
        }
    }

    fn handle_literal(&mut self, l: Literal) {
        let l = l.to_string();

        if l.starts_with('"') {
            self.stmt_sg.push('\'');

            self.stmt_sg.push_str(&l[1..l.len() - 1].replace("'", "''"));

            self.stmt_sg.push_str("' ");

            return;
        }

        if l.starts_with('\'') {
            self.stmt_sg.push('\'');
            self.stmt_sg.push_str(&l[1..2]);
            self.stmt_sg.push_str("' ");

            return;
        }

        if l.starts_with('r') {
            panic!("raw string literal not supported");
            /* let mut s = String::from('\'');

            let hash_count = count_hash_in_raw_string(&l);

            let rs_cont = l[2 + hash_count..l.len() - 1 - hash_count]
                .replace(r#"""#, r#"\""#)
                .replace("'", "''");

            s.push_str(&rs_cont);

            s.push('\'');

            return s; */
        }

        if l.starts_with('c') {
            panic!("c string literal not supported")
        }

        if l.starts_with('b') {
            panic!("bytes is not supported")
        }

        // number
        self.handle_literal_number(l);
        //
        //
        //
    }

    fn handle_literal_number(&mut self, num: String) {
        self.stmt_sg
            .push_str(&remove_num_prefix_suffix(num).replace("_", ""));
        self.stmt_sg.push(' ');
    }

    fn handle_group(&mut self, g: Group) {
        let d = g.delimiter();
        match d {
            Delimiter::Brace => {
                self.handle_group_brace(g);
            } // {} --> variable

            Delimiter::Bracket => {
                self.handle_group_bracket(g);
            } // [] --> array or cast
            Delimiter::Parenthesis => {
                self.handle_group_parenthesis(g);
            } // () --> tuple == row
            Delimiter::None => {
                panic!("unexpected token")
            }
        }
    }

    // variable
    #[inline]
    fn handle_group_brace(&mut self, g: Group) {
        self.add_stmt_sg_to_sgs();

        let deli = g.to_string();
        let var_name = deli[1..deli.len() - 1].trim();

        self.stmt_sgs.push(String::from("&") + var_name);

        self.vars.add(var_name);
        self.stmt_sg = String::from(' ');
    }

    #[inline]
    fn add_stmt_sg_to_sgs(&mut self) {
        self.stmt_len += self.stmt_sg.len();

        self.stmt_sgs
            .push(String::from("\"") + &self.stmt_sg + "\"");
    }

    // [] array or cast
    #[inline]
    fn handle_group_bracket(&mut self, g: Group) {
        self.stmt_sg.push('[');
        for tt in g.stream() {
            self.match_token_tree(tt);
        }
        delete_last_space_from_string(&mut self.stmt_sg);
        self.stmt_sg.push_str("] ");
    }

    #[inline]
    fn handle_group_parenthesis(&mut self, g: Group) {
        self.stmt_sg.push('(');
        for tt in g.stream() {
            self.match_token_tree(tt);
        }

        delete_last_space_from_string(&mut self.stmt_sg);
        self.stmt_sg.push_str(") ");
    }

    fn handle_ident(&mut self, i: Ident) {
        self.stmt_sg.push_str(&i.to_string());
        self.stmt_sg.push(' ');
    }
    /*
    '=', '<', '>', '!', '~', '+', '-', '*', '/', '%', '^', '&', '|', '@', '.', ',', ';',
               ':', '#', '$', '?', '\'', */
    fn handle_punct(&mut self, p: Punct) {
        let c = p.as_char();

        if c == ',' {
            delete_last_space_from_string(&mut self.stmt_sg);

            self.stmt_sg.push_str(", ");
            return;
        }

        if c == '*' {
            self.stmt_sg.push_str("* ");
            return;
        }

        if c == '=' {
            self.stmt_sg.push_str("= ");
            return;
        }

        if c == '|' {
            self.stmt_sg.push_str(if self.stmt_sg.ends_with("|") {
                "| "
            } else {
                " |"
            });

            return;
        };

        if c == '&' {
            self.stmt_sg.push_str(if self.stmt_sg.ends_with("&") {
                "& "
            } else {
                " &"
            });

            return;
        };

        if c == ':' {
            delete_last_space_from_string(&mut self.stmt_sg);
            self.stmt_sg.push(':');

            return;
        };

        self.stmt_sg.push(c);
    }

    fn construct(&mut self) -> TokenStream {
        if self.stmt_sgs.len() == 1 {
            return (String::from("String::from(") + &self.stmt_sgs[0] + ")")
                .parse()
                .unwrap();
        };

        let mut stmt = String::new();
        stmt.push_str("{\n");

        for var in &self.vars.0 {
            stmt.push_str("let ");
            stmt.push_str(&var.name);
            stmt.push_str(" = ::sqler::VarToSql::sql(&");
            stmt.push_str(&var.name);
            stmt.push_str(");\n");
        }

        stmt.push_str("let mut ___sql___ = String::with_capacity(");
        stmt.push_str(&self.stmt_len.to_string());

        for var in &self.vars.0 {
            stmt.push_str(" + ");
            stmt.push_str(&var.name);
            stmt.push_str(".len()");
            if var.count > 1 {
                stmt.push_str(" * ");
                stmt.push_str(&var.count.to_string());
            }
        }

        stmt.push_str(");\n");

        for sg in &self.stmt_sgs {
            stmt.push_str("___sql___.push_str(");
            stmt.push_str(sg);
            stmt.push_str(");\n");
        }

        stmt.push_str("___sql___\n}");

        stmt.parse().unwrap()
    }
}

/// Convert Rust token stream to a String SQL Statement
/// and concatenate any variable between curly braces.
#[proc_macro]
pub fn sql(ts: TokenStream) -> TokenStream {
    let mut sqler = Sqler::new();
    sqler.construct_from_rust(ts)
}

// Used to count the number of times a variable appears.
struct Var {
    name: String,
    count: usize,
}

// Used to count the number of times each variable appears..
struct Vars(Vec<Var>);

impl Vars {
    fn new() -> Vars {
        Vars(Vec::new())
    }

    fn add(&mut self, name: &str) {
        for var in self.0.iter_mut() {
            if var.name == name {
                var.count += 1;
                return;
            }
        }
        self.0.push(Var {
            name: name.to_owned(),
            count: 1,
        });
    }
}

// Remove the suffix at the end of the number if exists
// Example: (15i32 -> 15, 101usize -> 101, 7 -> 7)

fn remove_num_suffix(n: String) -> String {
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

fn remove_num_prefix_suffix(n: String) -> String {
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

fn delete_last_space_from_string(s: &mut String) {
    if s.ends_with(' ') {
        *s = String::from(&s[0..s.len() - 1]);
    }
}
