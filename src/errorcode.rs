#[derive(Copy, Clone)]
pub enum ErrorCode {
    NoError = 0,
    MissingArgs,
    PathError,
    OptionError,
    CopyError,
    MetadataError,
}

pub struct Error {
    err_code: ErrorCode,
    err_message: String,
}

impl Error{

    pub fn new(err_code: ErrorCode, err_message: String) -> Error{
        return Error{
            err_code,
            err_message,
        };
    }

    pub fn err_code(&self) -> &ErrorCode{
        return &self.err_code;
    }

    pub fn err_message(&self) -> &String{
        return &self.err_message;
    }
}