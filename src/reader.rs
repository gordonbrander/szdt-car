use cid::Cid;
use futures::Stream;
use serde::de::DeserializeOwned;
use tokio::io::AsyncRead;

use crate::{
    error::Error,
    header::CarHeader,
    util::{ld_read, read_node},
};

/// Reads CAR files that are in a BufReader
#[derive(Debug)]
pub struct CarReader<R> {
    reader: R,
    header: Vec<u8>,
    buffer: Vec<u8>,
}

impl<R> CarReader<R>
where
    R: AsyncRead + Unpin,
{
    /// Creates a new CarReader and parses the CarHeader
    pub async fn new(mut reader: R) -> Result<Self, Error> {
        let mut buffer = Vec::new();

        match ld_read(&mut reader, &mut buffer).await? {
            Some(buf) => Ok(CarReader {
                reader,
                header: buf.to_vec(),
                buffer,
            }),
            None => Err(Error::Parsing(
                "failed to parse uvarint for header".to_string(),
            )),
        }
    }

    /// Deserializes a header from this car file, using a Serde deserializable type.
    /// This method allows for retreival of arbitrary metadata from the CBOR CAR header.
    pub fn deserialize_header<T: DeserializeOwned>(&self) -> Result<T, Error> {
        let header: T = serde_ipld_dagcbor::from_slice(&self.header)
            .map_err(|e| Error::Parsing(e.to_string()))?;
        Ok(header)
    }

    /// Returns the header of this car file.
    pub fn header(&self) -> Result<CarHeader, Error> {
        CarHeader::decode(&self.header)
    }

    /// Returns the next IPLD Block in the buffer
    pub async fn next_block(&mut self) -> Result<Option<(Cid, Vec<u8>)>, Error> {
        read_node(&mut self.reader, &mut self.buffer).await
    }

    pub fn stream(self) -> impl Stream<Item = Result<(Cid, Vec<u8>), Error>> {
        futures::stream::try_unfold(self, |mut this| async move {
            let maybe_block = read_node(&mut this.reader, &mut this.buffer).await?;
            Ok(maybe_block.map(|b| (b, this)))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use cid::Cid;
    use futures::TryStreamExt;
    use multihash_codetable::MultihashDigest;

    use crate::{header::CarHeaderV1, writer::CarWriter};

    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    async fn car_write_read() {
        let digest_test = multihash_codetable::Code::Blake3_256.digest(b"test");
        let cid_test = Cid::new_v1(0x71, digest_test);

        let digest_foo = multihash_codetable::Code::Blake3_256.digest(b"foo");
        let cid_foo = Cid::new_v1(0x71, digest_foo);

        let header = CarHeader::V1(CarHeaderV1::from(vec![cid_foo]));

        let mut buffer = Vec::new();
        let mut writer = CarWriter::new(header, &mut buffer);
        writer.write(cid_test, b"test").await.unwrap();
        writer.write(cid_foo, b"foo").await.unwrap();
        writer.finish().await.unwrap();

        let reader = Cursor::new(&buffer);
        let car_reader = CarReader::new(reader).await.unwrap();
        let files: Vec<_> = car_reader.stream().try_collect().await.unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].0, cid_test);
        assert_eq!(files[0].1, b"test");
        assert_eq!(files[1].0, cid_foo);
        assert_eq!(files[1].1, b"foo");
    }
}
