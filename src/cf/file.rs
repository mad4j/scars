use std::fmt;

use super::common_types::ErrorNumberType;

/**
 * This exception indicates a file-related error occurred.
 * The message provides information describing the error.
 */

#[derive(Debug)]
pub struct FileException {
    pub error_number: ErrorNumberType,
    pub message: String,
}

impl FileException {
    pub fn new(error_number: ErrorNumberType, message: String) -> FileException {
        FileException {
            error_number,
            message,
        }
    }
}

impl fmt::Display for FileException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


/**
 * This exception indicates an error occurred during a read
 * or write operation to a File. The message is component-dependent,
 * providing additional information describing the reason for
 * the error. 
 */
#[derive(Debug)]
pub struct IOException {
    pub error_number: ErrorNumberType,
    pub message: String,
}

impl fmt::Display for IOException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/**
 * This exception indicates the file pointer is out of range based upon
 * the current file size. 
 */
#[derive(Debug)]
pub struct InvalidFilePointer {
}

impl fmt::Display for InvalidFilePointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/**
 * This interface provides the ability to read and write files
 * residing within a distributed FileSystem. A file can be thought of
 * conceptually as a sequence of octets with a current filePointer
 * describing where the next read or write will occur. 
 */
pub trait FileTrait {

    fn get_file_name(&self) -> String;
    fn get_file_pointer(&self) -> usize;
    fn set_file_pointer(&self, file_pointer: usize) -> Result<(), InvalidFilePointer>;
    fn read(&self, data: Vec<u8>, length: usize) -> Result<(), IOException>;
    fn write(&self, data: Vec<u8>) -> Result<(), IOException>;
    fn size_of(&self) -> Result<usize, FileException>;
    fn close(&self) -> Result<(), FileException>;
}


pub struct File {
    path: String,
    handle: std::fs::File,
}

impl File {
    pub fn open(path: String) -> Result<File, FileException> {
        let data_result = std::fs::File::open(path);
        match data_result {
            Ok(handle) => Ok(File { path, handle}),
            Err(e) => Err(FileException::new(ErrorNumberType::CF_EBADF, format!("Unable to open {}", path))),
        }
    }
}


impl FileTrait for File {
    fn get_file_name(&self) -> String {
        todo!()
    }

    fn get_file_pointer(&self) -> usize {
        todo!()
    }

    fn set_file_pointer(&self, file_pointer: usize) -> Result<(), InvalidFilePointer> {
        todo!()
    }

    fn read(&self, data: Vec<u8>, length: usize) -> Result<(), IOException> {
        todo!()
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