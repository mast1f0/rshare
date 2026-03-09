use std::fs;
use std::io::Write;

use crate::other::qr;
use crate::other::state;
use axum::Router;
use axum::extract::{Multipart, State};
use axum::response::Redirect;
use axum::routing::post;
use axum::{response::Html, routing::get};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[tokio::main]
pub async fn run_srv(state: state::AppState) {
    let shared_dir = state.app_dir.join("shared");
    let templates_dir = state.app_dir.join("templates");
    let qrcode_path = state.app_dir.join("qrcode.png");

    for file in state.files.iter() {
        let file_name = file.file_name().unwrap();
        let new_path = shared_dir.join(file_name);

        fs::copy(file, new_path).unwrap();
    }
    let serve_dir = ServeDir::new(&shared_dir)
        .not_found_service(ServeFile::new(templates_dir.join("404.html")));

    let app = Router::new()
        .route("/", get(qr_handler))
        .nest_service("/shared", serve_dir)
        .nest_service("/qrcode.png", ServeFile::new(qrcode_path))
        .route("/gallery", get(gallery_handler))
        .route("/upload", post(upload_handler))
        .with_state(state.clone());

    let port: u16 = portpicker::pick_unused_port().unwrap_or(3001);
    println!("Server is listening on http://localhost:{}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    qr::create_qr(port, &state.app_dir.join("qrcode.png"));
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap(); // добавить удаление файлов
}

async fn qr_handler(State(state): State<state::AppState>) -> Html<String> {
    let filepath = state.app_dir.join("templates/qr.html");
    let content = fs::read_to_string(&filepath).expect("cant parse template");
    Html(content)
}

async fn gallery_handler(State(state): State<state::AppState>) -> Html<String> {
    let mut html = String::new();
    let tpl = fs::read_to_string(state.app_dir.join("templates/gallery.html"))
        .expect("cant read template");
    html.push_str(&tpl);
    let shared = state.app_dir.join("shared");
    if let Ok(entries) = fs::read_dir(&shared) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let escaped_name = html_escape(&name);
            html.push_str(&format!(
                r#"<div class="file-wrapper"><h1>{}</h1><a href="/shared/{}">Скачать</a></div>"#,
                escaped_name, escaped_name
            ));
        }
    }
    Html(html)
}

async fn upload_handler(
    State(state): State<state::AppState>,
    mut multipart: Multipart,
) -> Redirect {
    let share = state.app_dir.join("shared");
    while let Some(field) = multipart.next_field().await.unwrap() {
        //пока только мелки файлы
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let mut created_file =
            fs::File::create(format!("{}/{}", share.display(), name)).expect("cant create file");
        let is_success = created_file.write(&data);
        match is_success {
            Ok(_) => {}
            Err(_) => {
                println!("CANT WRITE TO FILE")
            }
        }
    }

    Redirect::to("/gallery")
}
