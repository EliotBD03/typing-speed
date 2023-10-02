use std::time::{Duration, Instant};
//use std::thread::sleep;

pub struct Timer{
    now : Instant,
    elapsed : f64
}

impl Timer{

    pub fn start()->Self{
        Self{
            now : Instant::now(),
            elapsed : 0.0
        }
    }

    pub fn stop(&mut self){
        self.elapsed = self.now.elapsed().as_secs_f64();
    }

    pub fn get_time(&self)->f64{
        self.elapsed
    }

    pub fn display_time(&self){
        println!("time elapsed : {} s", self.elapsed);
    }
}
/*
fn main(){
    let mut timer = Timer::start();
    sleep(Duration::new(2,0));
    timer.stop();
    timer.display_time();
}
*/
