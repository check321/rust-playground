use axum::extract::{
    Query, Json, Path, State,
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};
use bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    // init logger.
    tracing_subscriber::fmt::init();

    // init db via bb8.
    let manager =
        PostgresConnectionManager::new_from_stringlike("host=192.168.1.11 user=myuser dbname=mydb password=postgres3721", NoTls)
            .unwrap();

    // db connection pool.
    let pool = Pool::builder().build(manager).await.unwrap();

    // router init.
    let app = Router::new()
        .route("/todos", get(todos_index))
        .route("/todo/new", post(todo_create))
        .route("/todo/update", post(todo_update))
        .route("/todo/delete/:id", post(todo_remove))
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404)
        .with_state(pool);

    // launch app.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6001")
        .await
        .unwrap();

    tracing::debug!("axum is listening on {}",listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

async fn todos_index(State(pool): State<ConnectionPool>,pagination: Option<Query<Pagination>>,
                      ) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let Query(pagination) = pagination.unwrap_or_default();
    let offset: i64 = pagination.offset.unwrap_or(0);
    let limit: i64 = pagination.limit.unwrap_or(10);

    let rows = conn.query(
        "select id, description, completed from t_todo offset $1 limit $2",
        &[&offset, &limit],
    ).await.map_err(internal_error)?;

    let mut todos: Vec<Todo> = Vec::new();
    for row in rows {
        let id = row.get(0);
        let description = row.get(1);
        let completed = row.get(2);
        let todo = Todo {
            id,
            description,
            completed,
        };
        todos.push(todo);
    }

    Ok(Json(todos))
}


#[derive(Debug, Deserialize)]
struct CreateTodo {
    description: String,
}

async fn todo_create(State(pool): State<ConnectionPool>, Json(input): Json<CreateTodo>) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let todo = Todo {
        id: Uuid::new_v4().simple().to_string(),
        description: input.description,
        completed: false,
    };

    let conn = pool.get().await.map_err(internal_error)?;

    let _ret = conn.execute(
        "insert into t_todo (id, description, completed) values ($1, $2, $3) returning id",
        &[&todo.id, &todo.description, &todo.completed],
    ).await.map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    id: String,
    description: Option<String>,
    completed: Option<bool>,
}

async fn todo_update(State(pool): State<ConnectionPool>, Json(input): Json<UpdateTodo>) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let id: String = input.id.clone();
    let description: String = input.description.unwrap_or("".to_string());
    let completed: bool = input.completed.unwrap_or(false);

    let _ret = conn.execute("update t_todo set description=$1,completed=$2 where id=$3", &[&description, &completed, &id])
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(id)))
}

async fn todo_remove(Path(id): Path<String>, State(pool): State<ConnectionPool>) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let _ret = conn.execute("delete from t_todo where id = $1", &[&id])
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(id)))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "there is nothing here.")
}


fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

