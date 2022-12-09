use serde_derive::Deserialize;
use serde_derive::Serialize;
// dc6zaTOxFJmzC
pub async fn get_trending_gifs(number: i32) -> Result<TrendingGifs, reqwest::Error> {
    reqwest::Client::new()
        .get("https://api.giphy.com/v1/gifs/trending")
        .query(&[
            ("api_key", "goXttWBt3z9zJPqoEckmMaf379lAHvEQ"),
            ("limit", &number.to_string()),
        ])
        .send()
        .await?
        .json()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingGifs {
    pub data: Vec<Daum>,
    pub pagination: Pagination,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub url: String,
    pub slug: String,
    #[serde(rename = "bitly_gif_url")]
    pub bitly_gif_url: String,
    #[serde(rename = "bitly_url")]
    pub bitly_url: String,
    #[serde(rename = "embed_url")]
    pub embed_url: String,
    pub username: String,
    pub source: String,
    pub title: String,
    pub rating: String,
    #[serde(rename = "content_url")]
    pub content_url: String,
    #[serde(rename = "source_tld")]
    pub source_tld: String,
    #[serde(rename = "source_post_url")]
    pub source_post_url: String,
    #[serde(rename = "is_sticker")]
    pub is_sticker: i64,
    #[serde(rename = "import_datetime")]
    pub import_datetime: String,
    #[serde(rename = "trending_datetime")]
    pub trending_datetime: String,
    pub images: Images,
    pub user: Option<User>,
    #[serde(rename = "analytics_response_payload")]
    pub analytics_response_payload: String,
    pub analytics: Analytics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images {
    pub original: Original,
    pub downsized: Downsized,
    #[serde(rename = "downsized_large")]
    pub downsized_large: DownsizedLarge,
    #[serde(rename = "downsized_medium")]
    pub downsized_medium: DownsizedMedium,
    #[serde(rename = "downsized_small")]
    pub downsized_small: DownsizedSmall,
    #[serde(rename = "downsized_still")]
    pub downsized_still: DownsizedStill,
    #[serde(rename = "fixed_height")]
    pub fixed_height: FixedHeight,
    #[serde(rename = "fixed_height_downsampled")]
    pub fixed_height_downsampled: FixedHeightDownsampled,
    #[serde(rename = "fixed_height_small")]
    pub fixed_height_small: FixedHeightSmall,
    #[serde(rename = "fixed_height_small_still")]
    pub fixed_height_small_still: FixedHeightSmallStill,
    #[serde(rename = "fixed_height_still")]
    pub fixed_height_still: FixedHeightStill,
    #[serde(rename = "fixed_width")]
    pub fixed_width: FixedWidth,
    #[serde(rename = "fixed_width_downsampled")]
    pub fixed_width_downsampled: FixedWidthDownsampled,
    #[serde(rename = "fixed_width_small")]
    pub fixed_width_small: FixedWidthSmall,
    #[serde(rename = "fixed_width_small_still")]
    pub fixed_width_small_still: FixedWidthSmallStill,
    #[serde(rename = "fixed_width_still")]
    pub fixed_width_still: FixedWidthStill,
    pub looping: Looping,
    #[serde(rename = "original_still")]
    pub original_still: OriginalStill,
    #[serde(rename = "original_mp4")]
    pub original_mp4: OriginalMp4,
    pub preview: Preview,
    #[serde(rename = "preview_gif")]
    pub preview_gif: PreviewGif,
    #[serde(rename = "preview_webp")]
    pub preview_webp: PreviewWebp,
    pub hd: Option<Hd>,
    #[serde(rename = "480w_still")]
    pub n480w_still: n480wStill,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
    #[serde(rename = "webp_size")]
    pub webp_size: String,
    pub webp: String,
    pub frames: String,
    pub hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downsized {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownsizedLarge {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownsizedMedium {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownsizedSmall {
    pub height: String,
    pub width: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownsizedStill {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedHeight {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
    #[serde(rename = "webp_size")]
    pub webp_size: String,
    pub webp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedHeightDownsampled {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
    #[serde(rename = "webp_size")]
    pub webp_size: String,
    pub webp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedHeightSmall {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
    #[serde(rename = "webp_size")]
    pub webp_size: String,
    pub webp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedHeightSmallStill {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedHeightStill {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedWidth {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
    #[serde(rename = "webp_size")]
    pub webp_size: String,
    pub webp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedWidthDownsampled {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
    #[serde(rename = "webp_size")]
    pub webp_size: String,
    pub webp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedWidthSmall {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
    #[serde(rename = "webp_size")]
    pub webp_size: String,
    pub webp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedWidthSmallStill {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedWidthStill {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Looping {
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalStill {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalMp4 {
    pub height: String,
    pub width: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub height: String,
    pub width: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewGif {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewWebp {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hd {
    pub height: String,
    pub width: String,
    #[serde(rename = "mp4_size")]
    pub mp4_size: String,
    pub mp4: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n480wStill {
    pub height: String,
    pub width: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "banner_image")]
    pub banner_image: String,
    #[serde(rename = "banner_url")]
    pub banner_url: String,
    #[serde(rename = "profile_url")]
    pub profile_url: String,
    pub username: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    pub description: String,
    #[serde(rename = "instagram_url")]
    pub instagram_url: String,
    #[serde(rename = "website_url")]
    pub website_url: String,
    #[serde(rename = "is_verified")]
    pub is_verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Analytics {
    pub onload: Onload,
    pub onclick: Onclick,
    pub onsent: Onsent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Onload {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Onclick {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Onsent {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    pub count: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub status: i64,
    pub msg: String,
    #[serde(rename = "response_id")]
    pub response_id: String,
}
