use piston_window::{ControllerButton, ControllerAxisArgs, Key};

#[derive(Default)]
pub struct InputController {
    actions: Vec<Actions>,
    pub draw_help: bool
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Actions {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub boost: bool,
    pub shoot: bool,
    pub respawn: bool
}

impl InputController {
    pub fn new(num_players: usize) -> InputController {
        let mut actions = vec![];
        for i in 0..num_players {
            actions.push(Actions{
                rotate_left:false, 
                rotate_right:false, 
                boost:false, 
                shoot:false,
                respawn: false
            });
        }
        InputController {
            actions: actions,
            draw_help: false
        }
    }

    pub fn actions(&mut self) -> Vec<Actions> {
        self.actions.clone()
    }

    pub fn reset(&mut self) {
        for actions in self.actions.iter_mut() {
            actions.respawn = false;
        }
    }

    pub fn key_press(&mut self, key: Key) {
        match key {
            Key::F1 => self.draw_help = !self.draw_help,
            Key::R => self.actions[1].respawn = true,
            Key::Backspace => self.actions[0].respawn = true,
            _ => self.handle_key(key, true)
        }
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
