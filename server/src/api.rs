use anyhow::Result;
use sqlx::types::JsonValue;
use sqlx::{Pool, Postgres};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::entity::Dogs;

const URL: &str = "https://apis.data.go.kr/1543061/abandonmentPublicSrvc/abandonmentPublic?bgnde=20211201&4upkind=417000&endde=20231231&_type=json&pageNo=1&numOfRows=1000&serviceKey=";

async fn put_dog(mut dog: Dogs, pool: Pool<Postgres>, base_path: String) -> Result<()> {
    let image = reqwest::get(&dog.filename).await?;
    tokio::fs::create_dir_all(&base_path).await?;
    let ext = dog.filename.split('.').last().unwrap_or("jpg");
    let path = format!("{base_path}/{id}.{ext}", id = dog.desertion_no);
    let mut file = File::create(&path).await?;
    let mut image = image.bytes().await?;
    file.write_all_buf(&mut image).await?;

    tracing::info!("path: {:?}", path);

    dog.image_path = Some(path);

    sqlx::query(
        "
            insert into dogs (
                desertion_no, filename, image_path, happen_dt, kind_cd, color_cd, age, weight, sex_cd, neuter_yn, care_nm, care_tel, care_addr, charge_nm, officetel, notice_comment
            ) values (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            )"
    )
        .bind(&dog.desertion_no)
        .bind(&dog.filename)
        .bind(&dog.image_path)
        .bind(dog.happen_dt)
        .bind(&dog.kind_cd)
        .bind(&dog.color_cd)
        .bind(dog.age)
        .bind(dog.weight)
        .bind(&dog.sex_cd)
        .bind(&dog.neuter_yn)
        .bind(&dog.care_nm)
        .bind(&dog.care_tel)
        .bind(&dog.care_addr)
        .bind(&dog.charge_nm)
        .bind(&dog.officetel)
        .bind(&dog.notice_comment)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn fetch_dogs(pool: Pool<Postgres>) -> Result<()> {
    let base_path = std::env::var("IMAGE_BASE").unwrap_or_else(|_| "./images".to_string());
    let key = std::env::var("API_KEY")?;
    let resp = reqwest::get(format!("{URL}{key}")).await?;
    let text = resp.text().await?;
    let dogs: JsonValue = serde_json::from_str(&text)?;
    let dogs = dogs["response"]["body"]["items"]["item"]
        .as_array()
        .unwrap();

    let tasks = dogs
        .iter()
        .map(|dog| {
            tracing::info!("dog: {:?}", dog);
            let dog = serde_json::from_value(dog.clone()).unwrap();
            let pool = pool.clone();
            let base_path = base_path.clone();

            tokio::spawn(put_dog(dog, pool, base_path))
        })
        .collect::<Vec<_>>();

    for task in tasks {
        task.await??;
    }

    Ok(())
}
