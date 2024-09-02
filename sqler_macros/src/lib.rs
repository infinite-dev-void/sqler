//! # SQLer Macros
//!
//! `sqler_macors` is a collection of procedure macros used by
//! `sqler` crate

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

mod vars;
use vars::*;

mod hprs;
use hprs::*;

mod stmt_sg;
use stmt_sg::*;

mod need_space;
use need_space::*;

/// Convert Rust token stream to a String SQL Statement
/// and concatenate any variable between curly braces.
#[proc_macro]
pub fn sql(ts: TokenStream) -> TokenStream {
    let mut sqler = Sqler::new();

    let sql = sqler.construct_from_rust(ts);

    sql
}

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
    stmt_sg: StmtSg,
    stmt_len: usize,
    stmt_sgs: Vec<String>,
    vars: Vars,
}

impl Sqler {
    fn new() -> Sqler {
        Sqler {
            stmt_sg: StmtSg::new(),
            stmt_len: 0,
            stmt_sgs: Vec::new(),
            vars: Vars::new(),
        }
    }

    fn construct_from_rust(&mut self, ts: TokenStream) -> TokenStream {
        for tt in ts {
            self.match_token_tree(tt);
        }
        if self.stmt_sg.len() > 0 {
            self.stmt_len += self.stmt_sg.len();
            self.stmt_sgs.push(self.stmt_sg.construct());
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
            let mut sg = String::from('\'');

            sg.push_str(&l[1..l.len() - 1].replace("'", "''"));

            sg.push('\'');

            self.stmt_sg.add_token(false, &sg, false);

            return;
        }

        if l.starts_with('\'') {
            let mut sg = String::from('\'');
            sg.push('\'');
            sg.push_str(&l[1..2].replace("'", "''"));
            sg.push('\'');
            self.stmt_sg.add_token(false, &sg, true);
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
        self.stmt_sg.add_token(
            false,
            &remove_num_prefix_suffix(num).replace("_", ""),
            false,
        );
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
        self.stmt_sg = StmtSg::new();
    }

    #[inline]
    fn add_stmt_sg_to_sgs(&mut self) {
        self.stmt_len += self.stmt_sg.len();

        self.stmt_sgs.push(self.stmt_sg.construct());
    }

    // [] array or cast
    #[inline]
    fn handle_group_bracket(&mut self, g: Group) {
        self.stmt_sg.add_token(false, "[", false);
        for tt in g.stream() {
            self.match_token_tree(tt);
        }

        self.stmt_sg.add_token(false, "]", false);
    }

    #[inline]
    fn handle_group_parenthesis(&mut self, g: Group) {
        self.stmt_sg.add_token(false, "(", false);
        for tt in g.stream() {
            self.match_token_tree(tt);
        }

        self.stmt_sg.add_token(false, ")", false);
    }

    fn handle_ident(&mut self, i: Ident) {
        let i = i.to_string();
        let lower_i = i.to_lowercase();

        self.stmt_sg.add_token(
            IDENT_NEED_LEADING_SPACE.contains(&&lower_i[..]),
            &i,
            IDENT_NEED_TRAILING_SPACE.contains(&&lower_i[..]),
        );
    }

    /*     fn handle_ident_leading_space(&mut self, i: &String) {
           if self.stmt_sg.ends_with(' ') {
               return;
           }

           for ident in [
               "OR", "AND", "BETWEEN", "LEFT", "RIGHT", "CROSS", "JOIN", "LITERAL",
           ] {
               if i == ident {
                   self.stmt_sg.push(' ');
                   return;
               };
           }
       }
    */
    /*
    '=', '<', '>', '!', '~', '+', '-', '*', '/', '%', '^', '&', '|', '@', '.', ',', ';',
               ':', '#', '$', '?', '\'', */
    fn handle_punct(&mut self, p: Punct) {
        let c = p.as_char();
        /* self.handle_punct_leading_space(c); */

        self.stmt_sg.add_token(false, &c.to_string(), false);
        /* self.handle_punct_trailing_space(c); */
    }

    /*    #[inline]
       fn handle_punct_leading_space(&mut self, p: char) {
           if p == '=' {
               if self.stmt_sg.ends_with("> ") || self.stmt_sg.ends_with("< ") {
                   remove_space_at_end(&mut self.stmt_sg);
                   return;
               }
               if self.stmt_sg.ends_with('!') {
                   return;
               }
           }

           if p == '>' && self.stmt_sg.ends_with("< ") {
               remove_space_at_end(&mut self.stmt_sg);
               return;
           }

           for pucnt in ['.', ':', ','] {
               if pucnt == p {
                   remove_space_at_end(&mut self.stmt_sg);
                   return;
               }
           }

           for punct in ['&', '|'] {
               if punct == p {
                   if self.stmt_sg.ends_with(punct) {
                       return;
                   }
                   add_space_at_end(&mut self.stmt_sg);
                   return;
               };
           }

           // Duplicate because other possibilities may be added in the future.
           add_space_at_end(&mut self.stmt_sg);
       }

       #[inline]
       fn handle_punct_trailing_space(&mut self, p: char) {
           for punct in ['*', '=', '<', '>', '+', '-', '/', ','] {
               if punct == p {
                   add_space_at_end(&mut self.stmt_sg);
                   return;
               }
           }
       }
    */
    fn construct(&mut self) -> TokenStream {
        if self.stmt_sgs.len() == 1 {
            return (String::from("String::from(") + &self.stmt_sgs[0] + ")")
                .parse()
                .unwrap();
        };

        let mut stmt = String::new();
        stmt.push_str("{\n");

        stmt.push_str(&self.vars.construct_defs_of_vars());

        stmt.push_str("let mut ___sql___ = String::with_capacity(");
        stmt.push_str(&self.stmt_len.to_string());

        stmt.push_str(&self.vars.construct_getting_len_of_vars());

        stmt.push_str(");\n");

        for sg in &self.stmt_sgs {
            stmt.push_str("___sql___.push_str(");
            stmt.push_str(&sg);
            stmt.push_str(");\n");
        }

        stmt.push_str("___sql___\n}");

        stmt.parse().unwrap()
    }
}
