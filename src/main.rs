#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
extern crate globwalk;

use rocket::response::status;
use rocket::response::content;
use tera::Tera;
use std::path::Path;

mod util;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Templates;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        let t = Templates::iter().map(|filename| {
            let p = filename;
            let ttext = Templates::get(&p).unwrap().into_owned();
            (p.into_owned(), std::str::from_utf8(&ttext).unwrap().to_owned())
        }).collect::<Vec<(_,_)>>();

        tera.add_raw_templates(t).unwrap();
        tera
    };
}

/// Root Folder
#[get("/")]
fn index() -> content::Html<String> {
    // Get Manga
    let manga = util::get_manga(Path::new("/home/fumon/tmp/manga_s/T"));

    let mut context = tera::Context::new();
    context.insert("manga", &vec!["apple", "kotabaru", "san", "samar", "sonic", "blah"]);
    content::Html(TEMPLATES.render("index.html.tera", &context).unwrap())
}

/// TODO: Handle grabbing images from manga folders

/// Embedded css supplier
#[get("/theme.css")]
fn css() -> content::Css<String> {
    // TODO: generalize to any embedded resource
    let css = Asset::get("theme.css").unwrap().into_owned();
    content::Css(std::str::from_utf8(&css).unwrap().to_owned())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, css])
        .launch();
}
