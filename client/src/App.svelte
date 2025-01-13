<script>
  import init, { TfIdfCollection } from '../../rust-embedding/pkg/rust_embedding.js';
  import { onMount } from 'svelte';
  import { collection } from './lib/store';
  import FileUpload from './components/FileUpload.svelte';
  import TextInput from './components/TextInput.svelte';
  import SearchBox from './components/SearchBox.svelte';
  import SearchResults from './components/SearchResults.svelte';

  let searchResults = [];
  let isLoaded = false;

  onMount(async () => {
    await init();
    $collection = new TfIdfCollection();
    isLoaded = true;
  });

  function handleSearchResults(event) {
    searchResults = event.detail;
  }
</script>

<main>
  <h1>Semantic Search Demo (TF-IDF)</h1>
  
  {#if isLoaded}
    <div>
      <h2>Add Content</h2>
      <TextInput />
      <FileUpload />
      <SearchBox on:search={handleSearchResults} />
      <SearchResults results={searchResults} />
    </div>
  {:else}
    <p>Loading WASM module...</p>
  {/if}
</main>

<style>
  main {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }
</style>