use sqler::XssString;
use sqler_macros::sql;

#[test]
fn sql_only() {
    assert_eq!(sql! {SELECT * FROM users}, "SELECT * FROM users")
}

#[test]
fn sql_with_line_feed() {
    assert_eq!(
        sql! {
            SELECT *
            FROM users
        },
        "SELECT * FROM users"
    )
}

#[test]
fn sql_with_string() {
    assert_eq!(
        sql! {SELECT * FROM users WHERE user_name="ali"},
        String::from("SELECT * FROM users WHERE user_name = 'ali'")
    )
}

#[test]
fn sql_with_escaped_string() {
    assert_eq!(
        sql! {SELECT * FROM users WHERE user_name="al\"i"},
        String::from("SELECT * FROM users WHERE user_name = 'al\"i'")
    )
}
/*
#[test]
fn sql_with_raw_string() {
    assert_eq!(
        sql! {SELECT * FROM users WHERE user_name=r#"al"i"#},
        String::from("SELECT * FROM users WHERE user_name = 'al\"i'")
    )
}
 */

#[test]
fn sql_with_vars() {
    let user_name = String::from("ali");
    let age = 24;
    assert_eq!(
        sql! {SELECT * FROM users WHERE user_name={user_name} AND age = {age}},
        String::from("SELECT * FROM users WHERE user_name = 'ali' AND age = 24")
    )
}

#[test]
fn sql_with_repeated_vars() {
    let name = String::from("ahmed");
    assert_eq!(
        sql! {SELECT * FROM users WHERE first_name={ name } OR last_name= {name}},
        String::from("SELECT * FROM users WHERE first_name = 'ahmed' OR last_name = 'ahmed'")
    )
}

#[test]
fn sql_with_prefixed_nums() {
    assert_eq!(
        sql! {SELECT * FROM users WHERE age = 0x1f OR age = 0o77 OR age = 0b1011},
        String::from("SELECT * FROM users WHERE age = 31 OR age = 63 OR age = 11")
    )
}

#[test]
fn sql_with_suffixed_nums() {
    assert_eq!(
        sql! {SELECT * FROM users WHERE age = 10i32 OR age = 15isize OR age = 30u64},
        String::from("SELECT * FROM users WHERE age = 10 OR age = 15 OR age = 30")
    )
}

#[test]
fn sql_with_nums_with_under_score() {
    assert_eq!(
        sql! {SELECT * FROM users WHERE age = 2_5 OR age = 3_5 OR age = 10_0},
        String::from("SELECT * FROM users WHERE age = 25 OR age = 35 OR age = 100")
    )
}

#[test]
fn sql_with_prefixed_suffixed_nums_with_under_score() {
    assert_eq!(
        sql! {SELECT * FROM users WHERE age = 0x1fi16 OR age = 0o77i8 OR age = 0b1011u8 OR age = 15_2i128},
        String::from("SELECT * FROM users WHERE age = 31 OR age = 63 OR age = 11 OR age = 152")
    )
}

#[test]
fn sql_with_xss_string() {
    let first_name = XssString(String::from("&ahmed <h1>"));
    assert_eq!(
        sql! {SELECT * FROM users WHERE first_name = {first_name}},
        String::from("SELECT * FROM users WHERE first_name = '&amp;ahmed &lt;h1&gt;'")
    )
}

#[test]
fn sql_insert_with_array() {
    let per1 = String::from("create user");
    let per2 = String::from("edit user");
    let user_name = String::from("sqler");

    assert_eq!(sql!(
        INSERT INTO users
            (user_name, permissions)
        VALUES
            ({user_name}, ARRAY[{per1}, {per2}, "delete user"]::TEXT [])
    ),
    "INSERT INTO users (user_name, permissions) VALUES ('sqler', ARRAY ['create user', 'edit user', 'delete user']::TEXT [])"
);
}

#[test]
fn sql_update() {
    let last_name = String::from("Ahmed");
    let age = 15;
    let user_id: isize = 120;
    assert_eq!(
        sql!(
            UPDATE users SET user_name="Ali", last_name={last_name}, age = {age} WHERE user_id={user_id}
        ),
        "UPDATE users SET user_name = 'Ali', last_name = 'Ahmed', age = 15 WHERE user_id = 120"
    )
}

#[test]
fn sql_join() {
    assert_eq!(
        sql!(
            SELECT *
            FROM users AS u

            LEFT JOIN employees AS e
            ON e.emp_id=u.user_id
        ),
        "SELECT * FROM users AS u LEFT JOIN employees AS e ON e.emp_id = u.user_id"
    );
}
