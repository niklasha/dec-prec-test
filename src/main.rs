mod create_numeric_table;
mod numeric_table;

extern crate diesel;
extern crate bigdecimal;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use bigdecimal::BigDecimal;

fn main() {
    let connection = PgConnection::establish("postgres://username:password@localhost/dbname").unwrap();

    let test_values = vec![
        BigDecimal::new(1.234),
        BigDecimal::new(1.235),
        BigDecimal::new(1.236),
        BigDecimal::new(1.237),
        BigDecimal::new(1.238),
        BigDecimal::new(1.239),
    ];

    for value in test_values {
        let rounded_value: BigDecimal = diesel::insert_into(numeric_table::table)
            .values(numeric_column.eq(value))
            .returning(numeric_column)
            .get_result(&connection)
            .unwrap();
        println!("Original value: {} Rounded value: {}", value, rounded_value);
    }
}
