use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use orgize::Org;
use serde::Deserialize;
use std::env;
use std::io::Result;

#[get("/")]
fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        "<h3><a href=\"https://github.com/PoiScript/orgize\">Orgize</a> demos</h3>\
         <form action=\"/export\" method=\"post\">\
         <p>Input content:</p>\
         <div><textarea name=\"content\" rows=\"10\" cols=\"100\">* DONE Title :tag:</textarea></div>\
         <p>Output format:</p>\
         <input type=\"radio\" name=\"format\" value=\"json\" checked> Json<br>\
         <input type=\"radio\" name=\"format\" value=\"html\"> HTML<br>\
         <input type=\"radio\" name=\"format\" value=\"org\"> Org<br>\
         <p><input type=\"submit\" value=\"Submit\"></p>\
         </form>",
    )
}

#[derive(Deserialize)]
struct FormData {
    format: String,
    content: String,
}

#[post("/export")]
fn export(form: web::Form<FormData>) -> Result<HttpResponse> {
    let org = Org::parse(&form.content);
    match &*form.format {
        "json" => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&org)?)),
        "html" => {
            let mut writer = Vec::new();
            org.html(&mut writer)?;
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(String::from_utf8(writer).unwrap()))
        }
        "org" => {
            let mut writer = Vec::new();
            org.org(&mut writer)?;
            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(String::from_utf8(writer).unwrap()))
        }
        _ => Ok(HttpResponse::BadRequest().body("Unsupported format".to_string())),
    }
}

fn main() -> Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| App::new().service(index).service(export))
        .bind(("0.0.0.0", port))?
        .run()
}
