// Used to count the number of times a variable appears.
struct Var {
    name: String,
    count: usize,
}

// Used to count the number of times each variable appears..
pub struct Vars(Vec<Var>);

impl Vars {
    pub fn new() -> Vars {
        Vars(Vec::new())
    }

    pub fn add(&mut self, name: &str) {
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

    pub fn construct_defs_of_vars(&self) -> String {
        let mut defs = String::with_capacity(38 * self.0.len());
        // estimated
        for var in &self.0 {
            defs.push_str("let ");
            defs.push_str(&var.name);
            defs.push_str(" = ::sqler::VarToSql::sql(&");
            defs.push_str(&var.name);
            defs.push_str(");\n");
        }
        defs
    }

    pub fn construct_getting_len_of_vars(&self) -> String {
        let mut getting_lens = String::with_capacity(12 * self.0.len()); // estimated
        for var in &self.0 {
            getting_lens.push_str(" + ");
            getting_lens.push_str(&var.name);
            getting_lens.push_str(".len()");
            if var.count > 1 {
                getting_lens.push_str(" * ");
                getting_lens.push_str(&var.count.to_string());
            }
        }

        getting_lens
    }
}
