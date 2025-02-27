#![allow(unused)]

use axum::extract::State;
use axum::http::StatusCode;
use construction::db::Database;
use maud::html;
use maud::Markup;

const POSTGRES_URL: &str = "postgres://construction:password@localhost:6543/construction";

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
    }
}

async fn run() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7333").await?;

    let database = Database::connect(POSTGRES_URL).await?;

    sqlx::migrate!("./migrations").run(&*database).await?;

    let router = axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/employees", axum::routing::get(employees))
        .route("/sites", axum::routing::get(sites))
        .route("/departments", axum::routing::get(departments))
        .route("/equipment", axum::routing::get(equipment))
        .with_state(database)
        .into_make_service();

    axum::serve(listener, router).await?;

    Ok(())
}

async fn index() -> Markup {
    html! {
        html {
            head {
                title { "Construct." }
                script src="https://unpkg.com/htmx.org@2.0.4" {}
            }

            body {
                header {
                    ul {
                        li { button hx-get="/employees" hx-target="main" { "Employees" } }
                        li { button hx-get="/sites" hx-target="main" { "Sites" } }
                        li { button hx-get="/departments" hx-target="main" { "Departments" } }
                        li { button hx-get="/equipment" hx-target="main" { "Equipment" } }
                    }
                }

                main {}
            }
        }
    }
}

async fn employees(State(db): State<Database>) -> Result<Markup, StatusCode> {
    Ok(Markup::default())
}

async fn sites(State(db): State<Database>) -> Result<Markup, StatusCode> {
    Ok(Markup::default())
}

async fn departments(State(db): State<Database>) -> Result<Markup, StatusCode> {
    Ok(Markup::default())
}

async fn equipment(State(db): State<Database>) -> Result<Markup, StatusCode> {
    Ok(Markup::default())
}
