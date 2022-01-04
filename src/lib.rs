pub mod query;
pub use tokio_postgres::Row;
pub trait Table
{
	fn table_name() -> String;
}
pub trait Index
{
	fn index() -> Option<String>
	{
		None
	}
}

impl<T> Index for T where T: Table {}
