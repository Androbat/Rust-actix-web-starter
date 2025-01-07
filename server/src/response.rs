use serde::Serialize;
use crate::model::Todo;

// API response struct
// Since the response is usually a struct
// which is deserialized to be converted into a JSON,
// we create those structs

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String
}

#[derive(Serialize, Debug)]
pub struct TodoData {
    pub todo: Todo,
}

#[derive(Serialize, Debug)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: TodoData,
}

#[derive(Serialize, Debug)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Todo>
}