use super::*;

#[test]
fn open_torrent() {
    let path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../../assets/torrents/ubuntu-22.04-desktop-amd64.iso.torrent");
    let path = std::path::Path::new(&path);
    let _torrent = Torrent::from_file(path);

    
}
