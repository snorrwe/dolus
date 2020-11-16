use anyhow::Context;
use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use log::debug;
use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::{collections::HashMap, convert::Infallible};
use warp::Filter;

const INDEX: &str = include_str!("index.html");

#[derive(Serialize, Debug)]
struct Crawled {
    created: DateTime<Utc>,
    counts: serde_json::Value,
    top_words: serde_json::Value,
    page_hash: String,
    url: String,
}

#[derive(Serialize, Debug)]
struct GroupedByUrl {
    url: String,
    payload: Vec<(DateTime<Utc>, u64)>,
}

async fn get_crawled(db: PgPool) -> anyhow::Result<Vec<Crawled>> {
    sqlx::query_as!(
        Crawled,
        r#"
        SELECT created, counts, top_words, page_hash, url
        FROM crawled
        ORDER BY created DESC
        "#
    )
    .fetch_all(&db)
    .await
    .with_context(|| "fetch crawled rows")
}

async fn get_words(db: PgPool) -> anyhow::Result<Vec<String>> {
    struct Keys {
        keys: Option<String>,
    }
    sqlx::query_as!(
        Keys,
        r#"
        SELECT DISTINCT json_object_keys(counts) AS keys
        FROM crawled
        "#
    )
    .fetch_all(&db)
    .await
    .with_context(|| "fetch crawled words")
    .map(|keys| keys.into_iter().filter_map(|Keys { keys }| keys).collect())
}

/// Query crawled results, filtering by given word
async fn get_by_word(db: PgPool, word: String) -> anyhow::Result<Vec<GroupedByUrl>> {
    struct Q {
        created: DateTime<Utc>,
        counts: serde_json::Value,
        url: String,
    }
    let data = sqlx::query_as!(
        Q,
        r#"
        SELECT created, counts, url
        FROM crawled
        ORDER BY created DESC
        "#
    )
    .fetch_all(&db)
    .await
    .with_context(|| "fetch crawled rows")?;

    tokio::task::spawn_blocking(move || {
        let mut res = HashMap::new();
        for (created, count, url) in data.into_iter().filter_map(
            // for each value that has a valid `count`
            |Q {
                 created,
                 counts,
                 url,
             }| {
                counts
                    .as_object()
                    .and_then(|m| {
                        let w = m.get(&word);
                        w.and_then(|c| serde_json::from_value(c.clone()).ok())
                    })
                    .map(move |c| (created, c, url))
            },
        ) {
            res.entry(url)
                .or_insert_with(Vec::new)
                .push((created, count));
        }

        let res = res
            .into_iter()
            .map(|(url, payload)| GroupedByUrl { url, payload })
            .collect();

        Ok(res)
    })
    .await
    .unwrap()
}

async fn index(hbs: Arc<Handlebars<'_>>, db: PgPool) -> Result<impl warp::Reply, Infallible> {
    let values = get_crawled(db).await.unwrap();

    debug!("values from db: {:#?}", values);

    let mut hb_values = HashMap::new();
    hb_values.insert("crawled", values);

    debug!("values: {:#?}", hb_values);

    let render = hbs.render("index.html", &hb_values).unwrap();

    Ok(warp::reply::html(render))
}

async fn counts(db: PgPool) -> Result<impl warp::Reply, Infallible> {
    let values = get_crawled(db).await.unwrap();

    debug!("values from db: {:#?}", values);

    Ok(warp::reply::json(&values))
}

async fn words(db: PgPool) -> Result<impl warp::Reply, Infallible> {
    let values = get_words(db).await.unwrap();

    debug!("values from db: {:#?}", values);

    Ok(warp::reply::json(&values))
}

async fn counts_by_word(
    WordQuery { word }: WordQuery,
    db: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    let values = get_by_word(db, word).await.unwrap();
    Ok(warp::reply::json(&values))
}

#[derive(serde::Deserialize, Debug)]
struct WordQuery {
    word: String,
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

    let index = warp::get().and(hb()).and(db_pool()).and_then(index);
    let counts = warp::get()
        .and(warp::path!("api" / "counts"))
        .and(db_pool())
        .and_then(counts);
    let words = warp::get()
        .and(warp::path!("api" / "words"))
        .and(db_pool())
        .and_then(words);

    let count_by_words = warp::get()
        .and(warp::path!("api" / "counts"))
        .and(warp::filters::query::query())
        .and(db_pool())
        .and_then(counts_by_word);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_owned());
    let port = port.parse().unwrap();

    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);

    let filters = words
        .or(count_by_words)
        .or(counts)
        .or(index)
        .with(cors)
        .with(warp::log("dolus"));

    warp::serve(filters).run(([0, 0, 0, 0], port)).await;
}
