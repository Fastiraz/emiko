/**
 * loader
 * chunks
 * embeddings
 * vector database
 * query
*/


use std::{
  path::Path,
  collections::HashMap
};


#[derive(Debug, Clone)]
struct Document {
  page_content: String,
  metadata: HashMap<String, String>,
}

struct RAG {
  dataset_path: String,
  documents: String,
  embedding_model: String,
  embedding_url: String,
  embed: String
}


impl RAG {
  fn new(dataset_path: &str) -> Self {
    RAG {
      dataset_path: dataset_path.to_string(),
      embedding_model: "nomic-embed-text:latest".to_string(),
      embedding_url: "http://localhost:11434/api/embed".to_string(),
    }
  }

  fn loader(&self, recursive: bool) -> Vec<Document> {
    let mut documents = Vec::new();

    let paths = std::fs::read_dir(&self.dataset_path).unwrap();
    for entry in paths {
      let path = entry.unwrap().path();
      if path.is_file() {
        let content = std::fs::read_to_string(&path).unwrap();
        let mut metadata = HashMap::new();
        metadata.insert("filename".to_string(), path.to_str().unwrap().to_string());

        documents.push(Document {
          page_content: content,
          metadata,
        });
      } else if path.is_dir() && recursive {
        let sub_rag = RAG::new(path.to_str().unwrap(), &self.embedding_url);
        documents.extend(sub_rag.loader(true));
      }
    }
    return documents
  }
}
