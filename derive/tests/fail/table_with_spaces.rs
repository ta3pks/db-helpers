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
	//invalid space
	Q!("select Foo ::{_bar} from Foo::__TABLE__");
}
