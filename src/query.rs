#[macro_export]
macro_rules! query {
	($tbl:ty;$q:expr$(,$param:expr)*) => {
		format!($q, $($param,)*table = <$tbl>::table_name()).as_str()
	};
}

#[macro_export]
macro_rules! params {
	($($param:expr),*) => {
		&[$(&$param),*]
	};
}
