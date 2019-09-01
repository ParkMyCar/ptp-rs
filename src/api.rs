use reqwest;
use regex::Regex;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io::prelude::Write;
use std::fs::File;
use url::Url;

use crate::config::PtpKeys;
use crate::movie::Movie;
use crate::torrent::Torrent;

/*
================================================ API ===============================================
*/

#[derive(Debug)]
pub struct API {
    client: reqwest::Client,
    logged_in: bool,

    username: String,
    password: String,
    pass_key: String,
    auth_key: Option<String>,
}
impl API {    
    pub fn new(keys: PtpKeys) -> API {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        
        API {
            client: client,
            logged_in: false,
            username: keys.username,
            password: keys.password,
            pass_key: keys.pass_key,
            auth_key: None,
        }
    }

    /* Login */
    pub fn login(&mut self) {
        let mut data = HashMap::new();
        data.insert("username", self.username.clone());
        data.insert("password", self.password.clone());
        data.insert("passkey", self.pass_key.clone());

        // Make a login request to set our cookie
        println!("Logging in...");
        self.client.post("https://passthepopcorn.me/ajax.php?action=login")
            .form(&data)
            .send()
            .unwrap();
        self.logged_in = true;

        // Make a request to get an auth_key
        let mut auth_req = self.client.get("https://passthepopcorn.me/index.php")
            .send().unwrap();
        let body = auth_req.text().unwrap();

        self.auth_key = Some(get_auth_key_from_body(&body));
        println!("Login Successful!");
    }

    /* Logout */
    pub fn logout(&self) {
        let mut params = HashMap::new();

        // Given an auth_key make a request to logout
        println!("Logging out...");
        match &self.auth_key {
            Some(key) => {
                params.insert("auth", key);
                let url = Url::parse_with_params("https://passthepopcorn.me/logout.php", params)
                    .unwrap()
                    .into_string();
                self.client.get(&url).send().unwrap();
                println!("Logout successful!");
            },
            None => println!("Theres no auth_key! Cannot logout because we never logged in!"),
        }
    }

    /* Search */
    pub fn search(&mut self, filter: &SearchFilter) -> SearchResult {
        // Create our URL with params
        let mut params = HashMap::new();
        params.insert("json", "noredirect".to_string());
        if let Some(name) = &filter.name {
            params.insert("searchstr", name.clone());
        }
        if let Some(year) = &filter.year {
            params.insert("year", year.clone());
        }
        let url = Url::parse_with_params("https://passthepopcorn.me/torrents.php", params)
            .unwrap()
            .into_string();


        // Make the request, parse the JSON
        let json: SearchResult = self.client.get(&url)
            .send()
            .unwrap()
            .json()
            .unwrap();
        
        // If we get a new auth_key from the response, update ours
        if let Some(auth_key) = json.auth_key() {
            self.auth_key = Some(auth_key.clone());
        }

        // Return the Json
        json
    }

    /* Download Torrent */
    pub fn download_torrent(&self, torrent: &Torrent) -> File {
        let torrent_id: &str = &torrent.Id.unwrap().to_string();
        let mut params = HashMap::new();
        params.insert("action", "download");
        params.insert("id", torrent_id);
        let url = Url::parse_with_params("https://passthepopcorn.me/torrents.php", params)
            .unwrap()
            .into_string();

        let mut res = self.client.get(&url)
            .send()
            .unwrap();

        let filename = match get_torrent_name_from_header(res.headers()) {
            Some(name) => name,
            None => {
                let release_name = torrent.ReleaseName.
                                        as_ref()
                                        .expect("Could not get torrent name!");
                format!("{}.torrent", release_name)
            }
        };

        let file_content = res.text().unwrap();
        let mut torrent_file = File::create(filename).unwrap();
        torrent_file.write_all(file_content.as_bytes()).unwrap();
        
        torrent_file
    }
}

/*
============================================== Search ==============================================
*/

#[derive(Debug)]
pub struct SearchFilter {
    pub name: Option<String>,
    pub year: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub AuthKey: Option<String>,
    pub Movies: Option<Vec<Movie>>,
    pub Page: Option<String>,
    pub PassKey: Option<String>,
    pub TotalResults: Option<String>,
}
impl SearchResult {
    // Snakecase getter for AuthKey
    pub fn auth_key(&self) -> &Option<String> {
        &self.AuthKey
    }
    
    // Returns a cloned list of the movies, or an empty list if there were none returned
    pub fn movies(&self) -> Vec<Movie> {
        match &self.Movies {
            Some(movies) => movies.clone(),
            None => Vec::new(),
        }
    }

    // Returns the length of the movie list that was returned
    pub fn num_movies(&self) -> usize {
        match &self.Movies {
            Some(movies) => movies.len(),
            None => 0,
        }
    }

    // Snakecase getter for AuthKey
    pub fn page(&self) -> &Option<String> {
        &self.Page
    }

    // Snakecase getter for PassKey
    pub fn pass_key(&self) -> &Option<String> {
        &self.PassKey
    }

    // Snakecase getter for TotalResults
    pub fn total_results(&self) -> &Option<String> {
        &self.TotalResults
    }
}

/*
============================================ API Helpers ===========================================
*/

fn get_auth_key_from_body(body: &String) -> String {
        let auth_key_re = Regex::new(r"auth=([0-9a-f]{32})").unwrap();
        let auth_key_cap = auth_key_re.captures(body)
            .expect("Not able to find an auth key!");
        
        auth_key_cap[1].to_string()
}

fn get_torrent_name_from_header(header: &reqwest::header::HeaderMap) -> Option<String> {
    let filename_re = Regex::new(r#"filename="([\w\s.]*torrent)"#).unwrap();    
    let c_d = header.get(reqwest::header::CONTENT_DISPOSITION)
        .unwrap()
        .to_str()
        .unwrap();
    
    match filename_re.captures(c_d) {
        Some(captures) => {
            captures.get(1).map_or(None, |m| Some(String::from(m.as_str())))
        },
        None => None,
    }
}
