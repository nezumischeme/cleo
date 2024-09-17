use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageLinks {
    #[serde(rename = "smallThumbnail")]
    pub small_thumbnail: Option<String>,
    pub thumbnail: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    #[serde(rename = "extraLarge")]
    pub extra_large: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeInfo {
    pub title: String,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub publisher: Option<String>,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,
    #[serde(rename = "imageLinks")]
    pub image_links: Option<ImageLinks>,
    #[serde(rename = "mainCategory")]
    pub main_category: Option<String>,
    #[serde(rename = "pageCount")]
    pub page_count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    pub kind: String,
    pub id: String,
    pub etag: String,
    #[serde(rename = "selfLink")]
    pub self_link: String,
    #[serde(rename = "volumeInfo")]
    pub volume_info: VolumeInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volumes {
    pub kind: String,
    pub items: Vec<Volume>,
}

pub fn highest_quality_image(images: &Option<ImageLinks>) -> Option<String> {
    if let Some(igs) = images.clone() {
        match igs {
            ImageLinks {
                extra_large: Some(x),
                ..
            } => Some(x),
            ImageLinks { large: Some(x), .. } => Some(x),
            ImageLinks {
                medium: Some(x), ..
            } => Some(x),
            ImageLinks { small: Some(x), .. } => Some(x),
            ImageLinks {
                thumbnail: Some(x), ..
            } => Some(x),
            ImageLinks {
                small_thumbnail: Some(x),
                ..
            } => Some(x),
            _ => None,
        }
    } else {
        None
    }
}

pub async fn search_volumes(q: String, api_key: String) -> Result<Volumes, reqwest::Error> {
    let url = Url::parse_with_params(
        "https://www.googleapis.com/books/v1/volumes",
        &[("q", q), ("key", api_key)],
    )
    .expect("Could not parse this url!");
    let response = reqwest::get(url).await.expect("Could not query API");
    let volumes = response.json::<Volumes>().await;

    match volumes {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}
