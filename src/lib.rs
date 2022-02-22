pub mod pg;
pub mod query;
#[cfg(feature = "pg")]
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
