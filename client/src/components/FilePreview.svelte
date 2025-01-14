<script>
  export let content = '';
  export let filename = '';
  export let onConfirm;
  export let onCancel;

  let chunks = content.split(/\n\s*\n/)
                     .filter(chunk => chunk.trim().length > 0)
                     .map(chunk => chunk.trim());
  
  let displayCount = 5;  // Initial number of chunks to display

  $: displayedChunks = chunks.slice(0, displayCount);
  $: remainingChunks = chunks.length - displayCount;

  function showMoreChunks() {
    displayCount = Math.min(displayCount + 5, chunks.length);
  }
</script>

<div class="preview-modal">
  <div class="preview-content">
    <h3>Preview: {filename}</h3>
    
    <div class="preview-info">
      <p>File will be split into {chunks.length} chunks for processing.</p>
    </div>

    <div class="preview-chunks">
      {#each displayedChunks as chunk, i}
        <div class="chunk">
          <div class="chunk-header">Chunk {i + 1}</div>
          <div class="chunk-content">{chunk}</div>
        </div>
      {/each}
      {#if remainingChunks > 0}
        <button class="show-more" on:click={showMoreChunks}>
          Show 5 more chunks ({remainingChunks} remaining)
        </button>
      {/if}
    </div>

    <div class="preview-actions">
      <button class="confirm" on:click={onConfirm}>Process File</button>
      <button class="cancel" on:click={onCancel}>Cancel</button>
    </div>
  </div>
</div>

<style>
  .preview-modal {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .preview-content {
    background-color: white;
    padding: 2rem;
    border-radius: 8px;
    width: 90%;
    max-width: 800px;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .preview-info {
    margin: 1rem 0;
    padding: 0.75rem;
    background-color: #e8f5e9;
    border-radius: 4px;
    color: #2e7d32;
  }

  .preview-chunks {
    margin: 1.5rem 0;
    max-height: 400px;
    overflow-y: auto;
    padding-right: 1rem;
  }

  .chunk {
    background-color: #f8f9fa;
    border: 1px solid #e0e0e0;
    border-left: 4px solid #4CAF50;
    border-radius: 4px;
    margin-bottom: 1rem;
  }

  .chunk-header {
    padding: 0.5rem 1rem;
    background-color: #fafafa;
    border-bottom: 1px solid #e0e0e0;
    color: #666;
    font-size: 0.9em;
    font-weight: 500;
  }

  .chunk-content {
    padding: 1rem;
    white-space: pre-wrap;
    font-family: system-ui, -apple-system, sans-serif;
    line-height: 1.5;
  }

  .more-chunks {
    text-align: center;
    padding: 1rem;
    color: #666;
    font-style: italic;
    background-color: #f8f9fa;
    border-radius: 4px;
  }

  .preview-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 1px solid #e0e0e0;
  }

  button {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: opacity 0.2s;
  }

  .confirm {
    background-color: #4CAF50;
    color: white;
  }

  .cancel {
    background-color: #f44336;
    color: white;
  }

  button:hover {
    opacity: 0.9;
  }
  
  .show-more {
    width: 100%;
    padding: 1rem;
    margin-top: 1rem;
    background-color: #f0f0f0;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    color: #666;
    cursor: pointer;
    font-size: 0.9em;
    transition: all 0.2s ease;
  }

  .show-more:hover {
    background-color: #e8e8e8;
    color: #333;
  }
</style>