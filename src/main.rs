use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use log::info;
use serde::{Deserialize, Serialize};
use shakmaty::{fen::Fen, CastlingMode, Chess, Move, Position};
use std::env;
use urlencoding::decode;

#[derive(Serialize, Deserialize, Debug)]
struct SerializableMove {
    role: String,
    from: String,
    capture: Option<String>,
    to: String,
    promotion: Option<String>,
}

impl From<&Move> for SerializableMove {
    fn from(m: &Move) -> Self {
        match m {
            Move::Normal {
                role,
                from,
                capture,
                to,
                promotion,
            } => SerializableMove {
                role: format!("{:?}", role),
                from: format!("{:?}", from),
                capture: capture.as_ref().map(|c| format!("{:?}", c)),
                to: format!("{:?}", to),
                promotion: promotion.as_ref().map(|p| format!("{:?}", p)),
            },
            Move::EnPassant { from, to } => SerializableMove {
                role: "Pawn".to_string(),
                from: format!("{:?}", from),
                capture: Some("Pawn".to_string()),
                to: format!("{:?}", to),
                promotion: None,
            },
            Move::Castle { king, rook } => SerializableMove {
                role: "King".to_string(),
                from: format!("{:?}", king),
                capture: None,
                to: format!("{:?}", rook),
                promotion: None,
            },
            Move::Put { role, to } => SerializableMove {
                role: format!("{:?}", role),
                from: "Hand".to_string(),
                capture: None,
                to: format!("{:?}", to),
                promotion: None,
            },
        }
    }
}

const README: &str = include_str!("../README.md");

#[get("/standard/{fen}")]
async fn standard(path: web::Path<String>) -> Result<HttpResponse> {
    let fen_encoded = path.into_inner();
    let fen_decoded = match decode(&fen_encoded) {
        Ok(fen) => fen.replace('+', " "),
        Err(_) => return Ok(HttpResponse::BadRequest().body("Failed to decode URL")),
    };

    let fen: Fen = match fen_decoded.parse() {
        Ok(fen) => fen,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid FEN")),
    };

    let pos: Chess = match fen.into_position(CastlingMode::Standard) {
        Ok(pos) => pos,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid FEN")),
    };

    let legal_moves: Vec<SerializableMove> = pos
        .legal_moves()
        .iter()
        .map(SerializableMove::from)
        .collect();

    info!("Returning legal moves for FEN: {}", fen_decoded);
    Ok(HttpResponse::Ok().json(legal_moves))
}

#[get("/")]
async fn readme() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/plain").body(README))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(standard)
            .service(readme)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_standard() {
        let app = test::init_service(App::new().service(standard)).await;
        let req = test::TestRequest::get().uri("/standard/rnbqkbnr%2Fpppppppp%2F8%2F8%2F8%2F8%2FPPPPPPPP%2FRNBQKBNR%20w%20KQkq%20-%200%201").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_readme() {
        let app = test::init_service(App::new().service(readme)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
