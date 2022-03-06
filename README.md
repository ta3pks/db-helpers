# db_helpers
db_helpers provide various helpers to simplify and make safer to interact with databases.  
This is not an orm library the idea is simply to be able to replace inline queries and start getting benefits without learning a new library

### Why did I create db_helpers
- compile time checked field names in queries to make regular sql little bit more secure.
- To reduce boilerplate ( creating From<Row> for every struct namely)

### Features
- Create table creation and indexing queries.(feature flag)
- Reduce boilerplate by implementing certain default such as row to struct conversion(feature flag)
- Compile time field name validation
- Meaningful error messages 

### Usage
```rust
use db_helpers::{Table,Q};
#[derive(Table)]
//index is optional
//__TABLE__ key is automatically replaced with table_name
#[table(name = "users", index = "create unique index if not exists unique_usernames_of_users on __TABLE__ (username)")]
struct User
{
//if name is not specified lowercase fieldname is used by default 
//q is mandatory
	#[table(name = "id", q = "bigserial primary key not null")]
	_id: i64,
	//name of this field is assumed username
	#[table( q = "text")]
	username: String,
}
#[derive(Table)]
//index is optional
#[table(name = "ops")]
struct Op{
#[table(q="bigserial not null primary key")]
id:i64,
#[table(q="bigint not null references")]
user_id:i64,
}
```
```rust 
db.batch_execure(
//Available if pg feature is enabled
[User::__pg_create_table_str(),User::__pg_index()].join(";")
).await;
//unfortunately for the time being `<struct>::{` part cannot contain spaces smarter parsing is in the todo list
let User:User = db.query_one(Q!("select User::{_id,username} from User::__TABLE__"),params!()).await.unwrap();
db.execute(Q!("insert into ( Foo::{username} ) VALUES ($1)"),params!("superman")).await.unwrap();
//you can also use tablename.fieldname format using > in the beginning of the field
//produces `select id , users.username from users`
let User:User = db.query_one(Q!("select User::{_id,>username} from User::__TABLE__"),params!()).await.unwrap();
let ops : Vec<Op> = db
.query(Q!("select * from Op::__TABLE__ where Op::{user_id} = (select Foo::{_id} from Foo::__TABLE__ where Foo::{username} = $1)"),
params!("superman"))
.await.unwrap().iter(Into::into).collect();


//it also supports runtime args using format macro
//if no additional arguments provided Q produces a &'static str otherwise it passes everything to format! macro 
Q!(
		"select Foo::{>username} from Foo::__TABLE__ where Foo::{>_id} in ({})",
		["1", "2", "3"].join(",")
	)
```

### Error Messages

![invalid_struct](https://github.com/NikosEfthias/db-helpers/raw/master/img/invalid_struct.png)
![missing_fields](https://github.com/NikosEfthias/db-helpers/raw/master/img/missing_fields.png)
![no_field](https://github.com/NikosEfthias/db-helpers/raw/master/img/no_field.png)
![missing_close](https://github.com/NikosEfthias/db-helpers/raw/master/img/missing_close.png)

# How to upgrade from 0x releases
Please check [changelog](https://github.com/NikosEfthias/db-helpers/blob/master/Changelog.md) for details


# TODO:
- [x] allow using format macro inside Q
- [ ] *- operator meaning all fields but the defined ones `Users::{*-password}`
- [ ] infer postgres types from rust type where possible making `q` argument optional
- [ ] parse `Q` smarter to allow using spaces in more places as well as no spaces in places like inserts
- [ ] Sqlite backend
