<script>
  import { collection } from '../lib/store';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  
  let searchText = '';

  function handleSearch() {
    if (searchText && $collection) {
      const results = $collection.search(searchText);
      dispatch('search', Array.from(results).sort((a, b) => b.score - a.score));
    }
  }
</script>

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

<style>
  .search-section {
    margin-bottom: 2rem;
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