pub mod query;
pub use tokio_postgres::Row;
pub trait Table
{
	fn table_name() -> String;
}
