use std::io::Read;
use anyhow::Result;
use thiserror::Error;

use super::common_types::ErrorNumberType;

#[derive(Error, Debug)]
#[error("FileException: num: {error_number:?}, msg: '{message}'.")]
/**
 * This exception indicates a file-related error occurred.
 * The message provides information describing the error.
 */
pub struct FileException {
    error_number: ErrorNumberType,
    message: String,
}

impl From<std::io::Error> for FileException {
    fn from(value: std::io::Error) -> Self {
        FileException { error_number: value.kind().into(), message: value.to_string() }
    }
}

/**
 * This exception indicates an error occurred during a read
 * or write operation to a File. The message is component-dependent,
 * providing additional information describing the reason for
 * the error. 
 */
#[derive(Error, Debug)]
#[error("IOException: num: {error_number:?}, msg: '{message}'.")]
pub struct IOException {
    error_number: ErrorNumberType,
    message: String,
}

/**
 * This exception indicates the file pointer is out of range based upon
 * the current file size. 
 */
#[derive(Error, Debug)]
#[error("InvalidFilePointer.")]
pub struct InvalidFilePointer {

}

/**
 * This interface provides the ability to read and write files
 * residing within a distributed FileSystem. A file can be thought of
 * conceptually as a sequence of octets with a current filePointer
 * describing where the next read or write will occur. 
 */
pub trait FileTrait {

    fn get_file_name(&self) -> &String;
    fn get_file_pointer(&self) -> usize;
    fn set_file_pointer(&self, file_pointer: usize) -> Result<(), InvalidFilePointer>;
    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize>;
    fn write(&self, data: Vec<u8>) -> Result<(), IOException>;
    fn size_of(&self) -> Result<usize, FileException>;
    fn close(&self) -> Result<(), FileException>;
}


pub struct File<'a> {
    file_name: &'a String,
    handle: std::fs::File,
}

impl<'a> File<'a> {
    pub fn open(file_name: &'a String) -> Result<File<'a>, FileException> {
        
        let handle = std::fs::File::open(file_name)?;
        Ok(File { file_name, handle })
    }

    pub fn create(file_name: &'a String) -> Result<File<'a>, FileException> {

        let handle = std::fs::File::create(file_name)?;
        Ok(File { file_name, handle })
    }
}


impl<'a> FileTrait for File<'a> {
    fn get_file_name(&self) -> &'a String {
        self.file_name
    }

    fn get_file_pointer(&self) -> usize {
        todo!()
    }

    fn set_file_pointer(&self, file_pointer: usize) -> Result<(), InvalidFilePointer> {
        todo!()
    }

    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize> {
        let result = self.handle.read(buffer)?;
        Ok(result)
    }

    fn write(&self, data: Vec<u8>) -> Result<(), IOException> {
        todo!()
    }

    fn size_of(&self) -> Result<usize, FileException> {
        todo!()
    }

    fn close(&self) -> Result<(), FileException> {
        todo!()
    }
}