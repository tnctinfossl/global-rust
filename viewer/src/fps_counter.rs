use std::time::{Instant,Duration};
use std::cell::Cell;
pub struct FPSCounter{
    last:Cell<Instant>,
}

impl FPSCounter{
    pub fn new()->FPSCounter{
        FPSCounter{
            last:Cell::new(Instant::now())
        }    
    }

    pub fn count(&self)->f32{
        let now = Instant::now();
        let diff :Duration= now-self.last.get();
        let result = 1.0/(diff.as_secs_f32());
        self.last.set(now);
        result
    }
}