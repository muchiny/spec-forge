pub mod docx_reader;
pub mod markdown_reader;
pub mod pdf_reader;
pub mod story_parser;
pub mod yaml_reader;

/// Taille maximale d'un fichier d'entree (10 Mo)
pub const MAX_INPUT_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Verifie que la taille du fichier ne depasse pas la limite
pub fn check_file_size(path: &std::path::Path) -> Result<(), crate::domain::errors::InputError> {
    match std::fs::metadata(path) {
        Ok(meta) if meta.len() > MAX_INPUT_FILE_SIZE => {
            Err(crate::domain::errors::InputError::ReadError(format!(
                "Fichier trop volumineux: {} ({} octets, limite: {} octets)",
                path.display(),
                meta.len(),
                MAX_INPUT_FILE_SIZE,
            )))
        }
        _ => Ok(()),
    }
}
