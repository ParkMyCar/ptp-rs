use serde_derive::Deserialize;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Torrent {
    pub Checked: Option<bool>,
    pub Codec: Option<String>,
    pub Container: Option<String>,
    pub GoldenPopcorn: Option<bool>,
    pub Id: Option<usize>,
    pub Leechers: Option<String>,
    pub Quality: Option<String>,
    pub ReleaseGroup: Option<String>,
    pub ReleaseName: Option<String>,
    pub Resolution: Option<String>,
    pub Scene: Option<bool>,
    pub Seeders: Option<String>,
    pub Size: Option<String>,
    pub Snatched: Option<String>,
    pub Source: Option<String>,
    pub UploadTime: Option<String>,
}
impl Torrent {
    pub fn gb(&self) -> String {
        match &self.Size {
            Some(size) => {
                let size_float = size.parse::<f64>().unwrap_or(0.0);
                format!("{:.2}", (size_float / 1024.0 / 1024.0 / 1024.0))
            },
            None => "0".to_string(),
        }
    }
}