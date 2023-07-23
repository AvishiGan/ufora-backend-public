use simple_collection_macros::bmap;

use axum::http::StatusCode;
use::surrealdb::sql::{Thing,Table,Object,Value,Part,Fields,Field,Ident,Idiom,Output,Data,Values,Strand,
    statements::CreateStatement
};

#[derive(serde::Deserialize,Debug)]
pub struct Undergraduate {
    id: Option<Thing>,
    name: Option<String>,
}

impl Undergraduate {
    pub fn from(name: Option<String>) -> Self {
        Self {
            id: None,
            name,
        }
    }

    pub async fn get_register_query(
        self
     ) -> Result<CreateStatement,StatusCode> {

        match self.name.clone() {
            None => Err(StatusCode::BAD_REQUEST) ?,
            _ => {}
        }

        Ok(CreateStatement {
            what: Values(
                vec![Value::Table(Table("undergraduate".to_string()))]
            ),
            data: Some(Data::ContentExpression(Value::Object( Object (bmap! {
                "name".to_string() => Value::Strand(Strand(self.name.unwrap())),
            })))),
            output: Some(
                Output::Fields(
                    Fields(vec![
                        Field::Alone(Value::Idiom(Idiom(vec![Part::Field(Ident("id".to_string()))])))
                        ],true))
            ),
            timeout: None,
            parallel: false,
        })

    }
}