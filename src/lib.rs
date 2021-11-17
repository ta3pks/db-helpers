pub mod query;
#[cfg(feature = "tokio_postgres")]
pub use tokio_postgres::Row;
pub trait Table
{
	fn table_name() -> String;
}
