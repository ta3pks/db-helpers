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

#[macro_export]
macro_rules! table {
	(
		$(#[$($meta:meta),*])?
		$pub:vis $name:ident
		{
			$(
				$(#[$($inner_meta:meta),*])?
				$rust_key:ident $rust_type:ty : $db_name:tt $db_type:expr
			 ),+
		}
	) => {
		$(#[$($meta),*])?
		$pub struct $name {
		$(
			$(#[$($inner_meta),*])?
			$rust_key:$rust_type
		),+
		}
		impl $name{
			pub fn __new($($rust_key:$rust_type),+) -> $name{
				Self{
					$($rust_key),+
				}
			}
			pub fn to_create_table_str()->String{
				let fields = [
					$(concat!("\"",stringify!($db_name),"\""," ",$db_type)),+
				];
				format!("CREATE TABLE IF NOT EXISTS {} ({});",stringify!($name).to_lowercase(),fields.join(","))
			}
		}
		impl $crate::Table for $name{
			fn table_name()->String{
				stringify!($name).to_lowercase()
			}
		}
		impl From<&$crate::Row> for $name{
			fn from(r:&$crate::Row)->Self{
				Self{
					$($rust_key:r.get(stringify!($db_name))),+
				}
			}
		}
		impl From<$crate::Row> for $name{
			fn from(r:$crate::Row)->Self{
				Self::from(&r)
			}
		}
	};
}
