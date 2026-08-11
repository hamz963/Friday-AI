use std::path::{Path, PathBuf};
use std::fs;
use zip::ZipArchive;
use serde::{Deserialize, Serialize};

pub mod documents;
pub use documents::{DocumentFormat, DocumentMetadata, DocumentProcessor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub is_dir: bool,
    pub size_bytes: u64,
}

pub struct FileProcessor;

impl FileProcessor {
    pub fn read_file<P: AsRef<Path>>(file_path: P) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        Ok(content)
    }

    pub fn write_file<P: AsRef<Path>>(file_path: P, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = file_path.as_ref().parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(file_path, content)?;
        Ok(())
    }

    pub fn list_dir_contents<P: AsRef<Path>>(dir_path: P) -> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {
        let mut entries = Vec::new();
        if dir_path.as_ref().is_dir() {
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let meta = entry.metadata()?;
                entries.push(FileEntry {
                    path: entry.path().to_string_lossy().to_string(),
                    is_dir: meta.is_dir(),
                    size_bytes: meta.len(),
                });
            }
        }
        Ok(entries)
    }

    pub fn extract_zip<P: AsRef<Path>>(zip_path: P, target_dir: P) -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;
        
        fs::create_dir_all(&target_dir)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => target_dir.as_ref().join(path),
                None => continue,
            };

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        Ok(())
    }

    pub fn scan_directory<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        if dir_path.as_ref().is_dir() {
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    files.extend(Self::scan_directory(&path)?);
                } else {
                    files.push(path);
                }
            }
        }
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_file_read_write() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        FileProcessor::write_file(&file_path, "Hello Friday AI").unwrap();
        assert!(file_path.exists());

        let read_back = FileProcessor::read_file(&file_path).unwrap();
        assert_eq!(read_back, "Hello Friday AI");
    }

    #[test]
    fn test_zip_creation_and_extraction() {
        let temp_dir = tempfile::tempdir().unwrap();
        let zip_file_path = temp_dir.path().join("archive.zip");
        let extract_dir = temp_dir.path().join("extracted");

        // Write a mock zip file
        let file = fs::File::create(&zip_file_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        zip.start_file("hello.txt", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"Hello from ZIP!").unwrap();
        zip.finish().unwrap();

        // Extract using our file processor
        FileProcessor::extract_zip(&zip_file_path, &extract_dir).unwrap();

        let extracted_file = extract_dir.join("hello.txt");
        assert!(extracted_file.exists());
        let content = fs::read_to_string(extracted_file).unwrap();
        assert_eq!(content, "Hello from ZIP!");
    }
}
