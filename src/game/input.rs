use piston_window::{ControllerButton, ControllerAxisArgs, Key};

#[derive(Default)]
pub struct InputController {
    actions: Vec<Actions>
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Actions {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub boost: bool,
    pub shoot: bool
}

impl InputController {
    pub fn new(num_players: usize) -> InputController {
        let mut actions = vec![];
        for i in 0..num_players {
            actions.push(Actions{
                rotate_left:false, 
                rotate_right:false, 
                boost:false, 
                shoot:false
            });
        }
        InputController {
            actions: actions
        }
    }

    pub fn actions(&mut self) -> Vec<Actions> {
        self.actions.clone()
    }

    pub fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    pub fn key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }

    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Left => self.actions[0].rotate_left = pressed,
            Key::Right => self.actions[0].rotate_right = pressed,
            Key::Up => self.actions[0].boost = pressed,
            Key::RShift => self.actions[0].shoot = pressed,
            Key::A => self.actions[1].rotate_left = pressed,
            Key::D => self.actions[1].rotate_right = pressed,
            Key::W => self.actions[1].boost = pressed,
            Key::LShift => self.actions[1].shoot = pressed,
            _ => ()
        }
    }

}
