use crate::services::queryBuilder::{
    get_select_query, Column, Expression, ExpressionConnector, Item,
};
use axum::http::StatusCode;
use chrono::{DateTime, NaiveDate, Utc};

use dotenvy_macro::dotenv;
use simple_collection_macros::bmap;
use surrealdb::sql::{
    statements::CreateStatement, Data, Datetime, Field, Fields, Ident, Idiom, Object, Output, Part,
    Strand, Table, Thing, Value, Values,
};

fn generate_embed_link(address: Option<String>, api_key: &str) -> String {
    format!(
        "https://www.google.com/maps/embed/v1/place?key={}&q={}",
        api_key,
        address.unwrap()
    )
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct Profile {
    id: Option<Thing>,
    name: Option<String>,
    intro: Option<String>,
    profile_pic: Option<String>,
    contact: Option<String>,
    // optional params depending on user
    date_of_birth: Option<String>,
    address: Option<String>,

    // mutable gmap
    // gmap: Option<String>,
    gmap: Option<String>,
}

impl Profile {
    // returns a new profile model
    pub fn from(
        id: Option<Thing>,
        name: Option<String>,
        intro: Option<String>,
        profile_pic: Option<String>,
        date_of_birth: Option<String>,
        address: Option<String>,
        contact: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            intro,
            profile_pic,
            date_of_birth,
            address,
            contact,
            gmap: None,
        }
    }


    pub async fn get_profile_create_query(self) -> Result<CreateStatement, StatusCode> {
        // println!("{:?}",self);
        match self.id.clone() {
            None => Err(StatusCode::BAD_REQUEST)?,
            _ => {}
        }

        let mut map = bmap! {
            "id".to_string() => Value::Thing(self.id.unwrap()),
            "name".to_string() => Value::Strand(Strand(self.name.unwrap())),
            "intro".to_string() => Value::Strand(Strand(self.intro.unwrap())),
            "profile_pic".to_string() => Value::Strand(Strand(self.profile_pic.unwrap())),
            "contact".to_string() => Value::Strand(Strand(self.contact.unwrap())),
        };

        if let Some(date_of_birth) = self.date_of_birth.clone() {
            map.insert(
                "date_of_birth".to_string(),
                Value::Datetime(Datetime(DateTime::<Utc>::from_utc(
                    NaiveDate::parse_from_str(&date_of_birth, "%d-%m-%Y")
                        .expect("Invalid date format")
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    Utc,
                ))),
            );
        } 

        if let Some(address) = self.address.clone() {
            let api_key = dotenv!("MAP_API_KEY");
            let embed_link = generate_embed_link(Some(address.clone()), api_key).replace(" ", "%20");
         
            // add address to profile
            map.insert("address".to_string(), Value::Strand(Strand(address)));

            // println!("Embed link: {}", embed_link);
            map.insert("gmap".to_string(), Value::Strand(Strand(embed_link)));
        }

        Ok(CreateStatement {
            what: Values(vec![Value::Table(Table("profile".to_string()))]),
            data: Some(Data::ContentExpression(Value::Object(Object(map)))),
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

    pub async fn get_profile_by_user_id(user_id: Option<String>) -> Result<String, StatusCode> {
        let profile = get_select_query(
            Item::Table("profile".to_string()),
            Column::All,
            Some(vec![(
                Expression::EqualTo("id".to_string(), user_id.unwrap()),
                ExpressionConnector::End,
            )]),
            None,
            None,
            None,
            None,
        );

        // println!("{:?}", profile);
        Ok(profile)
    }
}


impl Into<serde_json::Value> for Profile {
    fn into(self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "intro": self.intro,
            "profile_pic": self.profile_pic,
            "contact": self.contact,
            "date_of_birth": self.date_of_birth,
            "address": self.address,
            "gmap": self.gmap
        })
    }
}