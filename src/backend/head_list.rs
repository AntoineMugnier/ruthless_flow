
use super::heads::{Head, Id};
use crate::utils::{Coordinates, Direction};
use crate::mpsc::Sender;

pub struct HeadList<HeadType: Head>{
    heads_vec: Vec<Option<HeadType>>,
    nb_heads: usize
}

impl <HeadType : Head> HeadList<HeadType>{
    pub fn new() -> HeadList<HeadType>{
        let heads_vec = Vec::new();
        HeadList{heads_vec, nb_heads: 0}
    }
    
    pub fn get_nb_heads(&self) -> usize{
        self.nb_heads
    }

    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut HeadType> {
        let filtering_fn : fn(&mut Option<HeadType>) -> Option<&mut HeadType> = |x : &mut Option<HeadType>| if let Some(head) = x {Some(head)} else {None};
        self.heads_vec.iter_mut().filter_map(filtering_fn)
    }

    #[allow(dead_code)]
    pub fn iter(&mut self) -> impl Iterator<Item = &HeadType> {
        let filtering_fn : fn(& Option<HeadType>) -> Option<& HeadType> = |x : & Option<HeadType>| if let Some(head) = x {Some(head)} else {None};
        self.heads_vec.iter().filter_map(filtering_fn)
    }

    pub fn add_head(&mut self,    
    position: Coordinates,
    coming_from: Direction,
    events_sender: Sender<super::Event>)-> &mut HeadType{

        let mut free_slot_pos : Option<usize> = None;

        // Try to put the head on an empty slot
        for (pos, head) in self.heads_vec.iter_mut().enumerate(){
            if let None = head {
                let new_head = HeadType::new(pos as Id, position, coming_from, events_sender.clone());
                head.replace(new_head);
                free_slot_pos = Some(pos);
                break;
            }
        }
        let new_slot_pos;

        if let Some(free_slot_pos) = free_slot_pos{
            new_slot_pos = free_slot_pos;
        }
        
        else{
            // If no slot is available, push the head on a new slot
            let new_head = HeadType::new(self.heads_vec.len() as Id, position, coming_from, events_sender.clone());
            self.heads_vec.push(Some(new_head));
            new_slot_pos = self.heads_vec.len() -1;
        }

        self.nb_heads+=1; // Increase current number of heads
        self.heads_vec[new_slot_pos].as_mut().unwrap()
    }

    pub fn remove(&mut self, id_of_head_to_remove: Id){
        for head_opt in self.heads_vec.iter_mut(){
            if let Some(head) = head_opt{
                if head.get_id() == id_of_head_to_remove{
                    head_opt.take();
                    self.nb_heads-=1; // Decrease current number of heads
                }
            }

        }
    }
}