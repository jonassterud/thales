use std::path::PathBuf;

/// Represents a file, both in single-file mode and multi-file mode.
#[derive(Debug)]
pub struct File {
    /// Length of the file in bytes
    pub length: i64,
    //pub md5sum: String,
    /// Path to where to save the file
    pub path: PathBuf,
}

/// Contains info about the torrent.
#[derive(Debug)]
pub struct TorrentInfo {
    /// The filename in single-file mode, or the directory name in multi-file mode
    pub name: Vec<u8>,
    /// Number of bytes in each piece
    pub piece_length: i64,
    /// Concatenation of all 20-byte SHA1 hash values, one per piece
    pub pieces: Vec<u8>,
    //pub private: bool,
    /// List of files
    pub files: Vec<File>,
}
