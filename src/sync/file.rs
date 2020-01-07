use std::fs;
use super::super::errorcode::*;
use std::path::PathBuf;
use std::time::SystemTime;
use super::storage::Storage;

pub struct File<'a>{
    path: &'a PathBuf,
    metadata: fs::Metadata,
}

// ajouter des verifications lorsque c'est des folders
// utiliser des traits avec plusieur struct et non une seul struct

impl<'a> Storage<'a> for File<'a>{
    fn new(path: &'a PathBuf, metadata: fs::Metadata) -> File<'a>{
        
        return File{
            path,
            metadata,
        };
    }

    fn modified(&self) -> Result<SystemTime, Error>{
        return match self.metadata.modified(){
            Ok(time) => Ok(time),
            Err(_) => Err(Error::new(ErrorCode::MetadataError, format!("Error accessing metadata of {:?}", self.path.file_name().unwrap())))
        }
    }

    // changer le second file pour utiliser seulement une location (path)
    fn copy_to(&self, location: &File) -> Result<u64, Error>{
        match fs::copy(&self.path, location.path()){
            Ok(byte) => {println!("{} bytes copied", byte);
                         return Ok(byte)},
            Err(e) => return Err(Error::new(ErrorCode::CopyError, format!("Copy failed dues to: {}", e))),
        };
    }

    fn path(&self) -> &PathBuf{
        return self.path;
    }
}