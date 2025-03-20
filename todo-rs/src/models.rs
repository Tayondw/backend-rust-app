use diesel::prelude::*; // brings in Diesel traits and functions, allowing interaction with the db
use serde::{Deserialize, Serialize}; // allows structs to be converted to/from JSON to API responses

// Queryable - enables Diesel to fetch db rows and map them into this struct
// Serialize - allows the struct to be serialized into JSON for API responses
#[derive(Queryable,Serialize)] // applies the two derive macros to the struct that precedes it
pub struct Todo {
    pub id: i32, // unique identifier of the todo item
    pub title: String, // title of todo item
    pub content: String, // content/description of the todo
}

// Insertable - allows this struct to be used for inserting new rows into the db
// Deserialize - allows it to be deserialized from JSON to API requests
#[derive(Insertable,Deserialize)]
#[diesel(table_name = crate::schema::todos)] // specifies that this struct maps to the todos table in the db schema
pub struct NewTodo { // defines NewTodo, which omits id since the database assigns it automatically
    pub title: String,
    pub content: String,
}

// AsChangeSet - allows Diesel to use this struct to update an existing database record
// Deserialize - enables JSON conversion when updating a todo via an API
#[derive(AsChangeset,Deserialize)]
#[diesel(table_name = crate::schema::todos)] // specifies that this struct corresponds to the todo table
pub struct UpdateTodo { // defines UpdateTodo which allows updating only specific fields (title and content)
    pub title: String,
    pub content: String,
}