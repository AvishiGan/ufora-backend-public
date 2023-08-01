use std::sync::Arc;

use simple_collection_macros::bmap;

use axum::http::StatusCode;
use surrealdb::{Surreal, engine::remote::ws::Client, sql::{statements::UpdateStatement, Operator, Cond, Expression}, opt::PatchOp};
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

    pub async fn update_university_email_verification(
        db:Arc<Surreal<Client>>,
        email: String
    ) -> Result<(),StatusCode> {

        let _response = db.query(
            UpdateStatement {
                what: Values(
                    vec![Value::Table(Table("user".to_string()))]
                ),
                data: Some(Data::SetExpression(
                    vec![
                        (
                            Idiom(vec![Part::Field(Ident("university_email_verification_flag".to_string()))]),
                            Operator::Equal,
                            Value::True
                        )
                    ]
                )),
                cond: Some(Cond(
                    Value::Expression(Box::from(Expression {
                        l: Value::Idiom(Idiom(vec![Part::Field(Ident("university_email".to_string()))])),
                        o: surrealdb::sql::Operator::Equal,
                        r: Value::Strand(Strand(email))
                    })))
                ),
                output: None,
                timeout: None,
                parallel: false
            }
        ).await;

        Ok(())
    }

    pub async fn update_university_details(
        user_id: Thing,
        db:Arc<Surreal<Client>>,
        university: Option<String>,
        university_email: Option<String>
    ) -> Result<(),StatusCode> {

        match (university.clone(),university_email.clone()) {
            (None,_) | (_,None) => {return Err(StatusCode::BAD_REQUEST)},
            (_,_) => {}
        }

        let response: Result<String,surrealdb::Error> = db.update(("undergraduate",user_id.id))
            .patch(PatchOp::replace("/university",university.unwrap()))
            .patch(PatchOp::replace("/university_email",university_email.unwrap()))
            .patch(PatchOp::replace("/university_email_verification_flag",false))
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(e) => {println!("{:?} ",e);Ok(())}
        }
    }
}