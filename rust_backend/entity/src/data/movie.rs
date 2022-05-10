use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub runtime: String,
    // pub production_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct IndexMovieResponse {
    pub movies: Vec<Movie>
}