<script lang="ts">
    export let movie;
    import  '../../public/index.css'
</script>

<div class="text-gray-500 text-center text-lg">
    <h1>{movie.id}</h1>
    <h1>{movie.title}</h1>
    <h1>{movie.runtime}</h1>
</div>