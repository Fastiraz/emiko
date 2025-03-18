#![allow(unused)]

/**
 * loader
 * chunks
 * embeddings
 * vector database
 * query
*/


use std::{
  path::{
    Path,
    PathBuf
  },
  collections::HashMap,
  fs,
  thread
};


#[derive(Debug, Clone)]
pub struct Document {
  page_content: String,
  metadata: HashMap<String, String>,
}

pub struct RAG {
  dataset_path: String,
  documents: Vec<Document>,
  embedding_model: String,
  embedding_url: String,
  embed: String,
  chunks: Vec<String>
}


impl RAG {
  pub fn new() -> Self {
    let mut instance = RAG {
      dataset_path: String::new(),
      documents: Vec::new(),
      embedding_model: "mxbai-embed-large:latest".to_string(),
      embedding_url: "http://localhost:11434/api/embed".to_string(),
      embed: String::new(),
      chunks: Vec::new()
    };

    instance.create_datasets_directory();
    return instance
  }


  /**
   * this function create a datasets
   * folder if not already exists
   */
  fn create_datasets_directory(&mut self) {
    let home_directory: String = std::env::var("HOME")
      .or_else(|_| std::env::var("USERPROFILE"))
      .unwrap_or_else(|_| panic!("Unable to get your home directory!"));

    let mut datasets_path = PathBuf::from(home_directory);
    datasets_path.push(".config");
    datasets_path.push("emiko");
    datasets_path.push("datasets");

    if !datasets_path.exists() {
      println!("Creating datasets directory...");
      std::fs::create_dir_all(&datasets_path)
        .expect("Failed to create datasets directory");
    }

    self.dataset_path = datasets_path.to_str().unwrap().to_string();
  }


  /**
   * this function load data from
   * datasets into a Document Vector
   */
  pub fn loader(&mut self, recursive: bool) -> Vec<Document> {
    let entries: Vec<PathBuf> = fs::read_dir(&self.dataset_path)
      .unwrap()
      .filter_map(|entry| entry.ok().map(|e| e.path()))
      .collect();

    let mut handles = Vec::new();

    for path in entries {
      if path.is_file() {
        let path_clone = path.clone();
        let handle = thread::spawn(move || {
          let content = fs::read_to_string(&path_clone).unwrap();
          let mut metadata = HashMap::new();
          metadata.insert("filename".to_string(), path_clone.to_str().unwrap().to_string());
          vec![Document {
            page_content: content,
            metadata,
          }]
        });
        handles.push(handle);
      } else if path.is_dir() && recursive {
        let dataset_path = path.to_str().unwrap().to_string();
        let embedding_url = self.embedding_url.clone();
        let handle = thread::spawn(move || {
          let mut sub_rag = RAG::new();
          sub_rag.dataset_path = dataset_path;
          let sub_docs = sub_rag.loader(true);
          sub_docs
        });
        handles.push(handle);
      }
    }

    for handle in handles {
      let mut docs = handle.join().unwrap();
      self.documents.append(&mut docs);
    }

    return self.documents.clone()
  }


  /**
   * semantic chunking
   * max size = 65535
   */
  pub fn chunk(&mut self, chunk_size: u16) -> Vec<String> {
    let separators: [&str; 7] = ["\n\n", "\n", " ", "", "! ", "? ", ". "];

    self.chunks.clear();

    for document in &self.documents {
      let mut doc_chunks = vec![document.page_content.clone()];

      for separator in &separators {
        doc_chunks = doc_chunks
          .into_iter()
          .flat_map(|chunk| chunk.split(separator).map(String::from).collect::<Vec<String>>())
          .filter(|s| !s.is_empty())
          .collect();
      }

      self.chunks.extend(doc_chunks);
    }

    self.chunks.clone()
  }
}
