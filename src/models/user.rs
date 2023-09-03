// imports
// __________________________________
use std::{sync::Arc, vec};

use chrono::{DateTime, NaiveDate, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use simple_collection_macros::bmap;

use axum::http::StatusCode;

use surrealdb::{
    engine::remote::ws::Client,
    opt::PatchOp,
    sql::{
        statements::{CreateStatement, SelectStatement, UpdateStatement},
        Array, Cond, Data, Datetime, Expression, Field, Fields, Ident, Idiom, Limit, Number,
        Object, Operator, Output, Part, Strand, Table, Thing, Value, Values,
    },
    Surreal,
};
use surrealdb_extra::query_builder::{
    filter::{LogicalOperator, RelationalOperator},
    Query,
};

use crate::services::{
    password,
    query_builder::{get_select_query, Column, ExpressionConnector, Item, OrderBy},
};

// model for user
// __________________________________
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    id: Option<Thing>,
    name: Option<String>,
    username: Option<String>,
    password: Option<String>,
    locked_flag: Option<bool>,
    user_type: Option<String>,
    email: Option<String>,
    registration_date: Option<Datetime>,
    email_verification_flag: Option<bool>,
    pub invalid_login_attempts: Option<i32>,

    // profile
    intro: Option<String>,
    profile_pic: Option<String>,
    contact: Option<String>,

    // optional params depending on user
    // company params
    address: Option<String>,
    gmap: Option<String>,

    // undergraduate params
    date_of_birth: Option<String>,
    university: Option<String>,
    is_premium: Option<bool>,

    // club params
    club_type: Option<String>,
    officials: Option<Vec<ClubOfficial>>,
    club_verification_file: Option<String>,
    club_verification_flag: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClubOfficial {
    user_id: Thing,
    role: String,
}

impl ClubOfficial {
    pub fn get_user_id(&self) -> Thing {
        self.user_id.clone()
    }

    pub fn get_role(&self) -> String {
        self.role.clone()
    }
}

// implementation of user
// __________________________________
impl User {
    // returns a new user model
    // __________________________________
    pub fn from(
        username: Option<String>,
        name: Option<String>,
        password: Option<String>,
        email: Option<String>,
    ) -> Self {
        Self {
            id: None,
            username,
            name,
            password,
            email,
            ..Default::default()
        }
    }

    pub async fn get_user_by_id(
        db: Arc<Surreal<Client>>,
        user_id: String,
    ) -> Result<Self,String> {
        let response: Result<Option<Self>, surrealdb::Error> = db.select(("user", user_id)).await;
        match response {
            Ok(user) => Ok(user.unwrap()),
            Err(e) => Err(e.to_string()),
        }
    }

    // returns the surrealQl query for creating a user
    // __________________________________
    pub async fn get_create_user_query(
        self,
        user_type: String,
    ) -> Result<CreateStatement, StatusCode> {
        match (self.username.clone(), self.password.clone()) {
            (None, _) | (_, None) => Err(StatusCode::BAD_REQUEST)?,
            (_, _) => {}
        }

        Ok(CreateStatement {
            what: Values(vec![Value::Table(Table("user".to_string()))]),
            data: Some(Data::ContentExpression(Value::Object(Object(bmap!(
                "username".to_string() => Value::Strand(Strand(self.username.unwrap())),
                "name".to_string() => Value::Strand(Strand(self.name.unwrap())),
                "password".to_string() => Value::Strand(Strand(password::hash_password(self.password.unwrap())?)),
                "locked_flag".to_string() => Value::False,
                "user_type".to_string() => Value::Strand(Strand(user_type)),
                "email".to_string() => Value::Strand(Strand(self.email.unwrap())),
                "email_verification_flag".to_string() => Value::False,
                "invalid_login_attempts".to_string() => Value::Number(Number::Int(0)),
                "registration_date".to_string() => Value::Datetime(Datetime::default()),
            ))))),
            output: Some(Output::Null),
            timeout: None,
            parallel: false,
        })
    }

    // updates the university email verification flag
    // __________________________________
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

    // updates the university email verification details
    // __________________________________
    pub async fn update_university_details(
        user_id: Thing,
        db: Arc<Surreal<Client>>,
        university: Option<String>,
        university_email: Option<String>,
    ) -> Result<(), StatusCode> {
        match (university.clone(), university_email.clone()) {
            (None, _) | (_, None) => {
                return Err(StatusCode::BAD_REQUEST);
            }
            (_, _) => {}
        }

        let response: Result<String, surrealdb::Error> = db
            .update(("user", user_id.id))
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

    // returns the user from the database
    // __________________________________
    pub async fn retrieve_user_from_database_by_username(
        db: Arc<Surreal<Client>>,
        username: String,
    ) -> Result<Self, StatusCode> {
        let mut response = db
            .query(SelectStatement {
                expr: Fields(vec![Field::All], true),
                what: Values(vec![Value::Table(Table("user".to_string()))]),
                cond: Some(Cond(Value::Expression(Box::from(Expression {
                    l: Value::Idiom(Idiom(vec![Part::Field(Ident("username".to_string()))])),
                    o: surrealdb::sql::Operator::Equal,
                    r: Value::Strand(Strand(username)),
                })))),
                group: None,
                order: None,
                limit: Some(Limit(Value::Number(Number::Int(1)))),
                start: None,
                fetch: None,
                version: None,
                split: None,
                timeout: None,
                parallel: false,
            })
            .await
            .unwrap();

        let users: Option<Self> = response.take(0).unwrap();

        match users {
            Some(user) => Ok(user),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    // gets the stored password
    // __________________________________
    pub fn get_password(&self) -> Option<String> {
        self.password.clone()
    }

    // updates the invalid login attempts and locked account
    // __________________________________
    pub async fn update_login_attempts(
        self,
        db: Arc<Surreal<Client>>,
        new_invalid_login_attempts: i32,
    ) -> () {
        #[derive(serde::Deserialize)]
        struct LoginAttemptUpdateResult {}

        let _response: Option<LoginAttemptUpdateResult> = match new_invalid_login_attempts {
            0..=4 => db
                .update(("user", self.id.unwrap().id))
                .patch(PatchOp::replace(
                    "/invalid_login_attempts",
                    new_invalid_login_attempts,
                ))
                .await
                .unwrap_or(None),
            5 => db
                .update(("user", self.id.unwrap().id))
                .patch(PatchOp::replace(
                    "/invalid_login_attempts",
                    new_invalid_login_attempts,
                ))
                .patch(PatchOp::replace("/locked_flag", true))
                .await
                .unwrap_or(None),
            _ => None,
        };
    }

    // returns whether the user is locked or not
    // __________________________________
    pub fn is_user_locked(&self) -> bool {
        self.locked_flag.unwrap()
    }

    // returns user type
    // __________________________________
    pub fn get_user_type(&self) -> String {
        self.user_type.as_ref().unwrap().clone()
    }

    // returns user id
    // __________________________________
    pub fn get_id(&self) -> Thing {
        self.id.as_ref().unwrap().clone()
    }

    pub fn get_club_officials(&self) -> Option<Vec<ClubOfficial>> {
        if self.user_type.clone().unwrap() == "club" {
            self.officials.clone()
        } else {
            None
        }
    }

    // returns whether the user is verified or not
    // __________________________________
    pub async fn update_email_verification(
        db: Arc<Surreal<Client>>,
        email: String,
    ) -> Result<(), StatusCode> {
        let _response = db
            .query(UpdateStatement {
                what: Values(vec![Value::Table(Table("user".to_string()))]),
                data: Some(Data::SetExpression(vec![(
                    Idiom(vec![Part::Field(Ident(
                        "email_verification_flag".to_string(),
                    ))]),
                    Operator::Equal,
                    Value::True,
                )])),
                cond: Some(Cond(Value::Expression(Box::from(Expression {
                    l: Value::Idiom(Idiom(vec![Part::Field(Ident("email".to_string()))])),
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

    // returns user by email
    // __________________________________
    pub async fn get_user_by_email(
        db: Arc<Surreal<Client>>,
        email: String,
    ) -> Result<Self, StatusCode> {
        let mut response = db
            .query(SelectStatement {
                expr: Fields(vec![Field::All], true),
                what: Values(vec![Value::Table(Table("user".to_string()))]),
                cond: Some(Cond(Value::Expression(Box::from(Expression {
                    l: Value::Idiom(Idiom(vec![Part::Field(Ident("email".to_string()))])),
                    o: surrealdb::sql::Operator::Equal,
                    r: Value::Strand(Strand(email)),
                })))),
                group: None,
                order: None,
                limit: Some(Limit(Value::Number(Number::Int(1)))),
                start: None,
                fetch: None,
                version: None,
                split: None,
                timeout: None,
                parallel: false,
            })
            .await
            .unwrap();

        let users: Option<Self> = response.take(0).unwrap();

        match users {
            Some(user) => Ok(user),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    // returns user email
    // __________________________________
    pub fn get_user_email(&self) -> String {
        match self.email.clone() {
            Some(email) => email,
            None => "".to_string(),
        }
    }

    // returns username
    // __________________________________
    pub fn get_user_username(&self) -> String {
        match self.username.clone() {
            Some(username) => username,
            None => "".to_string(),
        }
    }

    // returns user name by email or username
    // __________________________________
    pub async fn get_user_by_email_or_username(
        db: Arc<Surreal<Client>>,
        email: Option<String>,
        username: Option<String>,
    ) -> Result<Self, StatusCode> {
        match (email.clone(), username.clone()) {
            (None, None) => {
                return Err(StatusCode::BAD_REQUEST);
            }
            (_, _) => {}
        }

        let response = Query::new()
            .from("user", None)
            .limit(1)
            .field("name")
            .field("username")
            .field("email")
            .field("password")
            .field("user_id")
            .field("user_type")
            .field("locked_flag")
            .field("invalid_login_attempts")
            .field("id")
            .where_filter()
            .filter((
                "username",
                RelationalOperator::Equal,
                username.unwrap_or("".to_string()),
                LogicalOperator::Or,
            ))
            .unwrap_right()
            .filter((
                "email",
                RelationalOperator::Equal,
                email.unwrap_or("".to_string()),
                LogicalOperator::End,
            ))
            .unwrap_left()
            .execute(&db)
            .await;

        match response {
            Err(_) => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            Ok(_) => {}
        }

        let mut response = response.unwrap();

        let user: Option<Self> = response.take(0).unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    pub async fn get_club_account_from_email_or_name(
        db: Arc<Surreal<Client>>,
        email: Option<String>,
        name: Option<String>,
    ) -> Option<Self> {
        db.query(get_select_query(
            Item::Table("user".to_string()),
            Column::All,
            Some(vec![
                (
                    crate::services::query_builder::Expression::EqualTo(
                        "email".to_string(),
                        format!("'{}'", email.unwrap()),
                    ),
                    ExpressionConnector::Or,
                ),
                (
                    crate::services::query_builder::Expression::EqualTo(
                        "name".to_string(),
                        format!("'{}'", name.unwrap()),
                    ),
                    ExpressionConnector::End,
                ),
            ]),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap()
    }

    pub async fn create_a_club_account(
        db: Arc<Surreal<Client>>,
        username: String,
        name: String,
        email: String,
        club_type: String,
        creator: Thing,
        club_verification_file: String,
        profile_pic: Option<String>,
    ) -> Result<serde_json::Value,String>{
        let create_club_query = CreateStatement {
            what: Values(vec![Value::Table(Table("user".to_string()))]),
            data: Some(Data::ContentExpression(Value::Object(Object(bmap!(
                "username".to_string() => Value::Strand(Strand(username)),
                "name".to_string() => Value::Strand(Strand(name)),
                "user_type".to_string() => Value::Strand(Strand("club".to_string())),
                "email".to_string() => Value::Strand(Strand(email)),
                "email_verification_flag".to_string() => Value::False,
                "registration_date".to_string() => Value::Datetime(Datetime::default()),
                "club_type".to_string() => Value::Strand(Strand(club_type)),
                "club_verification_file".to_string() => Value::Strand(Strand(club_verification_file)),
                "club_verification_flag".to_string() => Value::False,
                "profile_pic".to_string() => Value::Strand(Strand(profile_pic.unwrap_or("".to_string()))),
                "officials".to_string() => Value::Array(Array (vec![Value::Object(Object(bmap!(
                    "user_id".to_string() => Value::Thing(creator),
                    "role".to_string() => Value::Strand(Strand("admin".to_string())),
                )))])),
            ))))),
            output: Some(Output::Fields(Fields(
                vec![Field::Alone(Value::Idiom(Idiom(vec![Part::Field(Ident(
                    "email".to_string(),
                ))])))],
                false,
            ))),
            timeout: None,
            parallel: false,
        };

        match db.query(create_club_query).await {
            Ok(mut response) => {
                let club: Option<serde_json::Value> = response.take(0).unwrap();
                Ok(club.unwrap())
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

// implementation of user into json
// __________________________________
impl Into<serde_json::Value> for User {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

// create profile for user
// __________________________________
#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    name: Option<String>,
    intro: Option<String>,
    profile_pic: Option<String>,
    contact: Option<String>,
    // optional params depending on user
    date_of_birth: Option<String>,
    address: Option<String>,
}

pub async fn update_user_profile_query(
    user_id: String,
    user_type: String,
    profile_details: Profile,
) -> Result<UpdateStatement, StatusCode> {
    println!("{:?}", profile_details);

    // fields array to pass all optional fields
    let mut fields = bmap!();

    match profile_details.name {
        None => {}
        _ => {
            fields.insert(
                "name".to_string(),
                Value::Strand(Strand(profile_details.name.unwrap())),
            );
        }
    }

    match profile_details.intro {
        None => {}
        _ => {
            fields.insert(
                "intro".to_string(),
                Value::Strand(Strand(profile_details.intro.unwrap())),
            );
        }
    }

    match profile_details.profile_pic {
        None => {}
        _ => {
            fields.insert(
                "profile_pic".to_string(),
                Value::Strand(Strand(profile_details.profile_pic.unwrap())),
            );
        }
    }

    match profile_details.contact {
        None => {}
        _ => {
            fields.insert(
                "contact".to_string(),
                Value::Strand(Strand(profile_details.contact.unwrap())),
            );
        }
    }

    // only update dob or address if user type is undergraduate or company respectively
    match user_type.as_str() {
        "undergraduate" => match profile_details.date_of_birth {
            None => {}
            _ => {
                fields.insert(
                    "date_of_birth".to_string(),
                    Value::Datetime(Datetime(DateTime::<Utc>::from_utc(
                        NaiveDate::parse_from_str(
                            &profile_details.date_of_birth.unwrap(),
                            "%d-%m-%Y",
                        )
                        .expect("Invalid date format")
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                        Utc,
                    ))),
                );
            }
        },
        "company" => match profile_details.address {
            None => {}
            _ => {
                fields.insert(
                    "address".to_string(),
                    Value::Strand(Strand(profile_details.address.clone().unwrap())),
                );
                fields.insert(
                    "gmap".to_string(),
                    Value::Strand(Strand(
                        format!(
                            "https://www.google.com/maps/embed/v1/place?key={}&q={}",
                            dotenv!("MAP_API_KEY"),
                            profile_details.address.unwrap()
                        )
                        .replace(" ", "%20"),
                    )),
                );
            }
        },
        _ => {}
    }

    println!("{:?}", fields);

    Ok(UpdateStatement {
        what: Values(vec![Value::Table(Table("user".to_string()))]),
        data: Some(Data::MergeExpression(Value::Object(Object(fields)))), // optional fields passed here
        cond: Some(Cond(Value::Expression(Box::from(Expression {
            l: Value::Idiom(Idiom(vec![Part::Field(Ident("id".to_string()))])),
            o: surrealdb::sql::Operator::Equal,
            r: Value::Strand(Strand(format!("user:{}", user_id))),
        })))),
        output: None,
        timeout: None,
        parallel: false,
    })
}

// Getting the profile using the username/email and getting the associated profile
// _________________________________________________________
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    username: Option<String>,
    email: Option<String>,
}

pub async fn get_select_user_query(user_request: UserRequest) -> Result<String, StatusCode> {
    let cond_type: String;
    let cond_value: String;

    // check whether username or email is present or not (firstly username is checked)
    match user_request.username.clone() {
        // if username is not present then check whether email is present or not
        None => match user_request.email.clone() {
            // if email is also not present then return bad request
            None => {
                return Err(StatusCode::BAD_REQUEST);
            }
            // if email is present, continue with email
            _ => {
                cond_type = "email".to_string();
                cond_value = user_request.email.unwrap();
            }
        },

        // if username is present, continue with username
        _ => {
            cond_type = "username".to_string();
            cond_value = user_request.username.unwrap();
        }
    }
    // Ok(get_select_query(table_name, column_names, condition, group_by, order_by, limit, start))

    Ok(get_select_query(
        Item::Table("user".to_string()),
        Column::All,
        Some(vec![(
            crate::services::query_builder::Expression::EqualTo(
                cond_type,
                format!("'{}'", cond_value),
            ),
            ExpressionConnector::End,
        )]),
        None,
        None,
        None,
        None,
    ))
}

//  delete user
// _________________________________________________________
// pub async fn get_delete_user_query(user_id: String) -> Result<UpdateStatement, StatusCode> {
//     let mut fields = bmap!();

//     fields.insert("locked_flag".to_string(), Value::True);

//     Ok(UpdateStatement {
//         what: Values(vec![Value::Table(Table("user".to_string()))]),
//         data: Some(Data::MergeExpression(Value::Object(Object(fields)))), // optional fields passed here
//         cond: Some(Cond(Value::Expression(Box::from(Expression {
//             l: Value::Idiom(Idiom(vec![Part::Field(Ident("id".to_string()))])),
//             o: surrealdb::sql::Operator::Equal,
//             r: Value::Strand(Strand(format!("user:{}", user_id))),
//         })))),
//         output: None,
//         timeout: None,
//         parallel: false,
//     })
// }

// get all profiles
// _________________________________________________________

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectUsersParam {
    user_type: Option<String>,
    order_by_asc: Option<Vec<String>>,
    order_by_desc: Option<Vec<String>>,
    limit: Option<i32>,
    start: Option<i32>,
}

pub async fn get_all_users_query(user_param: SelectUsersParam) -> Result<String, StatusCode> {
    let mut condition: Vec<(
        crate::services::query_builder::Expression,
        ExpressionConnector,
    )> = vec![];

    // check whether user type is present or not
    match user_param.user_type {
        // if user type is not present then check whether user id is present or not
        None => {}
        // if user type is present, continue with user type and check whether user id is present or not
        _ => {
            condition.push((
                crate::services::query_builder::Expression::EqualTo(
                    "user_type".to_string(),
                    format!("'{}'", user_param.user_type.unwrap()),
                ),
                ExpressionConnector::End,
            ));
        }
    }

    Ok(get_select_query(
        Item::Table("user".to_string()),
        Column::All,
        Some(condition),
        None,
        // check whether order by asc or desc is present or not
        match user_param.order_by_asc {
            None => match user_param.order_by_desc {
                None => None,
                _ => Some(OrderBy::Descending(user_param.order_by_desc.unwrap())),
            },
            _ => Some(OrderBy::Ascending(user_param.order_by_asc.unwrap())),
        },
        user_param.limit,
        user_param.start,
    ))
}
