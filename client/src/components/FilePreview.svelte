<script>
  export let content = '';
  export let filename = '';
  export let onConfirm;
  export let onCancel;

  let previewContent = content.slice(0, 1000); // First 1000 chars
  let totalChunks = content.split(/\n\s*\n/)
                          .filter(chunk => chunk.trim().length > 0)
                          .length;
</script>

<div class="preview-modal">
  <div class="preview-content">
    <h3>Preview: {filename}</h3>
    
    <div class="preview-info">
      <p>File will be split into {totalChunks} chunks for processing.</p>
    </div>

    <div class="preview-text">
      {previewContent}
      {#if content.length > 1000}
        <span class="truncated">... (content truncated)</span>
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
  }

  .preview-info {
    margin: 1rem 0;
    padding: 0.5rem;
    background-color: #f5f5f5;
    border-radius: 4px;
  }

  .preview-text {
    white-space: pre-wrap;
    font-family: monospace;
    background-color: #f8f9fa;
    padding: 1rem;
    border-radius: 4px;
    margin: 1rem 0;
    max-height: 400px;
    overflow-y: auto;
  }

  .truncated {
    color: #666;
    font-style: italic;
  }

  .preview-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1rem;
  }

  button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
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
</style>