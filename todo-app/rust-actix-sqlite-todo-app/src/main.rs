use actix_web::{error, get, middleware::Logger, post, web, App, HttpServer, Responder};
use derive_more::{Display, Error};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

struct State {
    db: SqlitePool,
}

#[derive(Debug, Display, Error)]
enum Error {
    Sqlx(sqlx::Error),
}

impl error::ResponseError for Error {}

#[derive(sqlx::FromRow)]
struct Todo {
    name: String,
    id: u32,
    done: bool,
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Sqlx(value)
    }
}

#[get("/")]
async fn index(state: web::Data<Arc<State>>) -> Result<Markup, Error> {
    let todos = sqlx::query_as::<_, Todo>(
        "
            SELECT id, name, done FROM todos ORDER BY done DESC
        ",
    )
    .fetch_all(&state.db)
    .await?;

    Ok(html! {
        (DOCTYPE)
        html {
            head {
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                style {
                    "
                        * { margin: 0; padding: 0; border: 0px solid currentColor; box-sizing: border-box; font-family: sans-serif;}
                        body { padding: 2rem; box-sizing: content-box; max-width: 65ch; width: 100%; display: flex; flex-direction: column; }
                        html { padding: 2rem; display: flex; flex-direction: column; align-items: center; justify-content: center; }
                        ul { list-style: none; }
                        li { display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem;}
                        li button.delete { padding: 0; background: transparent; transition-duration: 200ms; padding: 0.25rem; border: 1px solid transparent; border-radius: 0.25rem; }
                        li button.delete:hover {opacity: 0.5;}
                        li button.delete:active, li button.delete:focus {border: 1px solid currentColor; outline: 0px transparent; }
                        li.completed .text { text-decoration: line-through; }
                        .new-todo { padding: 1rem; background-color: lightgray; display: block; border-radius: 0.5rem; flex-grow: 1; }
                        .new-todo-form { flex-grow: 1; display: flex; }
                    "
                }
            }
            body {
                h1 { "Todos" }
                ul {
                    @for todo in todos {
                        li class=(if todo.done {"completed"} else {""}){
                            form action=(format!("/todos/{}/delete", todo.id)) method="POST" {
                                button class="delete" aria-label="Delete todo" {
                                    "❌"
                                }
                            }
                            form action=(format!("/todos/{}/toggle-completion", todo.id)) method="POST" {
                                button class="text" {
                                    (todo.name)
                                }
                            }
                        }
                    }
                    li {
                        form class="new-todo-form" action="/todos/new" method="POST" {
                            input class="new-todo" placeholder="Create new TODO..." name="name" autofocus;
                        }
                    }
                }
            }
        }
    })
}

#[derive(Deserialize)]
struct NewTodoForm {
    name: String,
}

#[post("/todos/new")]
async fn new_todo(
    state: web::Data<Arc<State>>,
    web::Form(form): web::Form<NewTodoForm>,
) -> Result<impl Responder, Error> {
    sqlx::query(
        "
            INSERT INTO todos (name) VALUES (?)
        ",
    )
    .bind(form.name)
    .execute(&state.db)
    .await?;

    Ok(web::Redirect::to("/").see_other())
}

#[post("/todos/{id}/toggle-completion")]
async fn update_todo_completion(
    state: web::Data<Arc<State>>,
    id: web::Path<u32>,
) -> Result<impl Responder, Error> {
    sqlx::query(
        "
            UPDATE todos SET done = NOT done WHERE id=?
        ",
    )
    .bind(id.into_inner())
    .execute(&state.db)
    .await?;

    Ok(web::Redirect::to("/").see_other())
}

#[post("/todos/{id}/delete")]
async fn delete_todo(
    state: web::Data<Arc<State>>,
    id: web::Path<u32>,
) -> Result<impl Responder, Error> {
    sqlx::query(
        "
            DELETE FROM todos WHERE id=?
        ",
    )
    .bind(id.into_inner())
    .execute(&state.db)
    .await?;

    Ok(web::Redirect::to("/").see_other())
}

async fn initialize_db() -> Result<SqlitePool, sqlx::Error> {
    let db = SqlitePool::connect("sqlite::memory:").await?;

    sqlx::query(
        "
            CREATE TABLE todos (
                id INTEGER NOT NULL PRIMARY KEY,
                name STRING NOT NULL,
                done BOOL NOT NULL DEFAULT false
            )
        ",
    )
    .execute(&db)
    .await?;

    Ok(db)
}

#[actix_web::main]
async fn main() {
    env_logger::init();

    let state = Arc::new(State {
        db: initialize_db().await.expect("Failed to initialize db"),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(index)
            .service(new_todo)
            .service(update_todo_completion)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 3000))
    .expect("Failed to get port")
    .run()
    .await
    .expect("Failed to await server creation")
}
