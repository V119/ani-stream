use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Serialize, Clone, Debug)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct Item {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub desc: String,
    pub page_url: String,
}


#[derive(TS, Serialize, Clone, Debug)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct MainPage {
    pub items: Vec<Item>,
    pub pre_page: Option<String>,
    pub next_page: Option<String>,
}

#[derive(TS, Serialize, Clone, Debug)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct ItemPage {
    pub items: Vec<Item>,
    pub pre_page: Option<String>,
    pub next_page: Option<String>,
}