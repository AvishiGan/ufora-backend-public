use core::fmt;

use surrealdb::sql::Thing;


// Item enum specifies whether the query is for a table or a record
// for a table, Item::Table("table_name".to_string()) is used
// for a record, Item::Record{tb: "table_name".to_string(), id: "record_id".to_string()} is used
#[derive(Debug)]
pub enum Item {
    Table(String),
    Record {tb: String, id: String},
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Table(table_name) => write!(f, "{}", table_name),
            Item::Record {tb, id} => write!(f, "{}:{}", tb, id),
        }
    }
}

// Column enum specifies whether the query is for all columns or specific columns
// for all columns, Column::All is used
// for specific columns, Column::Specific(vec![String::from("column_name_1"),String::from("column_name_2")]) is used
#[derive(Debug)]
pub enum Column {
    All,
    Specific(Vec<String>),
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Column::All => write!(f, "Select * "),
            Column::Specific(column_names) => {
                let mut column_names_string = String::from("Select ");
                for column_name in column_names {
                    column_names_string.push_str(column_name);
                    column_names_string.push_str(", ");
                }
                column_names_string.pop();
                column_names_string.pop();
                write!(f, "{}", column_names_string)
            }
        }
    }
}

// LogicalOperator enum specifies the type of logical operator used in the query
// for and, LogicalOperator::And is used
// for or, LogicalOperator::Or is used
pub enum ExpressionConnector {
    And,
    Or,
    End
}

impl fmt::Display for ExpressionConnector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionConnector::And => write!(f, " AND "),
            ExpressionConnector::Or => write!(f, " OR "),
            ExpressionConnector::End => write!(f, " "),
        }
    }
}

// Expression enum specifies the type of expression used in the query
// for equal to, Expression::EqualTo(String::from("column_name"),String::from("value")) is used
// for not equal to, Expression::NotEqualTo(String::from("column_name"),String::from("value")) is used
// for greater than, Expression::GreaterThan(String::from("column_name"),String::from("value")) is used
// for less than, Expression::LessThan(String::from("column_name"),String::from("value")) is used
// for greater than or equal to, Expression::GreaterThanOrEqualTo(String::from("column_name"),String::from("value")) is used
// for less than or equal to, Expression::LessThanOrEqualTo(String::from("column_name"),String::from("value")) is used
// for is None, Expression::IsNone(String::from("column_name")) is used
// for is not None, Expression::IsNotNone(String::from("column_name")) is used
#[derive(Debug)]
pub enum Expression {
    EqualTo(String, String),
    NotEqualTo(String, String),
    GreaterThan(String, String),
    LessThan(String, String),
    GreaterThanOrEqualTo(String, String),
    LessThanOrEqualTo(String, String),
    IsNone(String),
    IsNotNone(String),
    EdgeExpression(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::EqualTo(column_name, value) => write!(f, " {} = {} ", column_name, value),
            Expression::NotEqualTo(column_name, value) => write!(f, " {} != {} ", column_name, value),
            Expression::GreaterThan(column_name, value) => write!(f, " {} > {} ", column_name, value),
            Expression::LessThan(column_name, value) => write!(f, " {} < {} ", column_name, value),
            Expression::GreaterThanOrEqualTo(column_name, value) => write!(f, " {} >= {} ", column_name, value),
            Expression::LessThanOrEqualTo(column_name, value) => write!(f, " {} <= {} ", column_name, value),
            Expression::IsNone(column_name) => write!(f, " {} = None ", column_name),
            Expression::IsNotNone(column_name) => write!(f, " {} != None ", column_name),
            Expression::EdgeExpression(edge_condition) => write!(f, " {} ", edge_condition),
        }
    }
}

// GroupBy enum specifies whether the query is for all columns or specific columns
// for all columns, GroupBy::All is used
// for specific columns, GroupBy::Specific(vec![String::from("column_name_1"),String::from("column_name_2")]) is used
pub enum Group {
    All,
    Specific(Vec<String>),
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group::All => write!(f, " GROUP ALL"),
            Group::Specific(column_names) => {
                let mut column_names_string = String::from(" GROUP ");
                for column_name in column_names {
                    column_names_string.push_str(column_name);
                    column_names_string.push_str(", ");
                }
                column_names_string.pop();
                column_names_string.pop();
                write!(f, "{} ", column_names_string)
            }
        }
    }
}

