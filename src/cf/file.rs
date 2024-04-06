use std::{io::{Read, Seek, SeekFrom, Write}, path::Path};
use thiserror::Error;

use super::common_types::ErrorNumberType;

/**
 * Convienence enum definition that includes all FileTrait errors.
 */
#[derive(Error, Debug)]
pub enum FileError {
    /**
     * This exception indicates a file-related error occurred.
     * The message provides information describing the error.
     */
    #[error("FileException: num: {error_number:?}, msg: '{message}'.")]
    FileException {
        error_number: ErrorNumberType,
        message: String,
    },
    /**
     * This exception indicates an error occurred during a read
     * or write operation to a File. The message is component-dependent,
     * providing additional information describing the reason for
     * the error.
     */
    #[error("IOException: num: {error_number:?}, msg: '{message}'.")]
    IOException {
        error_number: ErrorNumberType,
        message: String,
    },
    /**
     * This exception indicates the file pointer is out of range based upon
     * the current file size.
     */
    #[error("InvalidFilePointer.")]
    InvalidFilePointer,
}

impl From<std::io::Error> for FileError {
    fn from(value: std::io::Error) -> Self {
        FileError::IOException {
            error_number: value.kind().into(),
            message: value.to_string(),
        }
    }
}

/*
 * Convienence type definition that includes all FileTrait returned errors.
 */
pub type Result<T, E = FileError> = anyhow::Result<T, E>;

/**
 * This interface provides the ability to read and write files
 * residing within a distributed FileSystem. A file can be thought of
 * conceptually as a sequence of octets with a current filePointer
 * describing where the next read or write will occur.
 */
pub trait FileTrait {
    /// The readonly attribute contains the file name given to the FileSystem open/create operation.
    fn file_name(&self) -> &String;

    /// The readonly attribute contains the file position where the next read or write will occur.
    fn file_pointer(&self) -> u64;

    /// Applications require the read operation in order to retrieve data from remote files.
    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, FileError>;

    /// This operation writes data to the file referenced.
    fn write(&mut self, data: &Vec<u8>) -> Result<()>;

    /// This operation returns the current size of the file.
    fn size_of(&self) -> Result<u64>;

    /// This operation releases any OE file resources associated with the component.
    fn close(&mut self) -> Result<()>;

    /// This operation positions the file pointer where next read or write will occur.
    fn set_file_pointer(&mut self, file_pointer: u64) -> Result<()>;
}

#[derive(Error, Debug)]
#[error("Invalid file handle.")]
struct NoneFileHandleError;

impl From<NoneFileHandleError> for FileError {
    fn from(value: NoneFileHandleError) -> Self {
        FileError::FileException {
            error_number: ErrorNumberType::CF_EBADF,
            message: format!("{value}"),
        }
    }
}

#[derive(Debug)]
pub struct File<'a> {
    file_name: &'a String,
    file_handle: Option<std::fs::File>,
    file_pointer: u64,
}

impl<'a> File<'a> {
    pub fn open(file_name: &'a String, root_path: &Path) -> Result<File<'a>> {

        let file_handle = std::fs::File::open(root_path.join(file_name))?;

        Ok(File {
            file_name,
            file_handle: Some(file_handle),
            file_pointer: 0u64,
        })
    }

    pub fn create(file_name: &'a String, root_path: &Path) -> Result<File<'a>> {

        let file_handle = std::fs::File::create(root_path.join(file_name))?;

        Ok(File {
            file_name,
            file_handle: Some(file_handle),
            file_pointer: 0u64,
        })
    }
}

impl<'a> FileTrait for File<'a> {

    /** 
     * SCA320
     * The readonly fileName attribute shall return the pathname used as the input
     * fileName parameter of the FileSystem::create operation when the file was
     * created.
     */
    fn file_name(&self) -> &'a String {
        self.file_name
    }

    /**
     * SCA321
     * The readonly filePointer attribute shall return the current file position.
     */
    fn file_pointer(&self) -> u64 {
        self.file_pointer
    }


    /**
     * SCA322
     * The read operation shall read, from the referenced file, the number of octets
     * specified by the input length parameter and advance the value of the filePointer
     * attribute by the number of octets actually read.
     * SCA323
     * The read operation shall read less than the number of octets specified in the
     * input length parameter, when an end‐of‐file is encountered.
     * SCA324
     * The read operation shall return a CF::OctetSequence that equals the number of
     * octets actually read from the file via the out data parameter.
     * SCA325
     * If the filePointer attribute value reflects the end of the file, the read operation
     * shall return a zero‐length CF::OctetSequence.
     * SCA326 
     * The read operation shall raise the IOException when a read error occurs.
     */
    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize> {
        //verify if 'file_handle' is still valid
        let h = self.file_handle.as_mut().ok_or(NoneFileHandleError)?;

        let result = h.read(buffer)?;
        self.file_pointer += result as u64;

        Ok(result)
    }

    /**
     * SCA327
     * The write operation shall write data to the file referenced.
     * SCA328
     * The write operation shall increment the filePointer attribute to reflect the
     * number of octets written, when the operation is successful.
     * SCA329
     * If the write operation is unsuccessful, the value of the filePointer attribute shall
     * maintain or be restored to its value prior to the write operation call.
     * SCA330
     * The write operation shall raise the IOException when a write error occurs.
     */
    fn write(&mut self, buffer: &Vec<u8>) -> Result<()> {
        //verify if 'file_handle' is still valid
        let h = self.file_handle.as_mut().ok_or(NoneFileHandleError)?;

        // write to native file until whole buffer is consumed
        let target = buffer.len();
        let mut actual = 0;
        while actual < target {
            let result = h.write(&buffer[actual..])?;
            actual += result;
            self.file_pointer += result as u64;
        }

        // return ok
        Ok(())
    }

    /**
     * SCA331
     * The sizeOf operation shall return the number of octets stored in the file.
     * SCA443
     * The sizeOf operation shall raise the CF::FileException when a file‐related error
     * occurs (e.g., file does not exist anymore).
     */
    fn size_of(&self) -> Result<u64> {
        let h = self.file_handle.as_ref().ok_or(NoneFileHandleError)?;

        let metadata = h.metadata()?;
        Ok(metadata.len())
    }

    /**
     * SCA333
     * The close operation shall make the file unavailable to the component.
     * SCA334
     * The close operation shall raise the CF::FileException when it cannot successfully
     * close the file.
     */
    fn close(&mut self) -> Result<()> {
        self.file_handle = None;
        Ok(())
    }


    /**
     * SCA335
     * The setFilePointer operation shall set the filePointer attribute value to the input
     * filePointer.
     * SCA336
     * The setFilePointer operation shall raise the CF::FileException when the file
     * pointer for the referenced file cannot be set to the value of the input filePointer
     * parameter.
     * SCA337
     * The setFilePointer operation shall raise the InvalidFilePointer exception when
     * the value of the filePointer parameter exceeds the file size.
     */
    fn set_file_pointer(&mut self, file_pointer: u64) -> Result<()> {
        
        //verify if 'file_handle' is still valid
        let h = self.file_handle.as_mut().ok_or(NoneFileHandleError)?;

        //not allowed to move beyond end of file
        if file_pointer > h.metadata()?.len() {
            return Err(FileError::InvalidFilePointer);
        }

        //move native handler to requested position
        h.seek(SeekFrom::Start(file_pointer))?;

        //update internal state
        self.file_pointer = file_pointer;

        //return ok
        Ok(())
    }
}
