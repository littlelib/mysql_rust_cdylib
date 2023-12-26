# mysql_rust_cdylib
C ABI for Rust crate mysql

## Cautions
- If your query(or prepared statement) is something that will give you multiple result sets, don't expect them to run sequentially. In fact, it's only recommended to run only the queries that are independent to each other if you have to run them in a single transaction. Otherwise, it may not give you the result you expected.