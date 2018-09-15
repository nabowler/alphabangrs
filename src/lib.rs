extern crate rodio;

use std::io;
use std::sync::Arc;
use std::convert::AsRef;

pub struct Sound (Arc<Vec<u8>>);

impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Sound {
    pub fn load(bytes: &[u8]) -> io::Result<Sound> {
//        let mut buf = Vec::new();
        let mut buf = bytes.iter().map(|b| b.to_owned()).collect();
//        let mut buf = vec!(bytes);
        Ok(Sound(Arc::new(buf)))
    }
    pub fn cursor(self: &Self) -> io::Cursor<Sound> {
        io::Cursor::new(Sound(self.0.clone()))
    }
    pub fn decoder(self: &Self) -> rodio::Decoder<io::Cursor<Sound>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}