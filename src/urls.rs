use actix_web::App;
use app::AppState;
use views::{index, login};

pub fn urls_pattern(app: App<AppState>) -> App<AppState> {
    app
        .resource("/", |r| r.f(index))
        .resource("/test/", |r| r.f(index))
        .resource("/admin/login/", |r| r.f(login))
}