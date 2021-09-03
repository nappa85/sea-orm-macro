# sea-orm AutoColumn derive macro

## Basic Usage

Use it like a derive macro
```rust
use sea_orm_macro::AutoColumn;

#[derive(AutoColumn)]
struct Model {
    id: u64,
    name: String,
}
```
this will autogenerate the Column enum, like this
```rust
#[derive(Copy, Clone, Debug, sea_orm::prelude::EnumIter, sea_orm::prelude::DeriveColumn)]
enum Column {
    Id,
    Name,
}

impl sea_orm::prelude::ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> sea_orm::prelude::ColumnDef {
        match self {
            Column::Id => sea_orm::prelude::ColumnType::Integer.def(),
            Column::Name => sea_orm::prelude::ColumnType::String(None).def(),
        }
    }
}
```

## Primary key

To autogenerate the PrimaryKey enum too, simply add the attribute like this
```rust
use sea_orm_macro::AutoColumn;

#[derive(AutoColumn)]
struct Model {
    #[auto_column(primary_key)]
    id: u64,
    name: String,
}
```
this will autogenerate the PrimaryKey enum, like this
```rust
#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    fn auto_increment() -> bool {
        true
    }
}
```
if you have more than one primary key, the auto_increment methos will automatically turn to false
```rust
use sea_orm_macro::AutoColumn;

#[derive(AutoColumn)]
struct Model {
    #[auto_column(primary_key)]
    id1: u64,
    #[auto_column(primary_key)]
    id2: u64,
}
```
this will autogenerate the PrimaryKey enum, like this
```rust
#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id1,
    Id2,
}

impl PrimaryKeyTrait for PrimaryKey {
    fn auto_increment() -> bool {
        false
    }
}
```

## Custom types

If your struct uses a non-basic type, you'll have to specify the SQL type for that column:
```rust
use sea_orm_macro::AutoColumn;

#[derive(AutoColumn)]
struct Model {
    #[auto_column(primary_key)]
    id: u64,
    #[auto_column(type = "Char(Some(1))", nullable)]
    name: MyType,
}
```
this will autogenerate the Column enum, like this
```rust
#[derive(Copy, Clone, Debug, sea_orm::prelude::EnumIter, sea_orm::prelude::DeriveColumn)]
enum Column {
    Id,
    Name,
}

impl sea_orm::prelude::ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> sea_orm::prelude::ColumnDef {
        match self {
            Column::Id => sea_orm::prelude::ColumnType::Integer.def(),
            Column::Name => sea_orm::prelude::ColumnType::Char(Some(1)).def(),
        }
    }
}
```

You can also use it to override the default typing:
```rust
use sea_orm_macro::AutoColumn;

#[derive(AutoColumn)]
struct Model {
    #[auto_column(primary_key)]
    id: u64,
    #[auto_column(type = "String(Some(255))", nullable)]
    name: String,
}
```
this will autogenerate the Column enum, like this
```rust
#[derive(Copy, Clone, Debug, sea_orm::prelude::EnumIter, sea_orm::prelude::DeriveColumn)]
enum Column {
    Id,
    Name,
}

impl sea_orm::prelude::ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> sea_orm::prelude::ColumnDef {
        match self {
            Column::Id => sea_orm::prelude::ColumnType::Integer.def(),
            Column::Name => sea_orm::prelude::ColumnType::String(Some(255)).def(),
        }
    }
}
```

### Default type mapping

The default type mapping is:

| Rust | SQL |
|-----|-----|
| char | Char(None) |
| String | String(None) |
| &str | String(None) |
| u8" | TinyInteger |
| i8 | TinyInteger |
| u16 | SmallInteger |
| i16 | SmallInteger |
| u32 | Integer |
| u64 | Integer |
| i32 | Integer |
| i64 | Integer |
| u128 | BigInteger |
| i128 | BigInteger |
| f32 | Float |
| f64 | Double |
| bool | Boolean |
| NaiveDate | Date |
| NaiveTime | Time |
| NaiveDateTime | DateTime |
| Uuid | Uuid |
| Decimal | BigInteger |
