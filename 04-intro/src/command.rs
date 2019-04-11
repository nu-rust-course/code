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
