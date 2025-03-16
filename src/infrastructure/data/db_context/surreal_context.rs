use std::{sync::LazyLock, time::Duration};
use surrealdb::{
    Result, Surreal,
    engine::remote::ws::{Client, Ws},
    opt::{Config, auth::Root},
};

use crate::settings::DbConfig;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
pub async fn connect_db(settings: DbConfig) -> Result<()> {
    let config = Config::default().query_timeout(Duration::from_millis(30000));

    DB
        .connect::<Ws>((format!("{}:{}", settings.host, settings.port), config))
        .await?;
    let _ = DB
        .signin(Root {
            username: &settings.username,
            password: &settings.password,
        })
        .await;
    DB
        .use_ns(settings.namespace)
        .use_db(settings.db_name)
        .await?;

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
