
use actix_web::{get, Responder, HttpResponse};

use entity::data::movie::{Movie, IndexMovieResponse};



#[get("/")]
async fn index() -> impl Responder {
    
    let movies = entity::movie::get_movies().await;

    let response: Vec<Movie> = movies.into_iter().map(|movie| Movie {
        id: movie.id,
        runtime: movie.runtime,
        title: movie.title,
    }).collect();

    let index_movie_response = IndexMovieResponse {
        movies: response,
    };

    HttpResponse::Ok().json(index_movie_response)
}
