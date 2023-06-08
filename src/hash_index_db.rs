use std::fs::{ OpenOptions, File, create_dir_all, read_dir};
use std::io::{ Result, Write };
use std::path::{PathBuf};


pub const SEGMENTS_FOLDER: &str = "hashIdxSegments";
pub const SEGMENT_THRESHOLD: usize = 4096;

pub type SegmentID = usize;

#[derive(Clone)]
struct Segment
{
    size: usize,
    pub id: SegmentID,
    mutable: bool,
    file_name: String,
}

impl Segment
{
    fn new(id: SegmentID, name: String) -> Self
    {
        Self 
        {
            size: 0,
            id,
            mutable: true,
            file_name: name,
        }
    }

    fn is_above_threshold(&self) -> bool
    {
        self.size >= SEGMENT_THRESHOLD
    }
}


pub struct HashIdxDB
{
    segment_vector: Vec<Segment>,
    segments_folder: PathBuf,
}


impl HashIdxDB
{
    pub fn new() -> Self
    {
        let segments_folder = PathBuf::from(SEGMENTS_FOLDER);
        // create directoy for segemnts if not already created
        create_dir_all(&segments_folder).unwrap();

        // load existing segments
        let mut segment_vector: Vec<Segment> = Vec::new();

        for path in read_dir(&segments_folder).unwrap() {
            let file_name = path.unwrap().file_name().to_str().unwrap().to_string();

            let str_idx_str = file_name.find("_").unwrap() + 1;
            let seg_id = file_name.get(str_idx_str..).unwrap().parse::<usize>().unwrap();

            let new_seg = Segment::new(seg_id, file_name);
            segment_vector.push(new_seg);
        }

        Self{ 
            segment_vector,
            segments_folder,
        }
    }

    fn get_current_segment(&self) -> Option<&Segment>
    {
        if self.segment_vector.len() == 0
        {
            None
        }
        else {
            Some(self.segment_vector.last().unwrap())
        }
    }

    fn add_segment(&mut self) -> Result<&Segment> 
    {
        // get new segment id 
        let new_id: SegmentID;
        match self.get_current_segment() {
            Some(seg) => {new_id = seg.id},
            _ => { new_id = 0 }
        }

        let new_segment_name = format!("seg_{}", new_id);
        let path_segment = self.segments_folder.join(&new_segment_name);
        let _ = File::create(path_segment)?;

        let new_segment = Segment::new(new_id, new_segment_name);

        self.segment_vector.push(new_segment.clone());

        Ok(&new_segment)
    }
    
    fn open_file_append(&mut self) -> File
    {
        // get current segment file name and create if no file names yet
        let curr_file_name: String;
        match self.get_current_segment() {
            Some(seg) => {curr_file_name = seg.file_name.clone()},
            _ => {
                // no segments yet, create new one
                self.add_segment().unwrap();
                curr_file_name = self.get_current_segment().unwrap().file_name.clone();
            }
        }
        let path_to_seg = self.segments_folder.join(curr_file_name);
         OpenOptions::new()
            .append(true)
            .create(true)
            .open(path_to_seg)
            .unwrap()
    }

    pub fn write(&mut self, key: &str, value: &str) -> Result<()> 
    {
        //TODO: do writing logic inside of Segment 
        // check if first time writing
        let mut curr_seg;
        match self.get_current_segment() {
            None => {
                self.add_segment().unwrap();
                curr_seg = self.get_current_segment().unwrap();
            },
            Some(seg) => {curr_seg = seg},
        };
        // check if current segment is above threshold
        if curr_seg.is_above_threshold()
        {
            curr_seg = self.add_segment().unwrap();
        }
        // open file and add segment if first time writing to DB
        let mut db_file = self.open_file_append();

        let content = format!("{}:{}\n", key, value);

        // get size of file after writing
        let new_size = curr_seg.size + content.len();
        curr_seg.size = new_size;
        println!("New size: {}", curr_seg.size);

        write!(db_file, "{}", content)
    }

    pub fn read(&mut self, key: &str) -> std::io::Result<String>
    {
        Ok("".to_string())
    }


}
