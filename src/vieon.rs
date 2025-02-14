use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VieOnMeta {
    pub items: Vec<Item>,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub group_id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub title: String,
    pub slug: String,
    pub resolution: i64,
    pub images: Images,
    pub avg_rate: f64,
    pub movie: Movie,
    pub total_rate: i64,
    pub is_premium: i64,
    pub is_new: i64,
    pub is_trailer: i64,
    pub episode: i64,
    pub current_episode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Images {
    pub thumbnail_hot_v4: String,
    pub thumbnail_big_v4: String,
    pub carousel_tv_v4: String,
    pub carousel_app_v4: String,
    pub carousel_web_v4: String,
    pub thumbnail_v4: String,
    pub poster_v4: String,
    pub promotion_banner: String,
    pub title_card_light: String,
    pub title_card_dark: String,
    pub poster_original: String,
    pub thumb_original: String,
    pub thumbnail_hot_v4_ntc: String,
    pub thumbnail_big_v4_ntc: String,
    pub carousel_tv_v4_ntc: String,
    pub carousel_app_v4_ntc: String,
    pub carousel_web_v4_ntc: String,
    pub thumbnail_v4_ntc: String,
    pub poster_v4_ntc: String,
    pub ribbon_detail_app: String,
    pub ribbon_detail_web: String,
    pub promotion_banner_sm: String,
    pub poster_v4_original: String,
    pub poster_v4_tablet: String,
    pub thumbnail_v4_tablet: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub user_id: String,
    pub total: i64,
    pub limit: i64,
    pub page: i64,
}

impl VieOnMeta {
    pub(crate) fn get_info(&self) -> (String, String) {
        let last_item = match self.items.last() {
            Some(item) => item,
            None => return ("".to_string(), "".to_string()),
        };
        let full_title = format!("{} {}", last_item.movie.title, last_item.title);
        let image = last_item.images.thumbnail_v4_ntc.clone();
        (full_title, image)
    }
}

pub(crate) async fn get_source_list(slug: &str) -> Result<VieOnMeta> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse()?);
    headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36".parse()?);
    headers
        .insert("Content-Type", "application/x-www-form-urlencoded".parse()?);

    let mut params = std::collections::HashMap::new();
    params.insert("entity_slug",slug );
    params.insert("platform", "web");
    params.insert("ui", "012021");

    let request = client.request(reqwest::Method::POST, "https://api.vieon.vn/backend/cm/v5/slug/episode?limit=50&platform=web&ui=012021")
        .headers(headers)
        .form(&params);

    let response = request.send().await?;
    let body = response.text().await?;

    let meta: VieOnMeta = serde_json::from_str(&body)?;

    Ok(meta)
}

