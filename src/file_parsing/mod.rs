use std::{fs::File, io::Read, collections::VecDeque};

use crate::backend::map::TileType;

enum ReadingResult{
    Tile(TileType),
    NewLine,
    Ignored
}

fn read_char(character : char) -> ReadingResult{
    match character{
    '\u{25A3}' => return ReadingResult::Tile(TileType::Wall), 
    '\u{25A2}' => return ReadingResult::Tile(TileType::Free),
    '\u{25A4}' => return ReadingResult::Tile(TileType::Separator),
    '\n' => return ReadingResult::NewLine,
    '\r' => return ReadingResult::Ignored, // Present in Windows only
      _ => panic!("unknown character in file: {}", character)
    }
}

pub fn read_map(path : &str) -> VecDeque<Vec<TileType>> {

    // Read the file and store its content in a string
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut map_sto = VecDeque::new();
    let mut line =  Vec::new();
    let mut line_standard_len_opt = None;

    // Iterate over all chars and interpret them
    for character in content.chars(){
    
        match read_char(character){
            
            //This character correspond to a tile, which is added to the current map line
            ReadingResult::Tile(tile) => {
                line.push(tile);
            },

            // New line in map
            ReadingResult::NewLine =>{
                if let Some(line_standard_len) = line_standard_len_opt{
                    assert_eq!(line_standard_len, line.len(), "All lines of the files are not of the same length");
                }
                line_standard_len_opt = Some(line.len());
                map_sto.push_front(line); 
                line =  Vec::new();
            }
            ReadingResult::Ignored => {/*NOP*/},
        }
    }
    map_sto

}