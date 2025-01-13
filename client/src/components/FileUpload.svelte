<script>
  import { collection, uploadStatus } from '../lib/store';
  import { TfIdfDocument } from '../../../rust-embedding/pkg/rust_embedding.js';

  let fileInput;

  async function handleFileUpload(event) {
    const file = event.target.files[0];
    if (!file) return;

    try {
      $uploadStatus = {
        isUploading: true,
        message: `Reading ${file.name}...`,
        documentsAdded: 0
      };

      const text = await file.text();
      const chunks = text.split(/\n\s*\n/)
                        .filter(chunk => chunk.trim().length > 0)
                        .map(chunk => chunk.trim());
      
      for (const [index, chunk] of chunks.entries()) {
        const doc = new TfIdfDocument(chunk);
        $collection.add_document(doc);
        
        $uploadStatus.documentsAdded = index + 1;
        $uploadStatus.message = `Processing chunks: ${index + 1}/${chunks.length}`;
      }

      $uploadStatus = {
        isUploading: false,
        message: `Successfully added ${chunks.length} chunks from ${file.name}`,
        documentsAdded: chunks.length
      };

      // Clear the file input
      fileInput.value = '';
      
      // Clear status after a delay
      setTimeout(() => {
        if (!$uploadStatus.isUploading) {
          $uploadStatus.message = '';
        }
      }, 3000);

    } catch (error) {
      console.error('Error processing file:', error);
      $uploadStatus = {
        isUploading: false,
        message: 'Error processing file. Please try again.',
        documentsAdded: 0
      };
    }
  }
</script>

<div class="input-section">
  <h3>Upload File</h3>
  <input 
    bind:this={fileInput}
    type="file" 
    accept=".txt,.md,.csv"
    on:change={handleFileUpload}
  />
  {#if $uploadStatus.message}
    <div class="status-message" class:error={$uploadStatus.message.includes('Error')}>
      {$uploadStatus.message}
    </div>
  {/if}
</div>

<style>
  .input-section {
    margin-bottom: 2rem;
    padding: 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
  }

  .status-message {
    margin-top: 1rem;
    padding: 0.5rem;
    background-color: #e8f5e9;
    border-radius: 4px;
  }

  .error {
    background-color: #ffebee;
  }
</style>