use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

mod option_date_serializer {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y%m%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Ok(s) = String::deserialize(deserializer) {
            NaiveDate::parse_from_str(&s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom)
        } else {
            Ok(None)
        }
    }
}

mod date_serializer {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y%m%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(default)]
    #[serde(with = "option_date_serializer")]
    pub happen_dt: Option<NaiveDate>,
    pub kind_cd: Option<String>,
    pub sex_cd: Option<String>,
    pub neuter_yn: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Dogs {
    pub desertion_no: String,
    pub filename: String,
    pub image_path: Option<String>,
    #[serde(with = "date_serializer")]
    pub happen_dt: NaiveDate,
    pub kind_cd: String,
    pub color_cd: String,
    pub age: String,
    pub weight: String,
    pub sex_cd: String,
    pub neuter_yn: String,
    pub care_nm: String,
    pub care_tel: String,
    pub care_addr: String,
    pub charge_nm: Option<String>,
    pub officetel: String,
    pub notice_comment: Option<String>,
}