// OrderBy enum specifies whether the query is for ascending or descending order
// for ascending order, OrderBy::Ascending(vec![String::from("column_name_1"),String::from("column_name_2")]) is used
// for descending order, OrderBy::Descending(vec![String::from("column_name_1"),String::from("column_name_2")]) is used
#[derive(Debug)]
pub enum OrderBy {
    Ascending(Vec<String>),
    Descending(Vec<String>),
}

impl fmt::Display for OrderBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderBy::Ascending(column_names) => {
                let mut column_names_string = String::from(" ORDER BY ");
                for column_name in column_names {
                    column_names_string.push_str(column_name);
                    column_names_string.push_str(", ");
                }
                column_names_string.pop();
                column_names_string.pop();
                write!(f, "{}", column_names_string + " ASC")
            },
            OrderBy::Descending(column_names) => {
                let mut column_names_string = String::from(" ORDER BY ");
                for column_name in column_names {
                    column_names_string.push_str(column_name);
                    column_names_string.push_str(", ");
                }
                column_names_string.pop();
                column_names_string.pop();
                write!(f, "{}", column_names_string + " DESC")
            },
        }
    }
}

// struct to specify the type of return value
// for no return value, Return::NONE is used
// for difference between the new and old value, Return::Difference is used
// for old value, Return::Before is used
// for new value, Return::After is used
// for specific fields, Return::Fields{fields: vec![String::from("field_name_1"),String::from("field_name_2")]} is used
pub enum Return {
    NONE,
    Difference,
    Before,
    After,
    Fields {fields: Vec<String>},
} 

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Return::NONE => write!(f, " RETURN NONE"),
            Return::Difference => write!(f, " RETURN DIFF"),
            Return::Before => write!(f, " RETURN BEFORE"),
            Return::After => write!(f, " RETURN AFTER"),
            Return::Fields {fields} => {
                let mut fields_string = String::from(" RETURN ");
                for field in fields {
                    fields_string.push_str(field);
                    fields_string.push_str(", ");
                }
                fields_string.pop();
                fields_string.pop();
                write!(f, "{}", fields_string)
            },
        }
    }
}

// to get a select query with known parameters
// NOTE: if no condition is provided, pass None
// NOTE: last expression connector is always ExpressionConnector::End
// NOTE: if no group_by is provided, pass None
// NOTE: if no order_by is provided, pass None
// NOTE: if no limit is provided, pass None
// NOTE: if no start is provided, pass None
pub fn get_select_query(
    table_name: Item,
    column_names: Column,
    condition: Option<Vec<(Expression,ExpressionConnector)>>,
    group_by: Option<Group>,
    order_by: Option<OrderBy>,
    limit: Option<i32>, // limit is the number of records to be returned
    start: Option<i32>, // start is the number of records to be skipped
) -> String {

    let mut  query = String::new();

    query.push_str(&column_names.to_string());

    query.push_str(" FROM ");

    query.push_str(&table_name.to_string());

    if let Some(condition) = condition {
        if condition.len() > 0 {
            query.push_str(" WHERE ");
            for (expression_1,expression_2) in condition {
                query.push_str(&expression_1.to_string());
                query.push_str(&expression_2.to_string());
            }
        } else {
            println!("No condition provided")
        }
        
    } 

    if let Some(group_by) = group_by {
        query.push_str(&group_by.to_string());
    }

    if let Some(order_by) = order_by {
        query.push_str(&order_by.to_string());
    }

    if let Some(limit) = limit {
        query.push_str(&format!(" LIMIT {}",limit));
    }

    if let Some(start) = start {
        query.push_str(&format!(" START {}",start));
    }

    query
}

