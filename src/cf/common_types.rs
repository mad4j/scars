use std::fmt;

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

 #[allow(dead_code)]
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