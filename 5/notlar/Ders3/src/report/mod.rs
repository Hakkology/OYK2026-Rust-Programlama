//! report modulunun koku. Iki alt modulu var: summary ve table.

pub mod summary;
pub mod table;

// Alt modullerden gelenleri bu seviyede yeniden yayinliyoruz
pub use summary::summary;
pub use table::table;
