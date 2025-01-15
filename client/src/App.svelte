<script>
  import init, { TfIdfCollection, NeuralCollection } from '../../rust-embedding/pkg/rust_embedding.js';
  import { onMount } from 'svelte';
  import { collection, neuralCollection, searchMode } from './lib/store';
  import FileUpload from './components/FileUpload.svelte';
  import TextInput from './components/TextInput.svelte';
  import SearchBox from './components/SearchBox.svelte';
  import SearchResults from './components/SearchResults.svelte';

  let isLoaded = false;
  let searchResults = [];

  onMount(async () => {
    await init();
    $collection = new TfIdfCollection();
    $neuralCollection = new NeuralCollection();
    isLoaded = true;
  });
  
  function handleSearch(event) {
    searchResults = event.detail;
    console.log('Search results:', searchResults);  // Debug log
  }
</script>

<main>
  <h1>Semantic Search Demo</h1>
  
  {#if isLoaded}
    <div class="mode-selector">
      <label>
        <input 
          type="radio" 
          bind:group={$searchMode} 
          value="tf-idf"
        > TF-IDF
      </label>
      <label>
        <input 
          type="radio" 
          bind:group={$searchMode} 
          value="neural"
        > Neural
      </label>
    </div>

    <TextInput />
    <FileUpload />
    <SearchBox on:search={handleSearch} />
    <SearchResults results={searchResults} />
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