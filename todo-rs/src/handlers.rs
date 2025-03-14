// POST
/*
In this handler, we accept NewTodo request and will create new record in database. In axum handlers, you can see a state beside request body and they are used for passing dependencies like database connection pools to use for db operations.
*/
pub async fn create_todo(
    State(db): State<DbPool>, // accept db connection pool as dependency
    Json(new_todo): Json<NewTodo> // request body as NewTodo
) -> (StatusCode, Json<Todo>) {
    let mut conn = db
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap(); // get available connection from DB connection pool, throw error otherwise

    let todo = diesel 
        ::insert_into(todos::table) // insert new_todos in todos table
        .values(&new_todo)
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    (StatusCode::CREATED, Json(todo)) // return CREATED status code and new todo item as response body
}

// GET
/*
This time, we don't expect to see something in body, we just return todos items by using load function and cast them to Todo struct. As always, return results in response body with status code OK
*/
pub async fn get_todos(
    State(db): State<DbPool>,
) -> (StatusCode,Json<Vec<Todo>>) {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let results = todos::table.load::<Todo>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    (StatusCode::OK, Json(results))
}

// GET todo id
// We get the todo id from path params and do a query to todos table by filtering id as follows
pub async fn get_todo(
    Path(todo_id): Path<i32>,
    State(db): State<DbPool>,
) -> (StatusCode,Json<Todo>) {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let result = todos::table.filter(id.eq(todo_id)).first::<Todo>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    (StatusCode::OK, Json(result))
}

// UPDATE
// In this handler, we accept update payload from end user and update existing Todo by resolving the id from path params.
pub async fn update_todo(
    Path(todo_id): Path<i32>,
    State(db): State<DbPool>,
    Json(update_todo): Json<UpdateTodo>,
) -> (StatusCode,Json<Todo>) {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let todo = diesel::update(todos::table.filter(id.eq(todo_id)))
        .set(&update_todo)
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    (StatusCode::OK, Json(todo))
}

// DELETE
// As you guess, we resolve todo id from path params then execute delete query against todo table as follows.
pub async fn delete_todo(
    Path(todo_id): Path<i32>,
    State(db): State<DbPool>,
) -> StatusCode {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let _ =diesel::delete(todos::table.filter(id.eq(todo_id)))
        .execute(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    StatusCode::NO_CONTENT
}