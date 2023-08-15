use surrealdb::sql::Thing;

pub struct Project {
    id: Option<Thing>,
    title: Option<String>,
    description: Option<String>,
    delete: Option<String>,
    time: String,
}

impl Project {}
