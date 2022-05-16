<script lang="ts">
    import { onMount } from 'svelte';
    import { movies_url } from '../api_url';
    import MovieCard from '../components/MovieCard.svelte';
    import type {MovieResponse} from '../data/IMovie'
    
    let fetched_data: MovieResponse;

    onMount(async () => {
        await fetch(movies_url)
            .then(r => r.json())
            .then(data => {
                fetched_data = data;
            });
    })
</script>

<div class="grid grid-cols-3 gap-3">
    {#if fetched_data}
    {#each fetched_data.movies as movie }   
        <div>
            <MovieCard {movie} />
        </div>
    {/each}
    {:else}
        <p class="loading">loading...</p>
    {/if}
</div>
