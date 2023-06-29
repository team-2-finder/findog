use std::sync::OnceLock;

use anyhow::Result;
use regex::Regex;
use reqwest::header::USER_AGENT;
use reqwest::Response;
use sqlx::types::JsonValue;
use sqlx::{Pool, Postgres};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::entity::Dogs;

const URL: &str = "http://apis.data.go.kr/1543061/abandonmentPublicSrvc/abandonmentPublic?bgnde=20211201&4upkind=417000&endde=20231231&_type=json&pageNo=1&numOfRows=1000&serviceKey=";

fn time() -> &'static Mutex<()> {
    static TIME: OnceLock<Mutex<()>> = OnceLock::new();
    TIME.get_or_init(|| Mutex::new(()))
}

fn regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r#"_s(\[\d])?"#).unwrap())
}

async fn put_dog(mut dog: Dogs, pool: Pool<Postgres>, base_path: String) -> Result<()> {
    let image = get(&dog.filename).await?;
    tokio::fs::create_dir_all(&base_path).await?;
    let ext = dog.filename.split('.').last().unwrap_or("jpg");
    let path = format!("{base_path}/{id}.{ext}", id = dog.desertion_no);
    let mut file = File::create(&path).await?;
    let mut image = image.bytes().await?;
    file.write_all_buf(&mut image).await?;

    dog.image_path = Some(path);

    sqlx::query(
        "
            insert into dogs (
                desertion_no, filename, image_path, happen_dt, kind_cd, color_cd, age, weight, sex_cd, neuter_yn, care_nm, care_tel, care_addr, charge_nm, officetel, notice_comment
            ) values (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            ) on conflict do nothing"
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

async fn get(url: &str) -> Result<Response> {
    let _time = time().lock().await;
    sleep(std::time::Duration::from_millis(50)).await;

    let client = reqwest::Client::new();
    Ok(client.get(url).header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15").send().await?)
}

pub async fn fetch_dogs(pool: Pool<Postgres>) -> Result<()> {
    let base_path = std::env::var("IMAGE_BASE").unwrap_or_else(|_| "./images".to_string());
    let key = std::env::var("API_KEY")?;
    let resp = get(&format!("{URL}{key}")).await?;
    let text = resp.text().await?;
    let dogs: JsonValue = serde_json::from_str(&text)?;
    let dogs = dogs["response"]["body"]["items"]["item"]
        .as_array()
        .unwrap();

    tracing::info!("fetching {} results", dogs.len());

    let tasks = dogs
        .iter()
        .filter(|dog| dog.get("kindCd").unwrap().as_str().unwrap().contains('개'))
        .map(|dog| {
            let mut dog: Dogs = serde_json::from_value(dog.clone()).unwrap();
            dog.filename = regex().replace(&dog.filename, "").to_string();
            let pool = pool.clone();
            let base_path = base_path.clone();

            tokio::spawn(put_dog(dog, pool, base_path))
        })
        .collect::<Vec<_>>();

    for task in tasks {
        if let Err(e) = task.await? {
            tracing::info!("error occurred: {e}");
        }
    }

    tracing::info!("done fetching dogs");

    Ok(())
}
