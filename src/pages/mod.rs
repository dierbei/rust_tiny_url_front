mod create;
mod links;

pub use links::Links;
pub use create::Create;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
struct TinyData<T> {
    pub data: T,
    pub ok: bool,
    pub err: Option<String>,
}