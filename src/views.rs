use actix_web::http::ContentEncoding;
use actix_web::{HttpRequest, HttpResponse};
use app::AppState;
use tera;

/*
struct Table;

struct Column;

struct CharField;

struct IntergerField;

struct Product {
    gg: f64,
}

impl Product {
    fn init_field() -> Table {
        Table::new(
            Column::new("id", IntegerField),
            Column::new("name", CharField),
        )
    }
}
*/

pub fn index(req: &HttpRequest<AppState>) -> HttpResponse {
    let template = req.state().template.borrow();
    let mut context = tera::Context::new();
    context.insert("vat_rate", &0.20);
    let result = template.render("__base.html", &context).unwrap();
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html; charset=utf-8")
        .body(result)
}

pub fn login(req: &HttpRequest<AppState>) -> HttpResponse {
    let template = req.state().template.borrow();
    let mut context = tera::Context::new();
    context.insert("vat_rate", &0.20);
    let result = template.render("admin/login.html", &context).unwrap();
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html; charset=utf-8")
        .body(result)
}
