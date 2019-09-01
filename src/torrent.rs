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
    pub fn bytes(&self) -> String {
        match &self.Size {
            Some(size) => size.clone(),
            None => "0".to_string(),
        }
    }

    fn prefix_bytes(&self, power: u32) -> String {
        let base: u64 = 1024;
        format!(
            "{:.2}",
            self.bytes().parse::<f64>().unwrap_or(0.0) / base.pow(power) as f64
        )
    }

    pub fn kb(&self) -> String {
        self.prefix_bytes(1)
    }

    pub fn mb(&self) -> String {
        self.prefix_bytes(2)
    }

    pub fn gb(&self) -> String {
        self.prefix_bytes(3)
    }
}
