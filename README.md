# sqler - alpha

A procedural macro that helps with writing SQL queries using some of Rust syntax

## Usage

First, in your Cargo.toml file add the following:

```toml
[dependencies]
sqler = "0.0.1-alpha"
```

__Example 1__

To just embedding a value of a varible you can just do the following:

```rust
use sqler::sql;

fn main() {
    let first_name = String::from("Ali");
    let age = 24;
    let query = sql!(
        SELECT * FROM users
        WHERE first_name = {first_name}
        OR age = {age}
    );

    assert_eq!(
        query,
        "SELECT * FROM users WHERE first_name = 'Ali' OR age = 24"
    );
}
```

This macro will handle the process of embedding value of any variable (only built-in types) in addition to converting the Rust syntax to string that contains SQL statement.

__Example 2__

Also you can write the value directly:


```rust
use sqler::sql;

fn main() {
    let query = sql!(
        SELECT * FROM users
        WHERE first_name = "Ali"
        OR age = 24
    );

    assert_eq!(
        query,
        "SELECT * FROM users WHERE first_name = 'Ali' OR age = 24"
    );
}
```

__Example 3__

You can also use hexadecimal, octal, or binary number format and the macro will handle it by converting the value back to decimal.

```rust
use sqler::sql;

fn main() {
    let query = sql!(
        UPDATE employees 
        SET 
            age=0x1f,
            salery=0o5776
        WHERE
            emp_id=0b101
    );

    assert_eq!(
        query,
        "UPDATE employees SET age = 31, salery = 3070 WHERE emp_id = 5"
    );
}
```

__Example 4__

What about your custom types? you can embed your custom types by implementing the "VarToSql" trait as follows:

```rust
use sqler::{sql, VarToSql};

struct IntArr(Vec<i32>);

impl VarToSql for IntArr {
    fn sql(&self) -> String {
        let mut sql = String::from("ARRAY[");

        for i in 0..self.0.len() - 1 {
            sql.push_str(&self.0[i].to_string());
            sql.push_str(", ");
        }

        sql.push_str(&self.0[self.0.len() - 1].to_string());
        sql.push_str("]::INT[]");
        sql
    }
}

fn main() {
    let permissions = IntArr(vec![1, 2, 3]);
    let query = sql!(
        INSERT INTO user_permissions
            (user_id, permissions)
        VALUES
            (1, {permissions})
    );

    assert_eq!(
        query,
        "INSERT INTO user_permissions (user_id, permissions) VALUES (1, ARRAY[1, 2, 3]::INT[])"
    );
}
```