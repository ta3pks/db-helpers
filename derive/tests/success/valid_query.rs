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
	assert_eq!(
		Q!("select Foo::{_bar} from Foo::__TABLE__"),
		"select foos.bar from foos"
	);
	//more fields fields
	assert_eq!(
		Q!("select Foo::{_bar,other} from Foo::__TABLE__"),
		"select foos.bar , foos.other from foos"
	);
	//spaces
	assert_eq!(
		Q!("select Foo::{_bar, other} from Foo::__TABLE__"),
		"select foos.bar , foos.other from foos"
	);
	//more spaces
	assert_eq!(
		Q!("select Foo::{ _bar   , other} from Foo::__TABLE__"),
		"select foos.bar , foos.other from foos"
	);
	//db cast should work as expected
	assert_eq!(
		//mind the spaces between openning and closing parentheses
		Q!("insert into Foo::__TABLE__ ( Foo::{_bar,other} ) values ($1, $2::int)"),
		"insert into foos ( foos.bar , foos.other ) values ($1, $2::int)"
	);
}
