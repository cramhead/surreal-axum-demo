use std::{sync::LazyLock, time::Duration};

use surrealdb::{
    engine::remote::ws::{Client, Ws}, opt::{auth::Root, Config}, Result, Surreal
};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn connect_db() -> Result<()> {

let config = Config::default().query_timeout(Duration::from_millis(30000));
    let _ = DB.connect::<Ws>((format!("{}:{}", "127.0.0.1", 8000), config)).await?;
    let _ = DB
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await;
    let _ = DB.use_ns("todo").use_db("todo").await?;

    // check db is live
    // let some_queries = DB
    //     .query(
    //         "
    //     RETURN 9; 
    //     RETURN 10; 
    //     SELECT * FROM { is: 'Nice database' };
    // ",
    //     )
    //     .await?;
    // dbg!(some_queries);

    Ok(())
}
