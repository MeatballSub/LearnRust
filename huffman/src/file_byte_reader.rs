use std::fs::File;
use std::io::BufReader;
use std::io::Bytes;
use std::io::Error;
use std::io::Read;
use std::path::Path;

struct FileByteReader
{
    bytes: Bytes<BufReader<File>>,
}

impl FileByteReader
{
    pub fn new(path: &Path) -> Self
    {
        let error_msg = format!("Couldn't open: {:?}", path);
        let file = File::open(path).expect(error_msg.as_str());
        let buff_reader = BufReader::new(file);
        Self { bytes: buff_reader.bytes() }
    }

    pub fn next(&mut self) -> Option<Result<u8, Error>> { self.bytes.next() }
}

#[cfg(test)]
mod tests
{
    use std::io::Write;
    use std::path::PathBuf;

    use super::*;

    struct TestFile
    {
        path: PathBuf,
    }

    impl TestFile
    {
        pub fn create(path: PathBuf) -> Self
        {
            let error_msg = format!("Couldn't create: {:?}", path);
            let mut file = File::create_new(path.clone()).expect(error_msg.as_str());
            file.write(b"Hello World!").expect(error_msg.as_str());
            Self { path }
        }
    }

    impl Drop for TestFile
    {
        fn drop(&mut self) { let _ = std::fs::remove_file(self.path.as_path()); }
    }

    #[test]
    fn test_read()
    {
        let path = Path::new("kmd_FileByteReaderTest.txt");
        let test_file = TestFile::create(path.to_path_buf());

        let mut reader = FileByteReader::new(path);
        assert_eq!(reader.next().unwrap().unwrap(), b'H');
        assert_eq!(reader.next().unwrap().unwrap(), b'e');
        assert_eq!(reader.next().unwrap().unwrap(), b'l');
        assert_eq!(reader.next().unwrap().unwrap(), b'l');
        assert_eq!(reader.next().unwrap().unwrap(), b'o');
        assert_eq!(reader.next().unwrap().unwrap(), b' ');
        assert_eq!(reader.next().unwrap().unwrap(), b'W');
        assert_eq!(reader.next().unwrap().unwrap(), b'o');
        assert_eq!(reader.next().unwrap().unwrap(), b'r');
        assert_eq!(reader.next().unwrap().unwrap(), b'l');
        assert_eq!(reader.next().unwrap().unwrap(), b'd');
        assert_eq!(reader.next().unwrap().unwrap(), b'!');

        drop(test_file);
    }
}
