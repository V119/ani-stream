use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Serialize, Clone, Debug)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct Item {
    name: String,
    image_url: String,
    status: String,
    page_url: String,
}

