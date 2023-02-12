use crate::dynamodb::DbSettings;
use aws_sdk_dynamodb::model::AttributeValue;
use chrono::{TimeZone, Utc};
use rocket::http::ContentType;
use serde::{Deserialize, Serialize};
use std::borrow::ToOwned;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub partition_key: String,
    pub sort_key: f64,
    pub resource: String,
}

#[derive(rocket::Responder)]
pub enum BabySchema {
    A(String, ContentType),
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![handler]
}

const PARTITION: &str = "Baby1";

#[rocket::get("/?<resource>")]
pub async fn handler(resource: String, db_settings: &rocket::State<DbSettings>) -> BabySchema {
    if resource != "check" {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let object = Object {
            partition_key: PARTITION.to_owned(),
            sort_key: since_the_epoch.as_secs_f64(),
            resource,
        };
        let values = serde_dynamo::to_item(object).unwrap();
        crate::dynamodb::put_item(&db_settings.client, &db_settings.table_name, values)
            .await
            .unwrap();
    }

    let response = db_settings
        .client
        .query()
        .table_name(&db_settings.table_name)
        .key_condition_expression("#partition_key = :valueToMatch")
        .expression_attribute_names("#partition_key", "partition_key")
        .expression_attribute_values(":valueToMatch", AttributeValue::S(PARTITION.to_owned()))
        .limit(20)
        .scan_index_forward(false)
        .send()
        .await
        .unwrap();
    let items = response.items().unwrap();

    let mut html = "<!DOCTYPE html>
<html>
  <head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
	  <title>Baby Schema</title>
  </head>
  <body>
  <section class=\"section\">
    <div class=\"container\">
      <h1 class=\"title\">Baby Schema</h1>\
      <table>"
        .to_owned();
    for item in items {
        let sort_key = item.get("sort_key").unwrap().as_n().unwrap();
        let epoch_seconds: f64 = sort_key.parse().unwrap();
        #[allow(clippy::cast_possible_truncation)]
        let utc = Utc
            .timestamp_opt(epoch_seconds.round() as i64, 0u32)
            .unwrap();
        let local_time = utc.with_timezone(&chrono_tz::Europe::Amsterdam);
        let resource = item.get("resource").unwrap().as_s().unwrap();
        html.push_str(
            format!(
                "<tr><td>{}</td><td>{}</td></tr>",
                local_time.format("%a %b %d %T"),
                resource
            )
            .as_str(),
        );
    }

    html.push_str("</table></div></section></body></html>");
    BabySchema::A(html, ContentType::HTML)
}
