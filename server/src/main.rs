use std::{env, fs, time::Instant};

use actix::*;
use actix_files::NamedFile;
use actix_web::{
    get, http::header::ContentEncoding, middleware::Logger, post, web, App, Error, HttpRequest, HttpResponse, HttpServer
};
use actix_web_actors::ws;
use serde::Deserialize;
use dotenv::dotenv;

mod server;
mod ship;
mod keystate;
mod bullet;
mod session;

#[derive(Debug, Deserialize)]
pub struct RoomRequest {
    id: usize,
    watch: bool,
}

async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::GameServer>>,
    query: web::Query<RoomRequest>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::GameSession {
            id: 0,
            hb: Instant::now(),
            room: query.id,
            addr: srv.get_ref().clone(),
            watch: query.watch,
        },
        &req,
        stream,
    )
}

#[get("/")]
async fn lobby(_req: HttpRequest, srv: web::Data<Addr<server::GameServer>>) -> HttpResponse {
    let rooms = srv.send(server::ListRooms).await.unwrap();
    let mut rooms_html = String::new();

    for room in rooms {
        let player_count = srv.send(server::GetPlayerCount {
            room_id: room,
        }).await.unwrap();
        let is_playing = srv.send(server::IsPlaying {
            room_id: room,
        }).await.unwrap();

        rooms_html += "<div class=\"room\">";
        rooms_html += "<div class=\"information\">";
        rooms_html += format!("<p class=\"room-id\">{:04}</p>", room).as_str();
        rooms_html += if is_playing {
            "<p class=\"room-condition playing\">Playing</p>"
        } else {
            "<p class=\"room-condition recruiting\">Recruiting</p>"
        };
        rooms_html += format!("<p class=\"player-num\">{}/4</p>", player_count).as_str();
        rooms_html += "</div>";
        rooms_html += "<div class=\"buttons\">";
        rooms_html += format!("<a href=\"/game?id={}&watch=true\">Watch</a>", room).as_str();
        if player_count < 4 {
            if is_playing {
                rooms_html += "<a href=\"#\" class=\"full\">Join</a>"
            } else {
                rooms_html += format!("<a href=\"/game?id={}&watch=false\">Join</a>", room).as_str();
            }
        } else {
            rooms_html += "<a href\"#\" class=\"full\">Join</a>";
        }
        rooms_html += "</div>";
        rooms_html += "</div>";
    }

    if let Ok(content) = fs::read_to_string("./static/index.html") {
        let content = content.replace("{server_replace}", &rooms_html);

        HttpResponse::Ok()
            .content_type("text/html; charset=UTF-8")
            .body(content)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/game")]
async fn join_game(_req: HttpRequest, _srv: web::Data<Addr<server::GameServer>>) -> actix_web::Result<NamedFile> {
    let use_ssl = env::var("USE_SSL").unwrap_or("false".to_string())
        .parse()
        .expect("The value of USE_SSL is invalid");

    if use_ssl {
        Ok(NamedFile::open("./static/game_wss.html")?)
    } else {
        Ok(NamedFile::open("./static/game_ws.html")?)
    }
}

#[get("/static/{filename}")]
async fn static_file(info: web::Path<String>) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(format!("./static/{}", info.into_inner()))?)
}

#[get("/rooms")]
async fn get_rooms(_req: HttpRequest, srv: web::Data<Addr<server::GameServer>>) -> HttpResponse {
    let mut rooms = srv.send(server::ListRooms).await.unwrap();
    rooms.sort();
    HttpResponse::Ok().body(format!("{rooms:?}"))
}

#[post("/rooms")]
async fn create_room(_req: HttpRequest, srv: web::Data<Addr<server::GameServer>>) -> HttpResponse {
    let room_id = srv.send(server::CreateRoom).await.unwrap();
    HttpResponse::Created().body(format!("{room_id}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or("8080".to_string())
        .parse()
        .expect("The value of PORT is invalid");

    let server = server::GameServer::new().start();

    log::info!("Starting shooting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(ws_route))
            .service(lobby)
            .service(join_game)
            .service(static_file)
            .service(get_rooms)
            .service(create_room)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
