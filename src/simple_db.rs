use std::fs::{ OpenOptions, File };
use std::io::{ Write };
use std::io::{ BufReader };
extern crate rev_lines;
use rev_lines::RevLines;

/// Simple append only database which never deletes memory
/// 
/// Every key value pair is seperated by a newline character
/// 
/// the most recent key for a given search has precedence
pub struct SimpleDB{
    database_file: String,
}

impl SimpleDB
{
    pub fn new() -> Self
    {
        let database_file = "my_db".to_string();
        Self { 
            database_file,
        }
    }

    fn open_file_append(&self) -> File
    {
         OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.database_file)
            .unwrap()
    }

    pub fn write(&mut self, key: &str, value: &str) -> std::io::Result<()> {
        let content = format!("{}:{}\n",key, value);

        let mut db_file = self.open_file_append();

        write!(db_file, "{}", content)
    }

    pub fn read(&mut self, key: &str) -> std::io::Result<String>
    {
        let file = File::open(&self.database_file)?; // replace filename.txt with your file name
        let reader = BufReader::new(file);
        let thing = RevLines::new(reader).unwrap();

        let mut result: String = String::from("");

        for line in thing {
            if line.contains(key) {
                let div_idx = line.find(":").unwrap() + 1;
                result = line.get(div_idx..).unwrap().to_string();
                break;
            }
        }
        Ok(result)
    }
}