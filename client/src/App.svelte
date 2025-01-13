<script>
  import init, { Document } from '../../rust-embedding/pkg/rust_embedding.js';
  import { onMount } from 'svelte';

  let documents = [];
  let searchText = '';
  let isLoaded = false;

  onMount(async () => {
    await init();
    isLoaded = true;
  });

  function handleAddDocument() {
    if (searchText && isLoaded) {
      const doc = new Document(searchText);
      documents = [...documents, { text: searchText, doc }];
      searchText = '';
    }
  }

  function findSimilar(text) {
    const searchDoc = new Document(text);
    return documents
      .map(d => ({
        text: d.text,
        similarity: d.doc.similarity(searchDoc)
      }))
      .sort((a, b) => b.similarity - a.similarity);
  }
</script>

<main>
  <h1>Semantic Search Demo</h1>
  
  {#if isLoaded}
    <div>
      <h2>Add Document</h2>
      <textarea 
        bind:value={searchText} 
        placeholder="Enter text to add to the collection"
      ></textarea>
      <button on:click={handleAddDocument}>Add Document</button>
      
      {#if documents.length > 0}
        <h2>Documents ({documents.length})</h2>
        <div class="documents">
          {#each documents as {text}, i}
            <div class="document">
              <p>{text}</p>
              <button on:click={() => {
                const similar = findSimilar(text);
                console.log('Similar documents:', similar);
              }}>Find Similar</button>
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
  .documents {
    margin-top: 1rem;
  }
  .document {
    border: 1px solid #ccc;
    padding: 1rem;
    margin-bottom: 1rem;
  }
</style>