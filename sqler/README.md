# sqler - beta

A procedural macro that helps with writing SQL queries using some of Rust syntax

## Usage
First, in your Cargo.toml file add the following:

```toml
[dependencies]
sqler = "0.0.1-beta"
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
        "SELECT * FROM users WHERE first_name='Ali' OR age=24"
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
        "SELECT * FROM users WHERE first_name='Ali' OR age=24"
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
        "UPDATE employees SET age=31,salery=3070 WHERE emp_id=5"
    );
}
```

__Example 4__

What about variable of a custom type? variable of a custom type can be embedded by first implementing the "VarToSql" trait (which tells the macro how to embed the value of that type) as follows:

```rust
use sqler::{sql, VarToSql};

struct IntArr(Vec<i32>);

impl VarToSql for IntArr {
    #[inline]
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
        "INSERT INTO user_permissions(user_id,permissions) VALUES (1,ARRAY[1, 2, 3]::INT[])"
    );
}
```

## Notes
There are a few points you should consider when using this crate:

- **Delimited Identifier** - or quoted identifier is not supported. For example: `SELECT "first_name"` the column name `first_name` will be converted to a string (string is wrapped with single quote) as follows: `SELECT 'first_name'`.

- **Variables Of Custom Type** - to use variables of a custom type the `VarToSql` trait should be implemented for this type.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>