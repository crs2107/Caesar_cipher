use std::io;
fn shift(letter: char, key:i8)-> char{
    if key == 0 || key > 25 || key < -25 {
return letter;
    }
    else{
        let mut x=letter;
        //making the character Letter as integer ch.
        let ch= letter as i8;
        match key {
            //Positive Key 1 to 25
            key @ 0..=i8::MAX => {
                match ch {
                    //for capital letter
                    65..=90 => {
                        if ch.overflowing_add(key).1
                            || ch.overflowing_add(key).0 > 90
                        {
                            x = (ch + (key - 90 + 64)) as u8 as char;
                        } else {
                            x = (ch + key) as u8 as char;
                        }
                    }
                    //for small letters
                    97..=122 => {
                        if ch.overflowing_add(key).1
                            || ch.overflowing_add(key).0 > 122
                        {
                            x = (ch + (key - 122 + 96)) as u8 as char;
                        } else {
                            x = (ch + key) as u8 as char;
                        }
                    }
                    _ => { /* return the same character that it receieved */ }
                }
            }
            // Negative key -25 to -1
            yek @ i8::MIN..=-1 => {
                let yek = -yek;
                match ch {
                     //for capital letter
                    65..=90 => {
                        if ch - yek < 65 {
                            let diff = 65 - (ch - yek);
                            x = ((90 - diff) + 1) as u8 as char;
                        } else {
                            x = (ch - yek) as u8 as char;
                        }
                    }
                    //for small letters
                    97..=122 => {
                        if ch - yek < 97 {
                            let diff = 97 - (ch- yek);
                            x = ((122 - diff) + 1) as u8 as char;
                        } else {
                            x = (ch - yek) as u8 as char;
                        }
                    }
                    _ => { /* return the same character that it receieved */ }
                }
            }
        }
        x
    }
    }
fn main(){
println!("Enter the Text to to encripted");
let mut text=String::new();
io::stdin().read_line(&mut text).expect("falied to read");
println!("{}",text);
//Working properly!
/*println!("Enter the key(An integer in between 0 to 25)");
let mut k=String::new();
io::stdin().read_line(&mut k).expect("falied to read");
let key:i8=text.trim().parse().expect("Error");
println!("{}",key);
let mut encript=vec![];
let mut i=0;
for c in text.chars() { 
    encript[i]=shift(c, 5);
    i+=1;
}
let s: String = encript.into_iter().collect();
println!("Your encripted text will look like this");
println!("{}", s);*/
//test value
let s='a';
let key=1;
println!("{}",shift(s, key));
}
