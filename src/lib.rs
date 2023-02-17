/*
use rustygdb::APIWrapper;

let wrapper = APIWrapper::new(&access_token, &client_id).await.unwrap();

let zelda_games = wrapper
      .games()
      .fields("name")
      .search("zelda")
      .limit("5")
      .call()
      .await
      .unwrap();

*/
pub mod response_handler;
pub mod data;
pub mod helpers;

use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use response_handler::Result;
use helpers::games::GamesHelper;


pub(crate) const BASE_URL: &str = "https://api.igdb.com";
pub(crate) const VERSION:  &str = "v4";

pub struct APIWrapper {
    pub http_client: Client,
}

impl APIWrapper {
    pub async fn new(
        access_token: &str,
        client_id: &str
    ) -> Result<APIWrapper> {
        let mut headers = header::HeaderMap::new();
        
        headers.insert("Authorization", format!("Bearer {}", access_token).parse().unwrap());
        headers.insert("Client-ID", header::HeaderValue::from_str(client_id).unwrap());
    
        let http_client = Client::builder()
            .default_headers(headers)
            .build()?;

        let wrapper = APIWrapper { http_client };

        Ok(wrapper)
    }

    async fn post<D>(&self, endpoint: &str, body: String) -> Result<D>
    where
        D: DeserializeOwned,
    {
        let response = self
          .http_client
          .post(endpoint)
          .body(body)
          .send()
          .await?;

        Ok(response.json().await?)
    }

    pub fn games(&self) -> GamesHelper<'_> {
        GamesHelper { wrapper: self, query_string: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn games_endpoint() {
        let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
        let client_id = env::var("TWITCH_CLIENT_ID").unwrap();

        let api_wrapper = self::APIWrapper::new(&access_token, &client_id).await.unwrap();

        let limited_games = api_wrapper
            .games()
            .limit("5")
            .call()
            .await
            .unwrap();

        assert_eq!(limited_games.len(), 5);

        let zelda_games = api_wrapper
            .games()
            .fields("name")
            .search("zelda")
            .call()
            .await
            .unwrap();

        let first_zelda_game = zelda_games.first().unwrap().name().clone().unwrap();

        assert_eq!(first_zelda_game, "Zelda II: The Adventure of Link");
    }
}
