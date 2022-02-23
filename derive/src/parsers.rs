use syn::{Data, DataStruct, FieldsNamed};

pub fn parse_root(i: &Data) -> (&DataStruct, &FieldsNamed)
{
	match i {
		syn::Data::Struct(d) => match d.fields {
			syn::Fields::Named(ref f) => {
				if f.named.is_empty() {
					panic!("struct must have at least one named field");
				}
				(d, f)
			}
			_ => panic!("Only named struct fields are supported"),
		},
		_ => panic!("#[derive(Tbl)] can only be used on structs"),
	}
}
