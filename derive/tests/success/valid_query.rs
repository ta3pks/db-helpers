use db_helpers::{Table, Q};
#[derive(Debug, Table)]
#[table(name = "foos")]
struct Foo
{
	#[table(name = "bar", q = "int not null")]
	_bar: u32,
	#[table(q = "int not null")]
	other: u32,
}
#[derive(Debug, Table)]
#[table(name = "bars")]
struct Bar
{
	#[table(name = "bar", q = "int not null")]
	_bar: u32,
	#[table(q = "int not null")]
	other: u32,
}
fn main()
{
	//raw string literals should work
	assert_eq!(
		Q!(r#"select Foo::{_bar} from Foo::__TABLE__"#),
		r#"select "bar" from foos"#
	);
	assert_eq!(
		Q!(r"select Foo::{_bar} from Foo::__TABLE__"),
		r#"select "bar" from foos"#
	);
	assert_eq!(
		Q!(r#"select
 Foo::{_bar} from Foo::__TABLE__"#)
		.replace("\n", ""),
		r#"select "bar" from foos"#
	);
	assert_eq!(
		Q!("select Foo::{_bar} from Foo::__TABLE__"),
		r#"select "bar" from foos"#
	);
	//more fields
	assert_eq!(
		Q!("select Foo::{_bar,other} from Foo::__TABLE__"),
		r#"select "bar", "other" from foos"#
	);
	assert_eq!(
		Q!("select Foo::{_bar, other} from Foo::__TABLE__"),
		r#"select "bar", "other" from foos"#
	);
	//more spaces
	assert_eq!(
		Q!("select Foo::{ _bar   , other} from Foo::__TABLE__"),
		r#"select "bar", "other" from foos"#
	);
	//with_table_name
	assert_eq!(
		Q!("select Foo::{ >_bar   , other} from Foo::__TABLE__"),
		r#"select foos.bar, "other" from foos"#
	);
	//db cast should work as expected
	assert_eq!(
		//mind the spaces between openning and closing parentheses
		Q!("insert into Foo::__TABLE__ ( Foo::{_bar,other} ) values ($1, $2::int)"),
		r#"insert into foos ( "bar", "other" ) values ($1, $2::int)"#
	);
}
