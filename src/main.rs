use crate::libs::timer::Timer;
use crate::libs::random::Random;
use crate::libs::input::Input;

pub mod libs;

fn main(){
    
    
    let mut inp = Input::new();
    let rnd = Random::default();

    let mut timer = Timer::start();

    loop{
        
        let generated_string : &str = &rnd.generate();
        println!("{}", generated_string);
        
        if let Ok(_) = inp.take(){

            match inp.get_buff().as_str(){

                res if res == generated_string => {
                    timer.stop();
                    timer.display_time();
                    break;
                },
                "quit" => break,
                _ => println!("missed !"),
            }
            inp.flush();
        }
        
        else{
            println!("error occured when asking input...");
        }
    }
}
