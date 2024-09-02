#[derive(Debug)]
struct Sg {
    need_leading_space: bool,
    sg: String,
    need_trailing_space: bool,
    len: usize,
}

impl Sg {
    fn new(need_leading_space: bool, sg: &str, need_trailing_space: bool) -> Sg {
        let mut len = sg.len();
        if need_leading_space {
            len += 1;
        }

        if need_trailing_space {
            len += 1;
        }

        Sg {
            need_leading_space,
            sg: String::from(sg),
            need_trailing_space,
            len,
        }
    }
}

pub struct StmtSg {
    sgs: Vec<Sg>,
    len: usize,
}

impl StmtSg {
    pub fn new() -> StmtSg {
        Self {
            sgs: Vec::new(),
            len: 0,
        }
    }

    /* pub fn construct_with_one(need_leading_space: bool, sg:String, need_trailing_space: bool) -> StmtSg {

    } */

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn add_token(
        &mut self,
        need_leading_space: bool,
        sg: &str,
        need_trailing_space: bool,
    ) -> &mut Self {
        let sg = Sg::new(need_leading_space, sg, need_trailing_space);
        self.len += sg.len;
        self.sgs.push(sg);

        self
    }

    pub fn construct(&self) -> String {
        let mut stmt_sg = String::from('"');

        let mut trailing_space_added = false;
        for sg in &self.sgs {
            if sg.need_leading_space && !trailing_space_added {
                stmt_sg.push(' ');
            }

            stmt_sg.push_str(&sg.sg);

            if sg.need_trailing_space {
                stmt_sg.push(' ');
                trailing_space_added = true;
            } else {
                trailing_space_added = false;
            }
        }
        stmt_sg.push('"');
        stmt_sg
    }
}
