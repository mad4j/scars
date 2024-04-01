use std::fmt;
use std::io::ErrorKind;

/**
 * This enum is used to pass error number information in various
 * exceptions. Those exceptions starting with "CF_E" map to the POSIX
 * definitions.
 * The "CF_" has been added to the POSIX exceptions to avoid namespace
 * conflicts. CF_NOTSET is not defined in the POSIX specification.
 * CF_NOTSET is an SCA specific value that is applicable for any
 * exception when the method specific or standard POSIX error values
 * are not appropriate.
 */

 #[allow(non_camel_case_types)]
 #[derive(Debug)]
pub enum ErrorNumberType {
    CF_NOTSET,
    CF_E2BIG,
    CF_EACCES,
    CF_EAGAIN,
    CF_EBADF,
    CF_EBADMSG,
    CF_EBUSY,
    CF_ECANCELED,
    CF_ECHILD,
    CF_EDEADLK,
    CF_EDOM,
    CF_EEXIST,
    CF_EFAULT,
    CF_EFBIG,
    CF_EINPROGRESS,
    CF_EINTR,
    CF_EINVAL,
    CF_EIO,
    CF_EISDIR,
    CF_EMFILE,
    CF_EMLINK,
    CF_EMSGSIZE,
    CF_ENAMETOOLONG,
    CF_ENFILE,
    CF_ENODEV,
    CF_ENOENT,
    CF_ENOEXEC,
    CF_ENOLCK,
    CF_ENOMEM,
    CF_ENOSPC,
    CF_ENOSYS,
    CF_ENOTDIR,
    CF_ENOTEMPTY,
    CF_ENOTSUP,
    CF_ENOTTY,
    CF_ENXIO,
    CF_EPERM,
    CF_EPIPE,
    CF_ERANGE,
    CF_EROFS,
    CF_ESPIPE,
    CF_ESRCH,
    CF_ETIMEDOUT,
    CF_EXDEV,
}

impl fmt::Display for ErrorNumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ErrorKind> for ErrorNumberType {
    fn from(value: ErrorKind) -> Self {
        match value {
            ErrorKind::NotFound => ErrorNumberType::CF_ENOENT,
            ErrorKind::PermissionDenied => ErrorNumberType::CF_EPERM,
            ErrorKind::ConnectionRefused => todo!(),
            ErrorKind::ConnectionReset => todo!(),
            //ErrorKind::HostUnreachable => todo!(),
            //ErrorKind::NetworkUnreachable => todo!(),
            ErrorKind::ConnectionAborted => todo!(),
            ErrorKind::NotConnected => todo!(),
            ErrorKind::AddrInUse => todo!(),
            ErrorKind::AddrNotAvailable => todo!(),
            //ErrorKind::NetworkDown => todo!(),
            ErrorKind::BrokenPipe => todo!(),
            ErrorKind::AlreadyExists => todo!(),
            ErrorKind::WouldBlock => todo!(),
            //ErrorKind::NotADirectory => todo!(),
            //ErrorKind::IsADirectory => todo!(),
            //ErrorKind::DirectoryNotEmpty => todo!(),
            //ErrorKind::ReadOnlyFilesystem => todo!(),
            //ErrorKind::FilesystemLoop => todo!(),
            //ErrorKind::StaleNetworkFileHandle => todo!(),
            ErrorKind::InvalidInput => todo!(),
            ErrorKind::InvalidData => todo!(),
            ErrorKind::TimedOut => todo!(),
            ErrorKind::WriteZero => todo!(),
            //ErrorKind::StorageFull => todo!(),
            //ErrorKind::NotSeekable => todo!(),
            //ErrorKind::FilesystemQuotaExceeded => todo!(),
            //ErrorKind::FileTooLarge => todo!(),
            //ErrorKind::ResourceBusy => todo!(),
            //ErrorKind::ExecutableFileBusy => todo!(),
            //ErrorKind::Deadlock => todo!(),
            //ErrorKind::CrossesDevices => todo!(),
            //ErrorKind::TooManyLinks => todo!(),
            //ErrorKind::InvalidFilename => todo!(),
            //ErrorKind::ArgumentListTooLong => todo!(),
            ErrorKind::Interrupted => todo!(),
            ErrorKind::Unsupported => todo!(),
            ErrorKind::UnexpectedEof => todo!(),
            ErrorKind::OutOfMemory => todo!(),
            ErrorKind::Other => todo!(),
            _ => todo!(),
        }
    }
}

/**
 * This exception indicates an invalid file name was passed
 * to a file service operation. The message provides information
 * describing why the filename was invalid. 
 */
#[derive(Debug)]
 pub struct InvalidFileName {
    pub error_number: ErrorNumberType,
    pub message: String,
}

impl fmt::Display for InvalidFileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/** 
 * This exception indicates an invalid object reference error. 
 */
#[derive(Debug)]
pub struct InvalidObjectReference {
    pub message: String,
}

impl fmt::Display for InvalidObjectReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}