#[macro_export]
macro_rules! mysql_query {
    ($connection:ident, $query:expr, $param:expr, |$all_values:tt| $block:tt) => {
        $connection.prep_exec($query, $param).map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let $all_values = mysql::from_row(row);
                    $block
                }).collect()
        });
    };
}