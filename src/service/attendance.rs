// 
use sqlx::{FromRow, 
    // postgres::PgRow
};

use crate::corel::db;

use crate::controllers::UserCode;

// 

// #[derive(Debug, FromRow)]
// pub struct User {user_code: Option<String>}

pub async fn get_name() -> Result<Vec<UserCode>, sqlx::Error> {
    println!("DAO 层 get_name()");
    let pools = db::db_pool();
    if let Some(pool) = pools {
        Ok(sqlx::query_as!(UserCode, r#"SELECT user_code from public.ad_user"#,).fetch_all(pool).await?)
    } else {
        Ok(vec![ UserCode {user_code: None}])
    }

}

// pub async fn new_user() -> Result<(), Box<dyn std::error::Error>> {
//     let pools = db::db_pool();
//     if let Some(pool) = pools {
//         sqlx::query!(r#"insert into ad_user (user_code) values ($1)"#, "jiangkun").execute(pool).await?
//     } else {
//         Ok(())
//     }
// }


pub async fn new_user(name: &String) -> anyhow::Result<i64> {

    let pools = db::db_pool();
    if let Some(pool) = pools {
        let rec = sqlx::query!(
            r#"
            INSERT INTO ad_user ( user_code )
            VALUES ( $1 )
            "#,
            name
        )
        .fetch_one(pool)
        .await?;
        println!("{:#?}", rec);
        Ok(1)
    } else {
        Ok(0)
    }

}