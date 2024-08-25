mod xss_string;

pub use xss_string::XssString;

pub use sqler_macros::*;

pub trait VarToSql {
    fn sql(&self) -> String;
}

impl VarToSql for String {
    #[inline]
    fn sql(&self) -> String {
        String::from("'") + &self.replace("'", "''") + "'"
    }
}

impl VarToSql for i8 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for i16 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for i32 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for i64 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for i128 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for isize {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for u8 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for u16 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for u32 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for u64 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for u128 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for usize {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for bool {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for f32 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl VarToSql for f64 {
    #[inline]
    fn sql(&self) -> String {
        self.to_string()
    }
}
