
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
#[derive(sea_orm_macro::AutoColumn)]
#[derive(serde::Serialize)]
#[auto_column(table_name = "foo")]
#[serde(rename = "lower_case")]
pub struct Model {
    #[auto_column(primary_key)]
    id: u64,
    #[auto_column(type = "String(Some(255))", nullable)]
    name: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        unreachable!()
    }
}

#[test]
fn it_works() {
    use sea_orm::Iterable;
    assert_eq!("[Id, Name]", format!("{:?}", Column::iter().collect::<Vec<_>>()));
}
