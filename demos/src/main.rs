use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use orgize::Org;
use serde::Deserialize;
use std::env;
use std::io::Result;

use orgize::export::html::{DefaultHtmlHandler, SyntectHtmlHandler};
use orgize::syntect::html::IncludeBackground;

#[get("/")]
fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<h3><a href="https://github.com/PoiScript/orgize">Orgize</a> demos</h3>
<form action="/export" method="post">
    <p>Input content:</p>
    <div>
        <textarea name="content" rows="10" cols="100">
* DONE Title :tag:

#+BEGIN_SRC rust
println!("Hello");
#+END_SRC
        </textarea>
    </div>
    <p>Output format:</p>
        <input type="radio" name="format" value="json" checked> Json<br>
        <input type="radio" name="format" value="html"> HTML<br>
        <input type="radio" name="format" value="org"> Org<br>
        <input type="radio" name="format" value="syntect"> HTML with highlight<br>
    <p>Highlight theme:</p>
    <select name="theme">
        <option value="InspiredGitHub" selected="selected">InspiredGitHub</option>
        <option value="base16-ocean.dark">base16-ocean.dark</option>
        <option value="base16-eighties.dark">base16-eighties.dark</option>
        <option value="base16-mocha.dark">base16-mocha.dark</option>
        <option value="base16-ocean.light">base16-ocean.light</option>
        <option value="Solarized (dark)">Solarized (dark)</option>
        <option value="Solarized (light)">Solarized (light)</option>
    </select>
    <p>Highlight background:</p>
    <select name="background">
        <option value="no" selected="selected">No</option>
        <option value="yes">Yes</option>
    </select>
    <p><input type="submit" value="Submit"></p>
</form>"#,
    )
}

#[derive(Deserialize)]
struct FormData {
    format: String,
    content: String,
    theme: String,
    background: String,
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
        "syntect" => {
            let mut handler = SyntectHtmlHandler::new(DefaultHtmlHandler);

            match &*form.theme {
                "InspiredGitHub"
                | "base16-ocean.dark"
                | "base16-eighties.dark"
                | "base16-mocha.dark"
                | "base16-ocean.light"
                | "Solarized (dark)"
                | "Solarized (light)" => handler.theme = form.theme.clone(),
                _ => return Ok(HttpResponse::BadRequest().body("Unsupported theme".to_string())),
            }

            match &*form.background {
                "yes" => handler.background = IncludeBackground::Yes,
                "no" => handler.background = IncludeBackground::No,
                _ => {
                    return Ok(HttpResponse::BadRequest().body("Unsupported background".to_string()))
                }
            }

            let mut writer = Vec::new();
            org.html_with_handler(&mut writer, &mut handler)?;

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
