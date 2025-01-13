<script>
  export let results = [];
</script>

{#if results.length > 0}
  <div class="results">
    <h2>Search Results ({results.length})</h2>
    {#each results as result}
      <div class="result">
        <p class="result-text">{result.text}</p>
        <p class="score">Similarity: {result.score.toFixed(3)}</p>
      </div>
    {/each}
  </div>
{/if}

<style>
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
</style>