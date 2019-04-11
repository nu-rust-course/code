//! An enumeration to describe file transfer commands.

/// A file transfer command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    /// Uploads a local file to the remote host.
    UploadFile(String, String),
    /// Downloads a file from the remote host.
    DownloadFile(String, String),
    /// Removes a file on the remote host.
    RemoveFile(String),
    /// Lists the files on the remote host.
    ListFiles,
}

impl Command {
    pub fn src_file(&self) -> Option<&str> {
        match self {
            Command::UploadFile(src, _)   => Some(src),
            Command::DownloadFile(src, _) => Some(src),
            _                             => None
        }
    }

    pub fn remote_file(&self) -> Option<&str> {
        use self::Command::*;

        match self {
            UploadFile(_, rem)   => Some(rem),
            DownloadFile(rem, _) => Some(rem),
            RemoveFile(rem)      => Some(rem),
            _                    => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Command::*;

    #[test]
    fn upload_file() {
        let cmd = UploadFile("here".to_owned(), "there".to_owned());
        assert_eq!( cmd.src_file(),    Some("here") );
        assert_eq!( cmd.remote_file(), Some("there") );
    }

    #[test]
    fn download_file() {
        let cmd = DownloadFile("there".to_owned(), "here".to_owned());
        assert_eq!( cmd.src_file(),    Some("there") );
        assert_eq!( cmd.remote_file(), Some("there") );
    }

    #[test]
    fn remote_file() {
        let cmd = RemoveFile("there".to_owned());
        assert_eq!( cmd.src_file(),    None );
        assert_eq!( cmd.remote_file(), Some("there") );
    }

    #[test]
    fn list_files() {
        let cmd = ListFiles;
        assert_eq!( cmd.src_file(),    None );
        assert_eq!( cmd.remote_file(), None );
    }
}
