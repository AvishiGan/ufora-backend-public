use std::sync::Arc;

use chrono::{DateTime, NaiveDate, Utc};
use simple_collection_macros::bmap;

use ::surrealdb::sql::{
    statements::CreateStatement, Data, Field, Fields, Ident, Idiom, Object, Output, Part, Strand,
    Table, Thing, Value, Values,
};
use axum::http::StatusCode;
use surrealdb::{
    engine::remote::ws::Client,
    opt::PatchOp,
    sql::{statements::UpdateStatement, Cond, Datetime, Expression, Operator},
    Surreal,
};


#[derive(serde::Deserialize, Debug)]
pub struct Undergraduate {
    id: Option<Thing>,
    name: Option<String>,
    date_of_birth: Option<String>,
    university: Option<String>,
    university_email: Option<String>,
    university_email_verification_flag: Option<bool>,
}

impl Undergraduate {
    pub fn from(name: Option<String>, date_of_birth: Option<String>) -> Self {
        Self {
            id: None,
            name,
            date_of_birth,
            university: None,
            university_email: None,
            university_email_verification_flag: None,
        }
    }

    pub async fn get_register_query(self) -> Result<CreateStatement, StatusCode> {
        match self.name.clone() {
            None => Err(StatusCode::BAD_REQUEST)?,
            _ => {}
        }

        Ok(CreateStatement {
            what: Values(vec![Value::Table(Table("undergraduate".to_string()))]),
            data: Some(Data::ContentExpression(Value::Object(Object(bmap! {
                "name".to_string() => Value::Strand(Strand(self.name.unwrap())),
            })))),
            output: Some(Output::Fields(Fields(
                vec![Field::Alone(Value::Idiom(Idiom(vec![Part::Field(Ident(
                    "id".to_string(),
                ))])))],
                true,
            ))),
            timeout: None,
            parallel: false,
        })
    }

    pub async fn update_university_email_verification(
        db: Arc<Surreal<Client>>,
        email: String,
    ) -> Result<(), StatusCode> {
        let _response = db
            .query(UpdateStatement {
                what: Values(vec![Value::Table(Table("user".to_string()))]),
                data: Some(Data::SetExpression(vec![(
                    Idiom(vec![Part::Field(Ident(
                        "university_email_verification_flag".to_string(),
                    ))]),
                    Operator::Equal,
                    Value::True,
                )])),
                cond: Some(Cond(Value::Expression(Box::from(Expression {
                    l: Value::Idiom(Idiom(vec![Part::Field(Ident(
                        "university_email".to_string(),
                    ))])),
                    o: surrealdb::sql::Operator::Equal,
                    r: Value::Strand(Strand(email)),
                })))),
                output: None,
                timeout: None,
                parallel: false,
            })
            .await;

        Ok(())
    }

    pub async fn update_university_details(
        user_id: Thing,
        db: Arc<Surreal<Client>>,
        university: Option<String>,
        university_email: Option<String>,
    ) -> Result<(), StatusCode> {
        match (university.clone(), university_email.clone()) {
            (None, _) | (_, None) => return Err(StatusCode::BAD_REQUEST),
            (_, _) => {}
        }

        let response: Result<String, surrealdb::Error> = db
            .update(("undergraduate", user_id.id))
            .patch(PatchOp::replace("/university", university.unwrap()))
            .patch(PatchOp::replace(
                "/university_email",
                university_email.unwrap(),
            ))
            .patch(PatchOp::replace(
                "/university_email_verification_flag",
                false,
            ))
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?} ", e);
                Ok(())
            }
        }
    }
 

}

pub async fn update_dob(
    user_id: String,
    db: Arc<Surreal<Client>>,
    date_of_birth: Option<String>,
) -> Result<(), StatusCode> {
    match date_of_birth {
        None => return Err(StatusCode::BAD_REQUEST),
        _ => {}
    }

    let response: Result<String, surrealdb::Error> = db
        .update(("undergraduate", user_id))
        // parse dob as datetime
        // .patch(PatchOp::replace("/date_of_birth",date_of_birth.unwrap()))
        .patch(PatchOp::replace(
            "/date_of_birth",
            Datetime(DateTime::<Utc>::from_utc(
                NaiveDate::parse_from_str(&date_of_birth.unwrap(), "%d-%m-%Y")
                    .expect("Invalid date format")
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                Utc,
            )),
        ))
        .await;

    match response {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{:?} ", e);
            Ok(())
        }
    }
}



impl Into<serde_json::Value> for Undergraduate {
    fn into(self) -> serde_json::Value {
        serde_json::json!({
            "id":self.id,
            "name":self.name,
            "date_of_birth":self.date_of_birth,
            "university":self.university,
            "university_email":self.university_email,
            "university_email_verification_flag":self.university_email_verification_flag
        })
    }
}
