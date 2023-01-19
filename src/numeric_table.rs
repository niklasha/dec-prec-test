table! {
    numeric_table (id) {
        id -> Integer,
        numeric_column -> Numeric,
    }
}

#[derive(Queryable)]
pub struct NumericTable {
    pub id: i32,
    pub numeric_column: BigDecimal,
}
