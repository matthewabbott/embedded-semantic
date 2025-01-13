<script>
  import init, { TfIdfDocument, TfIdfCollection } from '../../rust-embedding/pkg/rust_embedding.js';
  import { onMount } from 'svelte';

  let collection;
  let searchText = '';
  let searchResults = [];
  let isLoaded = false;

  onMount(async () => {
    await init();
    collection = new TfIdfCollection();
    isLoaded = true;
  });

  function handleAddDocument() {
    if (searchText && isLoaded) {
      const doc = new TfIdfDocument(searchText);
      collection.add_document(doc);
      searchText = '';
    }
  }

  function handleSearch() {
    if (searchText && isLoaded) {
      const results = collection.search(searchText);
      searchResults = Array.from(results).sort((a, b) => b.score - a.score);
    }
  }
</script>

<main>
  <h1>Semantic Search Demo (TF-IDF)</h1>
  
  {#if isLoaded}
    <div>
      <h2>Add Document</h2>
      <textarea 
        bind:value={searchText} 
        placeholder="Enter text to add to the collection"
      ></textarea>
      <div class="buttons">
        <button on:click={handleAddDocument}>Add Document</button>
        <button on:click={handleSearch}>Search</button>
      </div>
      
      {#if searchResults.length > 0}
        <h2>Search Results</h2>
        <div class="results">
          {#each searchResults as result}
            <div class="result">
              <p>{result.text}</p>
              <p class="score">Similarity: {result.score.toFixed(3)}</p>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else}
    <p>Loading WASM module...</p>
  {/if}
</main>

<style>
  .buttons {
    display: flex;
    gap: 1rem;
    margin: 1rem 0;
  }
  .results {
    margin-top: 1rem;
  }
  .result {
    border: 1px solid #ccc;
    padding: 1rem;
    margin-bottom: 1rem;
  }
  .score {
    color: #666;
    font-size: 0.9em;
  }
</style>