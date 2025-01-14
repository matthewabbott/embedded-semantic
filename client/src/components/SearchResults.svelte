<script>
  export let results = [];
</script>

{#if results.length > 0}
  <div class="results">
    <h2>Search Results ({results.length})</h2>
    {#each results as result}
      <div class="result">
        <div class="document-chunk">
          <p class="result-text">{result.text}</p>
        </div>
        <div class="metadata">
          <p class="score">Similarity: {result.score.toFixed(3)}</p>
        </div>
      </div>
    {/each}
  </div>
{/if}

<style>
  .results {
    margin-top: 2rem;
  }

  .result {
    border: 1px solid #e0e0e0;
    border-left: 4px solid #4CAF50;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    border-radius: 4px;
    background-color: white;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  }

  .document-chunk {
    padding-bottom: 0.75rem;
    border-bottom: 1px solid #f0f0f0;
  }

  .result-text {
    white-space: pre-wrap;
    margin: 0;
    line-height: 1.5;
    color: #333;
    font-size: 1rem;
  }

  .metadata {
    margin-top: 0.75rem;
  }

  .score {
    color: #666;
    font-size: 0.9em;
    margin: 0;
    font-weight: 500;
  }
</style>