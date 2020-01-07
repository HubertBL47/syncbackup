use super::super::errorcode::Error;
use std::path::PathBuf;
use std::time::SystemTime;
use std::fs;

pub trait Storage<'a>{
    fn new(path: &'a PathBuf, metadat: fs::Metadata) -> Self;

    fn copy_to(&self, location: &Self) -> Result<u64, Error>;
    fn modified(&self) -> Result<SystemTime, Error>;

    fn path(&self) -> &PathBuf;

}