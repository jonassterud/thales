mod info;

pub use info::{TorrentInfo, File};
use std::path::Path;
use anyhow::Result;
use bencode::{Val, decode};
use std::path::PathBuf;
use anyhow::anyhow;

/// Torrent struct.
pub struct Torrent {
    /// Info about the torrent
    pub info: TorrentInfo,
    /// Announce URL of the tracker
    pub announce: Vec<u8>,
    /// Creation time of torrent, in standard UNIX epoch format
    pub creation_date: Vec<u8>,
    /// Comment from the author
    pub comment: Vec<u8>,
    /// Name and version of the program used to create the torrent
    pub created_by: Vec<u8>,
    //pub encoding: String,
}

impl Torrent {
    pub fn from_file(path: &Path) -> Result<Torrent> {
        let content = std::fs::read(path)?;
        let decoded = decode(&content);
        
        // TODO: Refactor!
        if let Val::Dictionary(dict) = decoded {
            // Get Torrent values
            let announce = dict.get(b"announce".as_ref()).unwrap().as_byte_string()?;
            let creation_date = dict.get(b"creation date".as_ref()).unwrap_or(&Val::ByteString(vec![])).as_byte_string()?;
            let comment = dict.get(b"comment".as_ref()).unwrap_or(&Val::ByteString(vec![])).as_byte_string()?;
            let created_by = dict.get(b"created by".as_ref()).unwrap_or(&Val::ByteString(vec![])).as_byte_string()?;
        
            // Get TorrentInfo values
            let info_dict = dict.get(b"info".as_ref()).unwrap().as_dictionary()?;
            let name = info_dict.get(b"name".as_ref()).unwrap().as_byte_string()?;
            let piece_length = info_dict.get(b"piece length".as_ref()).unwrap().as_number()?;
            let pieces = info_dict.get(b"pieces".as_ref()).unwrap().as_byte_string()?;
            let mut files: Vec<File> = vec![];

            // Handle both single-file mode and multi-file mode
            let is_multi_file = info_dict.contains_key(b"files".as_ref());
            if is_multi_file {
                let files_temp = info_dict.get(b"files".as_ref()).unwrap().as_list()?;
                for file_temp in files_temp {
                    let file_temp = file_temp.as_dictionary()?;
                    let file_length = file_temp.get(b"length".as_ref()).unwrap().as_number()?;
                    let mut file_path = PathBuf::new();
                    
                    // Assemble path from the path list
                    let file_path_list = file_temp.get(b"path".as_ref()).unwrap().as_list()?;
                    for file_path_part in file_path_list {
                        let file_path_part = String::from_utf8(file_path_part.as_byte_string()?)?;
                        file_path.push(file_path_part);
                    }

                    files.push(File {
                        length: file_length,
                        path: file_path,
                    });
                }
            } else {
                let file_length = info_dict.get(b"length".as_ref()).unwrap();

                files.push(File {
                    length: file_length.as_number()?,
                    path: PathBuf::new(),
                });
            }

            return Ok(Torrent {
                info: TorrentInfo {
                    name: name,
                    piece_length: piece_length,
                    pieces: pieces,
                    files: files,
                },
                announce: announce,
                creation_date: creation_date,
                comment: comment,
                created_by: created_by,
            })
        }

        Err(anyhow!("failed parsing torrent"))
    }


}