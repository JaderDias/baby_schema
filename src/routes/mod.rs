use std::time::{SystemTime, UNIX_EPOCH};
use crate::dynamodb::DbSettings;
use aws_sdk_dynamodb::model::AttributeValue;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![handler]
}

#[rocket::get("/?<resource>")]
pub async fn handler(
    resource: String,
    db_settings: &rocket::State<DbSettings>,
) -> Option<String> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let partition =  since_the_epoch.subsec_nanos() as u64;
    let fields = std::collections::HashMap::from([
        ("resource".to_owned(), AttributeValue::S(resource)),
    ]);
    crate::dynamodb::put_item(
        &db_settings.client,
        &db_settings.table_name,
        partition,
        fields,
    )
        .await
        .unwrap();
    let response =  db_settings.client
        .scan()
        .table_name(&db_settings.table_name)
        .limit(20)
        .send()
        .await
        .unwrap();
    let items = response
        .items()
        .unwrap();

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
      <ul>".to_owned();
    for item in items {
        html.push_str(format!("<li>{:?}</li>", item).as_str());
    }

    html.push_str("</ul></div></section></body></html>");
    Some(html.to_owned())
}
