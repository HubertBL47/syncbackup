use std::fs;
use super::super::errorcode::*;
use std::path::PathBuf;
use std::time::SystemTime;
use super::storage::Storage;

pub struct Folder<'a>{
    path: &'a PathBuf,
    metadata: fs::Metadata,
}

// ajouter des verifications lorsque c'est des folders

impl<'a> Storage<'a> for Folder<'a>{
    fn new(path: &'a PathBuf, metadata: fs::Metadata) -> Folder<'a>{
        
        return Folder{
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

    fn copy_to(&self, location: &Folder) -> Result<u64, Error>{
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