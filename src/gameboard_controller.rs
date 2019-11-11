
use super::Gameboard;
use piston::input::GenericEvent;

pub struct GameboardController{
    gameboard:Gameboard
}

impl GameboardController{
    pub fn new(gameboard:Gameboard)->Self{
        GameboardController{gameboard}
    }

    pub fn event<E:GenericEvent>(&mut self, event: &E){

    }
}