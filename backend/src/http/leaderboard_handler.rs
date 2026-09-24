use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api_error::ApiError;
use crate::auth::middleware::ClaimsExt;
use crate::models::{CloseSeasonRequest, PaginatedResponse, PaginationParams};
use crate::service::LeaderboardService;

fn require_admin(req: &HttpRequest) -> Result<(), ApiError> {
    let claims = req
        .claims()
        .ok_or_else(|| ApiError::unauthorized("Authentication required"))?;
    if claims.roles.contains(&"admin".to_string()) {
        Ok(())
    } else {
        Err(ApiError::forbidden("ROLE_ADMIN required"))
    }
}

/// GET /api/v1/leaderboards/:category
pub async fn get_leaderboard(
    pool: web::Data<PgPool>,
    category: web::Path<String>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiError> {
    let service = LeaderboardService::new(pool.get_ref().clone());
    let limit = query.resolved_limit();
    let offset = query.sql_offset();

    let leaderboard = service
        .get_leaderboard(&category, limit, offset)
        .await?;

    Ok(HttpResponse::Ok().json(PaginatedResponse {
        total: leaderboard.total_count,
        page: query.resolved_page(),
        limit,
        data: leaderboard.entries,
    }))
}

/// GET /api/v1/leaderboards/:category/season/:season
pub async fn get_seasonal_leaderboard(
    pool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiError> {
    let (category, season) = path.into_inner();
    let service = LeaderboardService::new(pool.get_ref().clone());
    let limit = query.resolved_limit();
    let offset = query.sql_offset();

    let leaderboard = service
        .get_seasonal_leaderboard(&category, &season, limit, offset)
        .await?;

    Ok(HttpResponse::Ok().json(PaginatedResponse {
        total: leaderboard.total_participants,
        page: query.resolved_page(),
        limit,
        data: leaderboard.entries,
    }))
}

/// GET /api/v1/leaderboards/:category/player/:player_id
pub async fn get_player_rank(
    pool: web::Data<PgPool>,
    path: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (category, player_id) = path.into_inner();
    let service = LeaderboardService::new(pool.get_ref().clone());

    let player_rank = service.get_player_rank(&category, player_id).await?;

    Ok(HttpResponse::Ok().json(player_rank))
}

/// GET /api/v1/leaderboards/:category/history/:player_id
pub async fn get_rank_history(
    pool: web::Data<PgPool>,
    path: web::Path<(String, Uuid)>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let (category, player_id) = path.into_inner();
    let service = LeaderboardService::new(pool.get_ref().clone());
    let days = query
        .get("days")
        .and_then(|d| d.parse::<i64>().ok())
        .unwrap_or(30);

    let history = service
        .get_rank_history(player_id, &category, days)
        .await?;

    Ok(HttpResponse::Ok().json(history))
}

/// POST /api/v1/leaderboards/:category/refresh
pub async fn refresh_leaderboard(
    pool: web::Data<PgPool>,
    category: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let service = LeaderboardService::new(pool.get_ref().clone());

    service.refresh_leaderboard(&category).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Leaderboard for {} refreshed successfully", category)
    })))
}

/// GET /api/v1/leaderboards/:category/stats
pub async fn get_leaderboard_stats(
    pool: web::Data<PgPool>,
    category: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let service = LeaderboardService::new(pool.get_ref().clone());

    let stats = service.get_leaderboard_stats(&category).await?;

    Ok(HttpResponse::Ok().json(stats))
}

// ─── Season close (#1075) ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SeasonHistoryQuery {
    pub season_id: Uuid,
}

/// POST /api/admin/seasons/{id}/close — admin-only (ROLE_ADMIN).
pub async fn close_season(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: Option<web::Json<CloseSeasonRequest>>,
) -> Result<HttpResponse, ApiError> {
    require_admin(&req)?;
    let season_id = path.into_inner();
    let top_n = body.and_then(|b| b.top_n).unwrap_or(100);

    let service = LeaderboardService::new(pool.get_ref().clone());
    let result = service.close_season(season_id, top_n).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/leaderboard/history?season_id=X
pub async fn get_season_history(
    pool: web::Data<PgPool>,
    query: web::Query<SeasonHistoryQuery>,
) -> Result<HttpResponse, ApiError> {
    let service = LeaderboardService::new(pool.get_ref().clone());
    let snapshots = service.get_season_history(query.season_id).await?;

    Ok(HttpResponse::Ok().json(snapshots))
}

/// Registers this module's season-close endpoints. Call inside the `/api`
/// scope alongside the other `.configure(...)` handlers.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/seasons").route("/{id}/close", web::post().to(close_season)),
    )
    .service(
        web::scope("/leaderboard").route("/history", web::get().to(get_season_history)),
    );
}
