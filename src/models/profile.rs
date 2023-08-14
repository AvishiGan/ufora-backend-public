use axum::http::StatusCode;
use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime};
use simple_collection_macros::bmap;
use surrealdb::sql::{Thing, statements::CreateStatement, Values, Value, Table, Data, Object, Strand, Fields, Output, Field, Idiom, Part, Ident, Datetime};

#[derive(serde::Serialize,serde::Deserialize,Default,Debug)]

pub struct Profile{
    id: Option<Thing>,
    intro: Option<String>,
    profile_pic: Option<String>,
    contact: Option<String>,
    // optional params depending on user
    date_of_birth: Option<String>,
    address: Option<String>,
    map: Option<String>,
}

impl Profile {

    // returns a new profile model
    pub fn from(
        intro: Option<String>,
        profile_pic: Option<String>,
        date_of_birth: Option<String>,
        address: Option<String>,
        contact: Option<String>,
        map: Option<String>,
    ) -> Self {
        Self {
            id: None,
            intro,
            profile_pic,
            date_of_birth,
            address,
            contact,
            map,
        }
    }
    
    // pub async fn get_register_query(
    //     self
    //  ) -> Result<CreateStatement,StatusCode> {

    //     match self.name.clone() {
    //         None => Err(StatusCode::BAD_REQUEST) ?,
    //         _ => {}
    //     }

    //     Ok(CreateStatement {
    //         what: Values(
    //             vec![Value::Table(Table("undergraduate".to_string()))]
    //         ),
    //         data: Some(Data::ContentExpression(Value::Object( Object (bmap! {
    //             "name".to_string() => Value::Strand(Strand(self.name.unwrap())),
    //         })))),
    //         output: Some(
    //             Output::Fields(
    //                 Fields(vec![
    //                     Field::Alone(Value::Idiom(Idiom(vec![Part::Field(Ident("id".to_string()))])))
    //                     ],true))
    //         ),
    //         timeout: None,
    //         parallel: false,
    //     })

    // }


    pub async fn get_profile_create_query(
        self
     ) -> Result<CreateStatement,StatusCode> {

        match self.id.clone() {
            None => Err(StatusCode::BAD_REQUEST) ?,
            _ => {}
        }

        Ok(CreateStatement {
            what: Values(
                vec![Value::Table(Table("profile".to_string()))]
            ),
            data: Some(Data::ContentExpression(Value::Object( Object (bmap! {
                // "id".to_string() => Value::Strand(Strand(self.id.unwrap())),
                "intro".to_string() => Value::Strand(Strand(self.intro.unwrap())),
                "profile_pic".to_string() => Value::Strand(Strand(self.profile_pic.unwrap())),
                "date_of_birth".to_string() => Value::Datetime(Datetime(DateTime::<Utc>::from_utc(NaiveDate::parse_from_str(&self.date_of_birth.unwrap(), "%d-%m-%Y").expect("Invalid date format").and_hms(0, 0, 0),Utc))),
                "address".to_string() => Value::Strand(Strand(self.address.unwrap())),
                "contact".to_string() => Value::Strand(Strand(self.contact.unwrap())),
                "map".to_string() => Value::Strand(Strand(self.map.unwrap())),
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