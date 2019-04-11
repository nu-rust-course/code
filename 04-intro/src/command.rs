//! An enumeration to describe file transfer commands.

/*
There are four possible commands:

  - Upload a local file to a remote file
  - Download a remote file to a local file
  - Remove a remote file
  - List all the remote files

How would we represent this in C?
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The operation of a command.
pub enum CommandOp {
    UploadFile,
    DownloadFile,
    RemoveFile,
    ListFiles,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A file transfer command.
pub struct Command0 {
    pub tag: CommandOp,
    pub src: Option<String>,
    pub dst: Option<String>,
}
// Invariants:
//  - self.is_valid()

impl Command0 {
    pub fn is_valid(&self) -> bool {
        match self.tag {
            CommandOp::UploadFile | CommandOp::DownloadFile =>
                self.src.is_some() && self.dst.is_some(),
            CommandOp::RemoveFile =>
                self.src.is_some() && self.dst.is_none(),
            CommandOp::ListFiles =>
                self.src.is_none() && self.dst.is_none(),
        }
    }
}

/*
In Rust we can do better by making the type only capable of
representing valid commands:
*/

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
            _                             => None,
        }
    }

    pub fn remote_file(&self) -> Option<&str> {
        use self::Command::*;

        match self {
            ListFiles =>
                None,
            UploadFile(_, rem) | DownloadFile(rem, _) | RemoveFile(rem) =>
                Some(rem),
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
