use std::io::{
    Read,
    Seek,
    SeekFrom,
};
use std::path::Path;
use std::fs::File;

use crate::util::{
    versioning::*,
    crc32::crc32,
};

pub struct Bxfs {
    vfs_name: String,
    version: Version,
    datablob: Vec<u8>,
    entries: Vec<Entry>,
}
struct Entry {
    name: String, //name of file/directory
    hash: String, //if dir then is a hash of the dirname
    datatype: Datatype,
    datainfo: [u32; 2], //i0 is datablob index, i1 is data length, if dir data is all the indexes
}
enum Datatype {
    File,
    Archive,
    Directory,
}

impl Bxfs {
    pub fn Load(path: String) {
        let location = Path::new(path.as_str());
        let file = match File::open(&location) {
            Err(why) => panic!("couldn't open {}: {}", path, why),
            Ok(file) => file,
        };
        let header = read_bxfs_meta(file);
        if &header[0..3] == "BXFS".as_bytes() {
            let version = get_vfs_version(header);
            let name = get_vfs_name(header);
            todo!();
        } else {
            panic!("file provided is invalid");
        }
    }
}

fn read_bxfs_meta(mut file: File) -> [u8; 4096] {
    let mut bxfs_header: [u8; 4096] = [0; 4096]; //4kb
    file.seek(SeekFrom::Start(0));
    file.read_exact(&mut bxfs_header).expect("Error reading file");
    return bxfs_header;
}
fn get_vfs_version(header: [u8; 4096]) -> Version {
    Version {
        major_revision: *header.get(4).unwrap(),
        minor_revision: *header.get(5).unwrap(),
        patch_revision: *header.get(6).unwrap(),
    }
}
fn get_vfs_name(header: [u8; 4096]) -> String {
    let size = header[7];
    return String::from_utf8(header[8..size as usize + 7].to_vec())
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
}
