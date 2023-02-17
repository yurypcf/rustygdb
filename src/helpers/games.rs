use crate::response_handler::Result;
use crate::APIWrapper;
use crate::data::games::Game;

pub struct GamesHelper<'a> {
    pub wrapper: &'a APIWrapper,
    pub query_string: Vec<&'a str>,
}

impl<'a> GamesHelper<'a> {
    pub fn fields(self, input: &'a str) -> GamesHelper<'a> {
        self.str_iterator(vec!["fields ", input, ";"])
    }

    pub fn search(self, input: &'a str) -> GamesHelper<'a> {
        self.str_iterator(vec!["search ", "\"", input, "\"", ";"])
    }

    pub fn where_statement(self, input: &'a str) -> GamesHelper<'a> {
        self.str_iterator(vec!["where ", input, ";"])
    }
    
    pub fn exclude(self, input: &'a str) -> GamesHelper<'a> {
        self.str_iterator(vec!["exclude ", input, ";"])
    }

    pub fn limit(self, input: &'a str) -> GamesHelper<'a> {
        self.str_iterator(vec!["limit ", input, ";"])
    }

    pub fn offset(self, input: &'a str) -> GamesHelper<'a> {
        self.str_iterator(vec!["offset ", input, ";"])
    }

    pub fn sort(self, input: &'a str) -> GamesHelper<'a> {
        self.str_iterator(vec!["sort ", input, ";"])
    }

    pub fn str_iterator(mut self, str_vector: Vec<&'a str>) -> GamesHelper<'a> {
        for str_slice in str_vector {
            self.query_string.push(str_slice)
        }
        self
    }

    pub async fn call(&self) -> Result<Vec<Game>> {
        let body = self.query_string.join("");

        self.wrapper.post(&format!("{}/{}/games", crate::BASE_URL, crate::VERSION), body).await
    }
}