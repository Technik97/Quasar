<script lang="ts">
    export let movie;
    import  '../../public/index.css'
</script>

<div class="text-center border-2 border-sky-500 bg-sky-200 rounded-md">
    <div class="text-blue-700 p-4">
        <h1 class="text-3xl">{movie.title}</h1>
        <h1>Runtime: {movie.runtime}</h1>
    </div>
</div>