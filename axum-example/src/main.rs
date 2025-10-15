use std::collections::HashSet;
use std::vec;

use axum::extract::State;
use axum::routing::{get, Router};
use axum::response::{Response, IntoResponse};
use axum_extra::extract::Query;

use hypertext::prelude::*;

use serde::Deserialize;
use tower_http::services::ServeDir;

mod errors;
mod icon;

use crate::errors::{AppError, error_404};
use crate::icon::SVG;

#[derive(Clone)]
struct Tag {
    id: String,
}

#[derive(Clone)]
#[allow(dead_code)]
struct Post {
    id: u32,
    title: String,
    content: String,
    tags: Vec<Tag>,
}

#[derive(Clone)]
struct AppState {
    posts: Vec<Post>,
    unique_tags: HashSet<String>,
}

#[tokio::main]
async fn main() {
    let posts: Vec<Post> = vec![
        Post {
            id: 1,
            title: "How to Rust".to_string(),
            content: "Bla bla bla".to_string(),
            tags: vec![Tag {id: "rust".to_string()}]
        },
        Post {
            id: 2,
            title: "How to Java".to_string(),
            content: "Bla bla bla".to_string(),
            tags: vec![Tag {id: "java".to_string()}]
        },
        Post {
            id: 3,
            title: "How to C#".to_string(),
            content: "Bla bla bla".to_string(),
            tags: vec![Tag {id: "java".to_string()}]
        },
        Post {
            id: 4,
            title: "Why Rust is best".to_string(),
            content: "Bla bla bla".to_string(),
            tags: vec![Tag {id: "rust".to_string()}]
        },
    ];
    let unique_tags: HashSet<String> = posts.clone().iter()
        .flat_map(|post| post.tags.clone())
        .map(|tag| tag.id)
        .collect::<HashSet<_>>();

    let state = AppState {
        posts,
        unique_tags,
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello_route))
        .route("/maybe_error", get(error_prone_handler))
        .route("/svg", get(svg))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .fallback(error_404);

    // run our app with hyper, listening globally on port 3000
    let addr = "0.0.0.0:3000";
    println!("Starting app on address: http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Debug)]
struct HelloParams {
    #[serde(rename = "f")]
    filter_query: Option<Vec<String>>,
}

async fn hello_route(
    State(state): State<AppState>,
    Query(params): Query<HelloParams>,
) -> Result<Response, AppError> {
    dbg!(params.filter_query);

    test(false)?;

    Ok(maud!{
        h1 {"Hello Axum!"}
        p {"And maud"}

        @for tag in &state.unique_tags {
            form action="/" method="GET" {
                button type="submit" {
                    (tag)
                }
            }
        }

        @for post in &state.posts {
            div {
                h2 {(post.title)}
                p {(post.content)}
            }
        }
    }.render().into_response())
}

#[derive(Deserialize, Debug)]
struct ErrorParams {
    should_bail: bool,
}

fn test(should_bail: bool) -> Result<(), anyhow::Error> {
    if should_bail {
        anyhow::bail!("Something has gone wrong")
    } else {
        Ok(())
    }
}

async fn error_prone_handler(
    Query(params): Query<ErrorParams>,
) -> Result<Response, AppError> {
    
    test(params.should_bail)?;

    Ok(maud!(
        h1 {"Everything is a-ok!"}
    ).render().into_response())
}

async fn svg() -> Response {
    maud!(
        html {
            link rel="stylesheet" href="/static/svg/svg.css";
        }
        h1 {"Type safe SVG icons!"}
        (SVG::BurgerMenu)
    ).render().into_response()
}
