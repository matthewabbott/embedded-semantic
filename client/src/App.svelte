<script>
  import init, { TfIdfDocument, TfIdfCollection } from '../../rust-embedding/pkg/rust_embedding.js';
  import { onMount } from 'svelte';

  let collection;
  let searchText = '';
  let searchResults = [];
  let isLoaded = false;
  let fileInput;

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

  async function handleFileUpload(event) {
    const file = event.target.files[0];
    if (!file) return;

    try {
      const text = await file.text();
      // Split into manageable chunks (e.g., by paragraphs)
      const chunks = text.split(/\n\s*\n/)  // Split on blank lines
                        .filter(chunk => chunk.trim().length > 0)
                        .map(chunk => chunk.trim());
      
      for (const chunk of chunks) {
        const doc = new TfIdfDocument(chunk);
        collection.add_document(doc);
      }

      // Clear file input
      fileInput.value = '';
    } catch (error) {
      console.error('Error processing file:', error);
      alert('Error processing file. Please try again.');
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
      <h2>Add Content</h2>
      
      <div class="input-section">
        <h3>Enter Text</h3>
        <textarea 
          bind:value={searchText} 
          placeholder="Enter text to add to the collection"
        ></textarea>
        <button on:click={handleAddDocument}>Add Text</button>
      </div>

      <div class="input-section">
        <h3>Upload File</h3>
        <input 
          bind:this={fileInput}
          type="file" 
          accept=".txt,.md,.csv"
          on:change={handleFileUpload}
        />
      </div>

      <div class="search-section">
        <h2>Search</h2>
        <div class="search-box">
          <input 
            type="text" 
            bind:value={searchText}
            placeholder="Enter search query"
          />
          <button on:click={handleSearch}>Search</button>
        </div>
      </div>
      
      {#if searchResults.length > 0}
        <div class="results">
          <h2>Search Results ({searchResults.length})</h2>
          {#each searchResults as result}
            <div class="result">
              <p class="result-text">{result.text}</p>
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
  main {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }

  .input-section {
    margin-bottom: 2rem;
    padding: 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
  }

  textarea {
    width: 100%;
    height: 100px;
    margin-bottom: 1rem;
    padding: 0.5rem;
  }

  .search-box {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .search-box input {
    flex: 1;
    padding: 0.5rem;
  }

  .results {
    margin-top: 2rem;
  }

  .result {
    border: 1px solid #ddd;
    padding: 1rem;
    margin-bottom: 1rem;
    border-radius: 4px;
  }

  .result-text {
    white-space: pre-wrap;
    margin-bottom: 0.5rem;
  }

  .score {
    color: #666;
    font-size: 0.9em;
    margin: 0;
  }

  button {
    padding: 0.5rem 1rem;
    background-color: #4CAF50;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  button:hover {
    background-color: #45a049;
  }
</style>