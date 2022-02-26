# db_helpers

### Usage
```rust
#[derive(Table)]
//index is optional
#[table(name = "foo_db", index = "...")]
struct Foo
{
//if name is not specified lowercased fieldname is used by default 
//q is mandatory
	#[table(name = "id", q = "int primary key not null")]
	_id: i32,
	#[table(name = "name", q = "text")]
	_name: String,
}
```
```rust 
db.batch_execure(
//Available if pg feature is enabled
[Foo::__pg_create_table_str(),Foo::__pg_index()].join(";")
).await;
```
