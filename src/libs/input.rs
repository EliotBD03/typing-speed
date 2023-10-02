use std::io;

#[derive(Debug)]
pub struct Input{
    buffer : String
}

impl Input{
    pub fn new()->Self{
        Input{
            buffer : String::new()
        }
    }

    pub fn take(&mut self)-> io::Result<()>{
        let stdin = io::stdin();
        stdin.read_line(&mut self.buffer)?;
        self.buffer.pop(); //to remove the '\n'
        Ok(())
    }

    pub fn compare(&self, reference : &String)->bool{
        self.buffer.eq(reference)
    }

    pub fn get_buff(&self)->&String{
        &self.buffer
    }
    
    pub fn flush(&mut self){
        self.buffer = String::new();
    }
}
/*
fn main(){
    let mut inp = Input::new();

    if let Ok(_) = inp.take(){
        println!("{}", inp.compare(&String::from("salut")));
        println!("{}", inp.buffer);
    }
}
*/
