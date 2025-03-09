use std::sync::LazyLock;

// use once_cell::sync::Lazy;
use surrealdb::{
    Result, Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

// pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn connect_db() -> Result<()> {
    let _ = DB.connect::<Ws>("127.0.0.1:8000").await?;
    let _ = DB
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await;
    let _ = DB.use_ns("todo").use_db("todo").await?;

    let some_queries = DB
        .query(
            "
        RETURN 9; 
        RETURN 10; 
        SELECT * FROM { is: 'Nice database' };
    ",
        )
        .await?;
    dbg!(some_queries);

    Ok(())
}
