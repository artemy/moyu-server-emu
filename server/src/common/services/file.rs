use crate::endpoints::errors::AppError;
use crate::endpoints::errors::AppError::IOError;
use bytes::Bytes;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio_util::io::ReaderStream;

#[derive(Clone)]
pub struct FileService {
    output_dir: String,
}

impl FileService {
    pub fn new(output_dir: String) -> Self {
        Self { output_dir }
    }
    pub fn save_audio_file(&self, filename: &String, bytes: &Bytes) -> Result<(), AppError> {
        let path = Path::new(&self.output_dir).join(filename);
        log::debug!("Saving to file: {}", path.display());
        let mut output_file = File::create(path).map_err(|e| IOError(e.to_string()))?;
        output_file
            .write_all(bytes)
            .map_err(|e| IOError(e.to_string()))
    }

    pub fn save_file_and_return_url(
        &self,
        filename: &String,
        bytes: &Bytes,
    ) -> Result<String, AppError> {
        self.save_audio_file(filename, bytes)?;
        Ok(format!(
            "http://hardware.xiangjiaochuxing.com/audio/{}",
            filename
        ))
    }

    pub async fn get_file_stream(
        &self,
        filename: &String,
    ) -> Result<ReaderStream<tokio::fs::File>, AppError> {
        let path = Path::new(&self.output_dir).join(filename);
        log::info!("Getting file by path: {}", path.display());
        tokio::fs::File::open(path)
            .await
            .map_err(|_| AppError::Generic("File not found".into()))
            .map(ReaderStream::new)
    }
}
