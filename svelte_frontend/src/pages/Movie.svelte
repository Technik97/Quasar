<script lang="ts">
    import { onMount } from 'svelte';
    import MovieCard from '../components/MovieCard.svelte';

    interface Movie {
        id: Number,
        title: String,
        runtime: String,
    }
    
    let movies: Movie[];

    onMount(async () => {
        await fetch(`http://localhost:8000/`)
            .then(r => r.json())
            .then(data => {
                movies = data
            });
    })
</script>

<div>
    {#if movies}
    {#each movies as movie }   
        <MovieCard {movie} />
    {/each}
    {:else}
        <p class="loading">loading...</p>
    {/if}
</div>