pub fn get_insert_query_by_fields(
    table_name: String,
    column_names: Vec<String>,
    values: Vec<String>,
) -> String {
    let mut query = String::new();

    query.push_str("INSERT INTO ");
    query.push_str(&table_name);
    query.push_str(" (");

    for column_name in column_names {
        query.push_str(&column_name);
        query.push_str(", ");
    }

    query.pop();
    query.pop();

    query.push_str(") VALUES (");

    for value in values {
        query.push_str(&value);
        query.push_str(", ");
    }

    query.pop();
    query.pop();

    query.push_str(")");

    query
}

// representation of a JSON object to be inserted into the database
// keys and values are stored in separate vectors
// keys and values are stored in the same order as they are inserted
// NOTE: if a value to be stored as string wrap it around single quotes inside double quotes
#[derive(Debug)]
pub struct DatabaseObject {
    pub keys: Vec<String>,
    pub values: Vec<String>,
}

impl DatabaseObject {
    pub fn new() -> DatabaseObject {
        DatabaseObject {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add_key(&mut self, key: String) {
        self.keys.push(key);
    }

    pub fn add_value(&mut self, value: String) {
        self.values.push(value);
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    pub fn get_values(&self) -> Vec<String> {
        self.values.clone()
    }
}

impl fmt::Display for DatabaseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        if self.keys.len() != self.values.len() {
            panic!("Keys and values are not of the same length");
        }

        let mut query = String::new();

        query.push_str("{");

        for (key,value) in self.keys.iter().zip(self.values.iter()) {
            query.push_str(&format!("{}: {}, ",key,value));
        }

        query.pop();
        query.pop();

        query.push_str("}");

        write!(f, "{}", query)
    }
}

// to insert a single json object into a table
pub fn get_insert_query_for_an_object(
    table_name: String,
    object: DatabaseObject,
    result: Return
) -> String {
    let mut query = String::new();

    query.push_str("INSERT INTO ");
    query.push_str(&table_name);
    query.push_str(" ");
    query.push_str(&object.to_string());

    query.push_str(&result.to_string());

    query
}

// to insert more than one json object into a table
pub fn get_insert_query_for_an_array_of_objects(
    table_name: String,
    objects: Vec<DatabaseObject>,
    result: Return,
) -> String {
    let mut query = String::new();

    query.push_str("INSERT INTO ");
    query.push_str(&table_name);
    query.push_str(" [");

    for object in objects {
        query.push_str(&object.to_string());
        query.push_str(", ");
    }

    query.pop();
    query.pop();

    query.push_str("]");

    query.push_str(&result.to_string());

    query
}

// to delete a specific record from a table with known record id
pub fn get_delete_query_for_specific_record(
    table_name: String,
    record_id: String,
) -> String {
    let mut query = String::new();

    query.push_str("DELETE ");
    query.push_str(&table_name);
    query.push_str(":");
    query.push_str(&record_id);

    query
}

// to delete records from a table with conditions
// conditions follow same syntax as in get_select_query
pub fn get_delete_query_with_conditions(
    table_name: String,
    condition: Vec<(Expression,ExpressionConnector)>,
    result: Option<Return>,
) -> String {
    let mut query = String::new();

    query.push_str("DELETE ");
    query.push_str(&table_name);
    query.push_str(" WHERE ");

    for (expression_1,expression_2) in condition {
        query.push_str(&expression_1.to_string());
        query.push_str(&expression_2.to_string());
    }

    match result {
        None => {}
        Some(result) => query.push_str(&result.to_string())
    }

    query
}

// to create a record with a specific id or auto generated one
pub fn get_create_query_for_an_object(
    table_name: Item,
    object: DatabaseObject,
    result: Return,
) -> String {

    let mut query = String::from("CREATE ");

    query.push_str(&table_name.to_string());

    query.push_str(" CONTENT ");

    query.push_str(&object.to_string());

    query.push_str(&result.to_string());

    query
}

pub fn get_relate_query_with_content(
    from: Thing,
    to: Thing,
    relation_name: String,
    content: Option<DatabaseObject>
) -> String {

    let mut query = String::from("Relate ");

    query.push_str(&from.to_string() );
    query.push_str("->");
    query.push_str(&relation_name);
    query.push_str("->");
    query.push_str(&to.to_string());

    match content {
        None => {}
        Some(content) => query.push_str(&content.to_string())
    }

    query
}