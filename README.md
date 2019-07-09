# About
This is just a quick macro to minimize the amount of maps you do with [rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple/) (shoutout to an amazing library)

from this:
```rust
    let result = connection.prep_exec(
        "SELECT id, email, password FROM users WHERE email = :email",
        mysql::params! {
            "email" => username
        }
    ).map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
               let (id, email, password) = mysql::from_row(row);
                User {
                    id,
                    email,
                    password
                }
            })
            .collect()
    });
```
to this:
```rust
    let result = crate::mysql_query!(connection,
        "SELECT id, email, password FROM users WHERE email = :email",
        mysql::params!(
            "email" => username
        ),
        |(id, email, password)| {
            User {
                id,
                email,
                password
            }
        }
    );
```
![Image of code you will be able to reduce from](https://i.imgur.com/vung9o7.png)