//! HTTP katmani. Her handler bir `async fn` - Gun 9 Lesson 1.
//!
//! axum her istegi ayri bir TASK'ta calistirir. Thread degil.
//! Bu yuzden paylasilan durum Arc<Mutex<T>> icinde.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::Arc;

use crate::depo::Depo;
use crate::kitap::{KitapHatasi, YeniKitap};

/// Tum handler'larin paylastigi durum.
/// `dyn Depo` - hangi depo oldugunu bilmiyoruz.
pub type Durum = Arc<dyn Depo>;

/// Hatayi HTTP durum koduna cevir.
/// Gun 4'teki match'in pratik karsiligi: her hata varyanti farkli davranis.
fn hataya_cevir(h: KitapHatasi) -> (StatusCode, Json<serde_json::Value>) {
    let kod = match h {
        KitapHatasi::Bulunamadi(_) => StatusCode::NOT_FOUND,
        KitapHatasi::BosBaslik | KitapHatasi::GecersizFiyat(_) => StatusCode::BAD_REQUEST,
        KitapHatasi::YetersizStok { .. } => StatusCode::CONFLICT,
    };
    (kod, Json(json!({ "hata": h.to_string() })))
}

/// GET /kitaplar
pub async fn listele(State(depo): State<Durum>) -> impl IntoResponse {
    Json(depo.hepsi())
}

/// GET /kitaplar/:id
pub async fn getir(State(depo): State<Durum>, Path(id): Path<u32>) -> impl IntoResponse {
    match depo.bul(id) {
        Ok(k) => (StatusCode::OK, Json(json!(k))),
        Err(e) => hataya_cevir(e),
    }
}

/// POST /kitaplar
pub async fn ekle(State(depo): State<Durum>, Json(y): Json<YeniKitap>) -> impl IntoResponse {
    match depo.ekle(y) {
        Ok(k) => (StatusCode::CREATED, Json(json!(k))),
        Err(e) => hataya_cevir(e),
    }
}

/// DELETE /kitaplar/:id
pub async fn sil(State(depo): State<Durum>, Path(id): Path<u32>) -> impl IntoResponse {
    match depo.sil(id) {
        Ok(k) => (StatusCode::OK, Json(json!(k))),
        Err(e) => hataya_cevir(e),
    }
}

/// POST /kitaplar/:id/satis/:adet
pub async fn satis(
    State(depo): State<Durum>,
    Path((id, adet)): Path<(u32, u32)>,
) -> impl IntoResponse {
    match depo.satis(id, adet) {
        Ok(tutar) => (
            StatusCode::OK,
            Json(json!({ "id": id, "adet": adet, "tutar": tutar })),
        ),
        Err(e) => hataya_cevir(e),
    }
}
