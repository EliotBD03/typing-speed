use rand::Rng;

#[derive(Debug)]
pub struct Random{
    
    size : usize,
    character_list : String
}

impl Default for Random{
    
    fn default()->Self{
        Random{
            size : 10,
            character_list : String::from(get_alph()) 
        }
    }
}

impl Random{
    pub fn generate(&self)->String{
        let mut rng = rand::thread_rng();
        let mut i = 0;
        let mut generated_sentence = String::from("");

        while i < self.size{
            let index = rng.gen_range(0..self.character_list.len()-1);
            generated_sentence.push(self.character_list.chars().nth(index).unwrap());
            i += 1;
        }

        generated_sentence
    }

    pub fn extend(&mut self, chars : &str){
        self.character_list += chars;
    }

    pub fn increase_size(&mut self, n : usize){
        self.size += n;
    }
}

fn get_alph()->String{
    let a = 'a' as u32;
    let z = 'z' as u32;
    let ascii_values = a..z;
    
    let mut alph = String::from("");
    
    for letter in ascii_values{
        alph.push(char::from_u32(letter).unwrap());
    }
    
    alph
}
/*
fn main()
{
    let mut r = Random::default();
    println!("{}", r.generate());
    r.increase_size(10);
    println!("{}", r.generate());
    r.extend("@'è");
    println!("{}", r.generate());
}
*/
