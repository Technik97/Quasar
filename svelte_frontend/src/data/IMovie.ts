export interface Movie {
    id: Number,
    title: String,
    runtime: String,
}

export interface MovieResponse {
    movies: Movie[],
}