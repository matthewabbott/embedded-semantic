# Embedded Semantic Search

A web application to (eventually, when it's working) perform local semantic search on documents using WebAssembly (WASM) and Rust for efficient client-side processing.

## Structure

- `client/`: Svelte frontend application
- `rust-embedding/`: Rust code compiled to WASM for document processing and embedding generation

## Development

### Prerequisites
- Rust and Cargo
- Node.js and npm
- wasm-pack

### Setup
1. Build the Rust WASM module:
```bash
cd rust-embedding
wasm-pack build --target web
```

2. Start the development server:
```bash
cd client
npm install
npm run dev
```

## Current Features
- TF-IDF search implementation
- File upload for plaintext files
- Simple similarity comparison
- Client-side processing using WASM

## TODOs
- Core: Neural net based search implementation
- Input: Support for PDF and other document formats
- UI: Fancy search result visualization
- UI: Better display for similarity scores
- Deployment: host on the web

## Tech Stack
- Frontend: Svelte
- Embedding Processing: Rust compiled to WebAssembly
- Build Tools: wasm-pack, Vite

## File Structure

```
embedded-semantic/               # Root project directory
├── .gitignore                  # Root gitignore file
├── readme.md                   # Root readme
├── client/                     # Svelte frontend
│   ├── node_modules/          # (ignored in git) Node.js dependencies
│   ├── public/                
│   │   └── vite.svg          
│   ├── src/                   
│   │   ├── App.svelte        # Main Svelte component
│	│	├── components/
│	│	│   ├── TextInput.svelte
│	│	│   ├── FileUpload.svelte
│	│	│   ├── SearchBox.svelte
│	│	│   └── SearchResults.svelte
│   │   ├── lib/              
│	│	│	└── store.js    # For sharing state between components
│   │   ├── assets/           
│   │   │   └── svelte.svg    
│   │   ├── app.css           
│   │   ├── main.js           # Entry point
│   │   └── vite-env.d.ts
│   ├── index.html            
│   ├── jsconfig.json         
│   ├── package-lock.json     
│   ├── package.json          # Node.js dependencies and scripts
│   ├── README.md             
│   ├── svelte.config.js      
│   └── vite.config.js        # Vite configuration (WASM settings)
│
└── rust-embedding/            # Rust/WASM code
	src/
	├── lib.rs
	├── embeddings/
	│   ├── mod.rs           # Module declarations
	│   ├── traits.rs        # Shared traits
	│   └── tf_idf.rs        # TF-IDF implementation
	├── utils/
	│	└── mod.rs           # Utility functions
    ├── target/               # (ignored in git) Rust build output
    ├── pkg/                  # (ignored in git) WASM build output
    ├── Cargo.lock           # Rust dependency versions
    └── Cargo.toml           # Rust dependencies and configuration
```