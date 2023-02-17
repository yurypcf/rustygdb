# IGDB Rust API wrapper

A Rust wrapper for [IGDB API]().

- Built on [reqwest]()
- Using [tokio]() asynchronous runtime

## Usage

```toml
[dependencies]
rustygdb = "0.1.0"
```

```Rust
use rustygdb::APIWrapper;
...

let wrapper = APIWrapper::new(&access_token, &client_id).await.unwrap();

// A vec of struct Game
let zelda_games = wrapper
      .games()
      .fields("name")
      .search("zelda")
      .limit("5")
      .call()
      .await
      .unwrap();
```
