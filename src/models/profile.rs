use crate::services::queryBuilder::{
    get_select_query, Column, Expression, ExpressionConnector, Item,
};
use axum::http::StatusCode;

use simple_collection_macros::bmap;
use surrealdb::sql::{
    statements::CreateStatement, Data, Field, Fields, Ident, Idiom, Object, Output, Part,
    Strand, Table, Thing, Value, Values,
};
 

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct Profile {
    id: Option<Thing>,
    intro: Option<String>,
    profile_pic: Option<String>,
    contact: Option<String>,
    // optional params depending on user
    
}

impl Profile {
    // returns a new profile model
    pub fn from(
        id: Option<Thing>,
        intro: Option<String>,
        profile_pic: Option<String>,
        contact: Option<String>,
    ) -> Self {
        Self {
            id,
            intro,
            profile_pic,
            contact,
        }
    }


    pub async fn get_profile_create_query(self) -> Result<CreateStatement, StatusCode> {
        // println!("{:?}",self);
        match self.id.clone() {
            None => Err(StatusCode::BAD_REQUEST)?,
            _ => {}
        }

        let map = bmap! {
            "id".to_string() => Value::Thing(self.id.unwrap()),
            "intro".to_string() => Value::Strand(Strand(self.intro.unwrap())),
            "profile_pic".to_string() => Value::Strand(Strand(self.profile_pic.unwrap())),
            "contact".to_string() => Value::Strand(Strand(self.contact.unwrap())),
        };

        // if let Some(date_of_birth) = self.date_of_birth.clone() {
        //     map.insert(
        //         "date_of_birth".to_string(),
        //         Value::Datetime(Datetime(DateTime::<Utc>::from_utc(
        //             NaiveDate::parse_from_str(&date_of_birth, "%d-%m-%Y")
        //                 .expect("Invalid date format")
        //                 .and_hms_opt(0, 0, 0)
        //                 .unwrap(),
        //             Utc,
        //         ))),
        //     );
        // } 

        // if let Some(address) = self.address.clone() {
        //     let api_key = dotenv!("MAP_API_KEY");
        //     let embed_link = generate_embed_link(Some(address.clone()), api_key).replace(" ", "%20");
         
        //     // add address to profile
        //     map.insert("address".to_string(), Value::Strand(Strand(address)));

        //     // println!("Embed link: {}", embed_link);
        //     map.insert("gmap".to_string(), Value::Strand(Strand(embed_link)));
        // }

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


// pub async fn update_dob(
//     user_id: String,
//     db: Arc<Surreal<Client>>,
//     date_of_birth: Option<String>,
// ) -> Result<(), StatusCode> {
//     match date_of_birth {
//         None => return Err(StatusCode::BAD_REQUEST),
//         _ => {}
//     }

//     let response: Result<String, surrealdb::Error> = db
//         .update(("undergraduate", user_id))
//         // parse dob as datetime
//         // .patch(PatchOp::replace("/date_of_birth",date_of_birth.unwrap()))
//         .patch(PatchOp::replace(
//             "/date_of_birth",
//             Datetime(DateTime::<Utc>::from_utc(
//                 NaiveDate::parse_from_str(&date_of_birth.unwrap(), "%d-%m-%Y")
//                     .expect("Invalid date format")
//                     .and_hms_opt(0, 0, 0)
//                     .unwrap(),
//                 Utc,
//             )),
//         ))
//         .await;

//     match response {
//         Ok(_) => Ok(()),
//         Err(e) => {
//             println!("{:?} ", e);
//             Ok(())
//         }
//     }
    pub async fn get_profile_update_query(self) -> Result<String,StatusCode>{
        let mut query = String::new();
        query.push_str("UPDATE profile SET ");
        let mut count = 0;
        if let Some(intro) = self.intro.clone() {
            query.push_str(&format!("intro = '{}'",intro));
            count += 1;
        }
        if let Some(profile_pic) = self.profile_pic.clone() {
            if count > 0 {
                query.push_str(",");
            }
            query.push_str(&format!("profile_pic = '{}'",profile_pic));
            count += 1;
        }
        if let Some(contact) = self.contact.clone() {
            if count > 0 {
                query.push_str(",");
            }
            query.push_str(&format!("contact = '{}'",contact));
            
        }
        query.push_str(&format!(" WHERE id = '{}'",self.id.unwrap()));
        Ok(query)
    }
   
}


impl Into<serde_json::Value> for Profile {
    fn into(self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "intro": self.intro,
            "profile_pic": self.profile_pic,
            "contact": self.contact,
        })
    }
}