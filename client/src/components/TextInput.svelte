<script>
  import { collection, neuralCollection, searchMode } from '../lib/store';
  import { TfIdfDocument, NeuralDocument } from '../../../rust-embedding/pkg/rust_embedding.js';

  let inputText = '';

  function handleAddDocument() {
    if (inputText) {
      if ($searchMode === 'tf-idf') {
        const doc = new TfIdfDocument(inputText);
        $collection.add_document(doc);
      } else {
        const doc = new NeuralDocument(inputText);
        $neuralCollection.add_document(doc);
      }
      inputText = '';
    }
  }
</script>

<div class="input-section">
  <h3>Enter Text</h3>
  <textarea 
    bind:value={inputText} 
    placeholder="Enter text to add to the collection"
  ></textarea>
  <button on:click={handleAddDocument}>Add Text</button>
</div>

<style>
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