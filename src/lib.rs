use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Json, Request, StatusCode};
use spin_sdk::http_service;
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Country {
    name: String,
    code: String,
    phone_prefix: String,
}

static COUNTRIES: OnceLock<BTreeMap<String, Country>> = OnceLock::new();

wit_bindgen::generate!({
    inline: "
        package thorstenhans:wizer;

        world wizer-init {
            export wizer-initialize: func();
        }
    ",
});

struct Init;

impl Guest for Init {
    fn wizer_initialize() {
        let raw = include_str!("../data/countries.json");
        let list: Vec<Country> = serde_json::from_str(raw).expect("failed to parse country dataset");

        let mut map = BTreeMap::new();
        for country in list {
            map.insert(country.code.to_uppercase(), country);
        }

        COUNTRIES.set(map).expect("COUNTRIES already initialized");
    }
}

export!(Init);

#[http_service]
async fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    let countries = COUNTRIES.get().expect("countries not initialized");

    let code = req.uri().path().trim_start_matches('/').to_uppercase();

    match countries.get(&code) {
        Some(country) => Ok(Json(country).into_response()),
        None => Ok((StatusCode::NOT_FOUND, String::from("Country not found")).into_response()),
    }
}
