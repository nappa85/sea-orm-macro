
#[sea_orm_macro::table(table_name = "foo", primary_key = "Id", relations_enum = "Bar")]
struct Foo {
    id: u64,
    name: Option<String>,
}

#[test]
fn it_works() {
    
}
