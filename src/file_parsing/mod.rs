use std::{fs::File, io::Read, collections::VecDeque};

use crate::backend::map::TileType;

enum ReadingResult{
    Tile(TileType),
    CR
}

fn read_char(character : char) -> ReadingResult{
    match character{
    '\u{25A0}' => return ReadingResult::Tile(TileType::Wall),
    '\u{25A1}' => return ReadingResult::Tile(TileType::Free),
    '\u{25A3}' => return ReadingResult::Tile(TileType::Separator),
    '\n' => return ReadingResult::CR,
      _ => panic!("Unknown character in file")
    }
}

pub fn read_map(path : &str) -> VecDeque<Vec<TileType>> {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut map_sto = VecDeque::new();
    let mut line =  Vec::new();
    let mut line_standard_len_opt = None;
    for character in content.chars(){
    
        match read_char(character){
            ReadingResult::Tile(tile) => {
                line.push(tile);
            },
            ReadingResult::CR =>{

                if let Some(line_standard_len) = line_standard_len_opt{
                    assert_eq!(line_standard_len, line.len(), "All lines of the files are not of the same length");
                }

                line_standard_len_opt = Some(line.len());
                map_sto.push_front(line); 
                line =  Vec::new();
            }
        }
    }
    map_sto

}