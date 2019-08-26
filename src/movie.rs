use serde_derive::Deserialize;

use crate::torrent::Torrent;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Director {
    pub Id: Option<String>,
    pub Name: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Movie {
    pub Cover: Option<String>,
    pub Directors: Option<Vec<Director>>,
    pub GroupId: Option<String>,
    pub ImbdId: Option<String>,
    pub LastUploadTime: Option<String>,
    pub MaxSize: Option<usize>,
    pub Tags: Option<Vec<String>>,
    pub Title: Option<String>,
    pub Torrents: Option<Vec<Torrent>>,
    pub TotalLeechers: Option<usize>,
    pub TotalSeeders: Option<usize>,
    pub TotalSnatched: Option<usize>,
    pub Year: Option<String>,
}
impl Movie {
    pub fn title(&self) -> &Option<String> {
        &self.Title
    }

    pub fn torrents(&self) -> Vec<Torrent> {
        match &self.Torrents {
            Some(torrents) => torrents.clone(),
            None => Vec::new(),
        }
    }
}