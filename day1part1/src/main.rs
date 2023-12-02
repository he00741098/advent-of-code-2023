use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("text.txt")?;
    let mut reader = BufReader::new(f);
    let mut sum = 0;
    let numbers = ["0", "1", "2","3","4","5","6","7","8","9"];
    let valid_words = ["one", "two","six", "four", "five","nine","three","seven","eight"];
    let mut line = String::new();
    let mut first_valid_number:Option<(usize, String)> = None;
    let mut last_valid_number:Option<(usize, String)> = None;
    let mut all_valid_words:Vec<(usize, String)> = vec![];
    while let Ok(_) = reader.read_line(&mut line){
        // let mut current_number = String::new();
        println!("line: {:?}", line);
        if line.len()==0{
            line=String::new();
            break;
        }
        let mut first_index = 0;
        //scan line for first digit
        for i in line.chars(){
            if numbers.contains(&&*i.to_string()){
                // current_number = i.to_string();
                first_valid_number = Some((first_index, i.to_string()));
                // println!("Current_first_number: {}", i);
                break;
            }
            first_index+=1;
        }
        let mut last_index = line.len()-1;
        for i in line.chars().rev(){
            if numbers.contains(&&*i.to_string()){
                // sum+=i.to_string().parse::<i32>().expect("char to parse");
                // current_number = format!("{}{}", current_number, i.to_string());
                last_valid_number = Some((last_index, i.to_string()));
                // println!("Current_second_number: {}", i);
                break;
            }
            last_index-=1;
        }

        let windower = line.chars().map(|x|x.to_string()).collect::<Vec<String>>();
        // let valid_words_3 = ["one", "two","six"];
        let mut word_index = 0;
        for i in windower.windows(3){
            let word = format!("{}{}{}", i[0], i[1], i[2]);
            if valid_words[0..=2].contains(&&*word){
                all_valid_words.push((word_index, word));
                break;
            }
            word_index+=1;
        }
        let mut word_index = 0;
        for i in windower.windows(4){
            let word = format!("{}{}{}{}", i[0], i[1], i[2], i[3]);
            if valid_words[3..=5].contains(&&*word){
                all_valid_words.push((word_index, word));
                break;
            }
            word_index+=1;
        }
        let mut word_index = 0;
        for i in windower.windows(5){
            let word = format!("{}{}{}{}{}", i[0], i[1], i[2], i[3],i[4]);
            if valid_words[6..].contains(&&*word){
                all_valid_words.push((word_index, word));
                break;
            }
            word_index+=1;
        }
        //check the first word
        // let first_number_data = first_valid_number.unwrap();
        // let last_number_data = last_valid_number.unwrap();
        // let first_word_data = None;
        let mut largest_index = -1;
        let mut largest_index_word = String::new();
        let mut lowest_index = None;
        let mut lowest_index_word = String::new();
        for i in all_valid_words{
            if i.0 as i32>largest_index{
                largest_index = i.0 as i32;
                largest_index_word = i.1.clone();
            }
            if lowest_index.is_none(){
                lowest_index = Some(i.0);
                lowest_index_word = i.1;
            }else if let Some(x) = lowest_index{
                if x>i.0{
                    lowest_index = Some(i.0);
                    lowest_index_word = i.1;
                }else{
                    lowest_index = Some(x);
                    lowest_index_word = i.1;
                }
            }
        }
        let mut lowest_absolute_number ;
        let lowest_is_number = false;
        if let Some(lowest_number_data) = first_valid_number{
            if let Some(lowest_index) = lowest_index{
                if lowest_index>lowest_number_data.0{
                   lowest_absolute_number = lowest_number_data.1; 
                }else{
                    lowest_absolute_number = lowest_index_word;
                }
            }else{
                lowest_absolute_number = lowest_index_word;
            }
        }else{
            lowest_absolute_number = lowest_index_word;
        }
        let mut largest_absolute_number;
        if let Some(largest_number_data) = last_valid_number{
            if largest_index<largest_number_data.0 as i32{
                largest_absolute_number = largest_number_data.1;
            }else{
                largest_absolute_number = largest_index_word;
            }
        }else{
            largest_absolute_number = largest_index_word;
        }
        let final_word;
        if numbers.contains(&&*lowest_absolute_number){
            final_word = lowest_absolute_number;
        }else {
            final_word = word_to_number(lowest_absolute_number);
        }
        let final_word_last;
        if numbers.contains(&&*largest_absolute_number){
            final_word_last = largest_absolute_number;
        }else{
            final_word_last = word_to_number(largest_absolute_number);
        }
        let final_number = format!("{}{}", final_word, final_word_last).parse::<u64>().unwrap();
        println!("{}", final_number);
        sum+=final_number;

        first_valid_number=None;
        last_valid_number=None;
        all_valid_words=vec![];
        line=String::new();
    }


    println!("Sum: {}", sum);

    Ok(())
}

fn word_to_number(lowest_absolute_number:String)->String{
    let mut final_word = String::new();
    if lowest_absolute_number=="one"{
        final_word = "1".to_string();
    }else if lowest_absolute_number=="two"{
        final_word = "2".to_string();
    }else if lowest_absolute_number=="three"{
        final_word="3".to_string();
    }else if lowest_absolute_number=="four"{
        final_word="4".to_string();
    }else if lowest_absolute_number=="five"{
        final_word="5".to_string();
    }else if lowest_absolute_number=="six"{
        final_word="6".to_string();
    }else if lowest_absolute_number=="seven"{
        final_word="7".to_string();
    }else if lowest_absolute_number=="eight"{
        final_word="8".to_string();
    }else if lowest_absolute_number=="nine"{
        final_word = "9".to_string();
    }
    final_word
}