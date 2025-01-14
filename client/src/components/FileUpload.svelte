<script>
  import { collection, uploadStatus } from '../lib/store';
  import { TfIdfDocument } from '../../../rust-embedding/pkg/rust_embedding.js';
  import { create_semantic_chunks } from '../../../rust-embedding/pkg/rust_embedding.js';
  import FilePreview from './FilePreview.svelte';

  let fileInput;
  let dragOver = false;
  let previewData = null;

  async function processFile(file) {
    if (!file) return;

    try {
      const text = await file.text();
      // Use semantic chunking with window size of 3 sentences and threshold of 0.3
      const chunks = create_semantic_chunks(text, 3, 0.3);
      previewData = {
        content: text,
        chunks: Array.from(chunks), // Convert from JS Array to regular array
        filename: file.name
      };
	  console.log(previewData);
      console.log('Chunks created:', previewData.chunks); // Debug log
    } catch (error) {
      console.error('Error processing file:', error);
      $uploadStatus = {
        isUploading: false,
        message: 'Error reading file. Please try again.',
        documentsAdded: 0
      };
    }
  }

  async function handleConfirmUpload() {
    try {
      const { content, filename } = previewData;
      
      $uploadStatus = {
        isUploading: true,
        message: `Processing ${filename}...`,
        documentsAdded: 0
      };

      const chunks = content.split(/\n\s*\n/)
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
        message: `Successfully added ${chunks.length} chunks from ${filename}`,
        documentsAdded: chunks.length
      };

      // Clear preview
      previewData = null;

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

  function handleCancelUpload() {
    previewData = null;
    fileInput.value = '';
  }

  async function handleFileUpload(event) {
    const file = event.target.files[0];
    await processFile(file);
    fileInput.value = ''; // Clear the file input
  }

  function handleDragOver(event) {
    event.preventDefault();
    dragOver = true;
  }

  function handleDragLeave() {
    dragOver = false;
  }

  async function handleDrop(event) {
    event.preventDefault();
    dragOver = false;
    
    const files = event.dataTransfer.files;
    if (files.length > 0) {
      await processFile(files[0]);
    }
  }
</script>

<div 
  class="input-section"
  class:drag-over={dragOver}
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
  on:drop={handleDrop}
  role="region"
  aria-label="File upload dropzone"
>
  <h3>Upload File</h3>
  <div class="upload-zone">
    <input 
      bind:this={fileInput}
      type="file" 
      accept=".txt,.md,.csv"
      on:change={handleFileUpload}
      class="file-input"
    />
    <div class="upload-message">
      Drag and drop a file here or click to select
    </div>
  </div>
  
  {#if $uploadStatus.message}
    <div class="status-message" class:error={$uploadStatus.message.includes('Error')}>
      {$uploadStatus.message}
    </div>
  {/if}
</div>

{#if previewData}
  <FilePreview 
    content={previewData.content}
    chunks={previewData.chunks}
    filename={previewData.filename}
    onConfirm={handleConfirmUpload}
    onCancel={handleCancelUpload}
  />
{/if}

<style>
  .input-section {
    margin-bottom: 2rem;
    padding: 1rem;
    border: 2px dashed #ddd;
    border-radius: 4px;
    transition: all 0.3s ease;
  }

  .drag-over {
    border-color: #4CAF50;
    background-color: rgba(76, 175, 80, 0.1);
  }

  .upload-zone {
    position: relative;
    height: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #f5f5f5;
    border-radius: 4px;
    cursor: pointer;
  }

  .file-input {
    position: absolute;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
  }

  .upload-message {
    color: #666;
    text-align: center;
    pointer-events: none;
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