#[macro_export]
#[deprecated = "use query_unchecked instead"]
macro_rules! query {
	($tbl:ty;$q:expr$(,$param:expr)*) => {
		format!($q, $($param,)*table = <$tbl as $crate::legacy::Table>::table_name()).as_str()
	};
}
#[macro_export]
macro_rules! query_unchecked {
	($tbl:ty;$q:expr$(,$param:expr)*) => {
		format!($q, $($param,)*table = <$tbl as $crate::legacy::Table>::table_name()).as_str()
	};
}
#[macro_export]
macro_rules! table {
	(
		$(#[$($meta:meta),*])?
		$pub:vis $name:ident $([table_name=$tbl_name:expr])?
		{
			$(
				$(#[$($inner_meta:meta),*])?
				$field_pub:vis $rust_key:ident $rust_type:ty : $db_name:tt $db_type:expr
			 ),+
		}
	) => {
		$(#[$($meta),*])?
		$pub struct $name {
		$(
			$(#[$($inner_meta),*])?
			$field_pub $rust_key:$rust_type
		),+
		}
		impl $crate::legacy::Table for $name{
			fn table_name()->String{
				let mut name = stringify!($name).to_lowercase();
				$(name=$tbl_name.to_string();)?
				name
			}
		}
		impl $name{
        pub fn is_valid_column_name(name:&str)->bool{
            match name{
                $(
                    stringify!($db_name) => true,
                )+
                _ => false
            }
        }
        $crate::pg_fns!(create_table_str $name $($db_name,$db_type);+);
		}
    $crate::pg_fns!(from_table $name $($rust_key,$db_name);+);
	};
}
