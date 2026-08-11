use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentFormat {
    Docx,
    Pdf,
    Pptx,
    Xlsx,
    Csv,
    Markdown,
    PlainText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: String,
    pub format: DocumentFormat,
    pub word_count: usize,
    pub page_count: usize,
}

pub struct DocumentProcessor;

impl DocumentProcessor {
    pub fn detect_format<P: AsRef<Path>>(path: P) -> DocumentFormat {
        match path.as_ref().extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase()).as_deref() {
            Some("docx") => DocumentFormat::Docx,
            Some("pdf") => DocumentFormat::Pdf,
            Some("pptx") => DocumentFormat::Pptx,
            Some("xlsx") => DocumentFormat::Xlsx,
            Some("csv") => DocumentFormat::Csv,
            Some("md") => DocumentFormat::Markdown,
            _ => DocumentFormat::PlainText,
        }
    }

    pub fn generate_document<P: AsRef<Path>>(path: P, title: &str, content: &str) -> Result<DocumentMetadata, Box<dyn std::error::Error>> {
        let fmt = Self::detect_format(&path);
        let word_count = content.split_whitespace().count();
        
        match fmt {
            DocumentFormat::Markdown | DocumentFormat::PlainText => {
                fs::write(&path, content)?;
            }
            DocumentFormat::Csv => {
                let csv_header = "Field,Value\nTitle,".to_string() + title + "\nContent,\"" + &content.replace("\"", "\"\"") + "\"";
                fs::write(&path, csv_header)?;
            }
            _ => {
                // Generate structured format text document
                let doc_payload = format!("# {}\n\n{}", title, content);
                fs::write(&path, doc_payload)?;
            }
        }

        Ok(DocumentMetadata {
            title: title.to_string(),
            format: fmt,
            word_count,
            page_count: (word_count / 250).max(1),
        })
    }
}
