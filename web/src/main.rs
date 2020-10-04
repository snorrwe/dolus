use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use log::debug;
use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::Arc;
use warp::Filter;

const INDEX: &str = include_str!("index.html");

#[derive(Serialize, Debug)]
struct Crawled {
    created: DateTime<Utc>,
    counts: serde_json::Value,
    page_hash: String,
    url: String,
}

async fn index(hbs: Arc<Handlebars<'_>>, db: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let values = sqlx::query_as!(
        Crawled,
        r#"
        SELECT created, counts, page_hash, url
        FROM crawled
        ORDER BY url,created DESC
        "#
    )
    .fetch_all(&db)
    .await
    .unwrap();

    debug!("values from db: {:#?}", values);

    let mut hb_values = HashMap::new();
    hb_values.insert("crawled", values);

    debug!("values: {:#?}", hb_values);

    let render = hbs.render("index.html", &hb_values).unwrap();

    Ok(warp::reply::html(render))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut hb = Handlebars::new();
    // register the template
    hb.register_template_string("index.html", INDEX).unwrap();

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);
    let hb = {
        let filter = warp::any().map(move || hb.clone());
        move || filter.clone()
    };

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var required");

    let db_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .unwrap();

    let db_pool = {
        let filter = warp::any().map(move || db_pool.clone());
        move || filter.clone()
    };

    let route = warp::get().and(hb()).and(db_pool()).and_then(index);

    warp::serve(route).run(([0, 0, 0, 0], 8000)).await;
}
