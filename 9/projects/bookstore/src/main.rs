//! Bookstore - basit bir kitap API'si.
//!
//! Kampta ogrenilenlerin toplandigi yer:
//!   kitap.rs  Gun 4 struct + enum, Gun 5 hata tipi
//!   depo.rs   Gun 7 trait + dyn, Gun 9 Arc<Mutex<T>>
//!   api.rs    Gun 9 async fn, tokio task'lari
//!
//! cargo run    -> http://127.0.0.1:3000
//! cargo test

mod api;
mod depo;
mod kitap;

use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;

use depo::{BellekDepo, Depo};

#[tokio::main]
async fn main() {
    // Arc<dyn Depo> - hangi depo oldugunu API katmani bilmiyor.
    // Yarin veritabani yazsaniz sadece bu satir degisir.
    let depo: Arc<dyn Depo> = Arc::new(BellekDepo::ornek_veriyle());

    let uygulama = Router::new()
        .route("/kitaplar", get(api::listele))
        .route("/kitaplar", post(api::ekle))
        .route("/kitaplar/:id", get(api::getir))
        .route("/kitaplar/:id", delete(api::sil))
        .route("/kitaplar/:id/satis/:adet", post(api::satis))
        .with_state(depo);

    let adres = "127.0.0.1:3000";
    let dinleyici = tokio::net::TcpListener::bind(adres).await.unwrap();

    println!("Bookstore API calisiyor: http://{}", adres);
    println!();
    println!("  curl http://127.0.0.1:3000/kitaplar");
    println!("  curl http://127.0.0.1:3000/kitaplar/1");
    println!("  curl http://127.0.0.1:3000/kitaplar/99");
    println!("  curl -X POST http://127.0.0.1:3000/kitaplar/1/satis/3");
    println!("  curl -X POST http://127.0.0.1:3000/kitaplar/3/satis/1");
    println!("  curl -X POST http://127.0.0.1:3000/kitaplar \\");
    println!("       -H 'Content-Type: application/json' \\");
    println!("       -d '{{\"baslik\":\"Yeni\",\"yazar\":\"X\",\"fiyat\":100,\"stok\":5}}'");
    println!();

    axum::serve(dinleyici, uygulama).await.unwrap();
}
