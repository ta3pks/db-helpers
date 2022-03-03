# 1.0.0
- Table is now a derive macro check Readme for how to use
- Old `Table` and `Index` traits are moved under `legacy` module to enable progressive upgrades from 0x versions to 1 with minimal code changes
- `query` macro is deprecated use `query_unchecked` macro for the same functionality
- there's a brand new `Q` macro for field name checked query's
# 0.11.1
- params macro is moved behind pg feature and renamed to pg_params
# 0.11.0
- postgres specific `From<Row>` impl is behind `pg` feature
- remove __new method
- rename create_table_str to pg_create_table_str and put behind bg feature


