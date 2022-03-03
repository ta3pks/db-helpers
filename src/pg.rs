#[cfg(feature = "pg")]
#[macro_export]
macro_rules! pg_fns {
	(from_table $name:ident $($rust_name:ident,$db_name:tt);+) => {
		impl From<&$crate::Row> for $name{
			fn from(r:&$crate::Row)->Self{
				Self{
					$($rust_name:r.get(stringify!($db_name))),+
				}
			}
		}
		impl From<$crate::Row> for $name{
			fn from(r:$crate::Row)->Self{
				Self::from(&r)
			}
		}
  };
	(create_table_str $name:ident $($db_name:tt,$db_type:expr);+) => {
			pub fn to_pg_create_table_str()->String{
				let fields = [
					$(concat!("\"",stringify!($db_name),"\""," ",$db_type)),+
				];
				format!("CREATE TABLE IF NOT EXISTS {} ({});",<Self as $crate::legacy::Table>::table_name(),fields.join(","))
			}
  };
}
/// add here the default macros for no backend enabled
#[cfg(not(feature = "pg"))]
#[macro_export]
macro_rules! pg_fns {
	(from_table $name:ident $($rust_name:ident,$db_name:tt);+) => {};
	(create_table_str $name:ident $($db_name:tt,$db_type:expr);+) => {};
}
#[cfg(feature = "pg")]
#[macro_export]
macro_rules! pg_params {
	($($param:expr),*) => {
		&[$(&$param),*]
	};
}
