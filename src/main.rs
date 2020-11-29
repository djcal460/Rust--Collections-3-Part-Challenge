use std::collections::HashMap;
use std::io;
use std::io::Write;
use permutation::permutation;


fn main() {
    
    /*
    Given a list of integers, use a vector and return the mean 
    (the average value), median (when sorted, the value in the 
    middle position), and mode (the value that occurs most often; 
    a hash map will be helpful here) of the list.
    */
    
    //mean
    let mut v = vec![12,78,424,2,56,8,34,3,99,12,99,99];
    let mean = mean(&v);
    println!("\nThe average is: {}",mean);

    //median
    let median = median(&mut v);
    println!("Median is: {}", median);


    //mode
    let mode = mode(&v);
    match mode {
        None => println!("No mode. All items are unique"),
        Some(x) => println!("Mode is {}",x),
    }
    println!();

    /*
    Convert strings to pig latin. The first consonant of each word 
    is moved to the end of the word and “ay” is added, so “first” 
    becomes “irst-fay.” Words that start with a vowel have “hay” 
    added to the end instead (“apple” becomes “apple-hay”). Keep in
    mind the details about UTF-8 encoding!
    */

    let s1 = String::from("台灣 加拿大經貿對話視訊舉行");
    let s2 = String::from("Not all bad boogers go to heaven");
    let s3 = String::from("Здравствуйте");
    let s4 = String::from("Pookie was a real bad dude");

    //print original, convert, and print to pl
    pig(&s1);
    pig(&s2);
    pig(&s3);
    pig(&s4);


    /*
    Using a hash map and vectors, create a text interface to allow a 
    user to add employee names to a department in a company. For example,
    “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user 
    retrieve a list of all people in a department or all people in the 
    company by department, sorted alphabetically.
    */

    println!("Welcome now to the company database. Please assign workers to departments.");
    let mut departments: Vec<String> = Vec::new();
    let mut workers: Vec<String>  = Vec::new();
    
    //enter number of workers
    let mut strnum = String::new();
    let mut num: u8;
    loop{
        println!("Number of workers you want to enter into the system: ");
        input_string(&mut strnum);
        num = match strnum.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
   
    //enter workers and departments
    loop{

        //add workers
        let mut name = String::new();
        print!("Enter a worker name: "); 
        input_string(&mut name);
        //workers need to have unique names in hashmap otherwise they will get overwritten
        if workers.contains(&name){
            println!("Sorry that name exists already, Try adding first name last initial.");
            continue;
        }
        workers.push(name); //add to workers

        let mut dept = String::new();
        print!("Enter a Department name: ");
        input_string(&mut dept);
        departments.push(dept); //add to dept
        //decrement entry-count
        num -= 1;
        if num == 0 {break}
    }// end loop

    //clone to sort later
    let workers_c = workers.clone();
    let departments_c = departments.clone();
    
    //zip workers into a hashmap
    let db: HashMap<_,_> = workers.into_iter().zip(departments.into_iter()).collect();
    print!("List All People in a Department? Yes[Y], No[Any Key]: ");
    loop{
        
        let mut yesno = String::new();
        input_string(&mut yesno);
        //if they want to list
        if yes(yesno) {
            //choose a department
            print!("Type in a department: ");
            let mut dept_choice = String::new();
            input_string(&mut dept_choice);

            for (key,value) in db.iter(){
                if value == &dept_choice{
                    print!("Worker: {} ",key);
                }
            }
            break;
        } else { //they don't want to list
            break;
        }; 
    }

    print!("List All People in Company Alphabetically? Yes[Y], No[Any Key]:");
    loop{
        let mut yesno = String::new();
        input_string(&mut yesno);
        //if they want to list
        if yes(yesno) {

            let permutation = permutation::sort(&workers_c[..]);
            let ordered_workers = permutation.apply_slice(&workers_c[..]);
            let mut ordered_departments= permutation.apply_slice(&departments_c[..]);
            for (x,y) in ordered_workers.iter().zip(ordered_departments.iter_mut()) {
                print!("Worker: {} Department: {}",x,y);
            }
            break;
             

        } else{
            break;
        }

    }

}
fn mean(mean: &Vec<i32>) -> f64{
    let mut avg = 0i32;
    let l = mean.len();
    for i in mean{
        avg += i;
    }
    //get float avg
    let avg = avg as f64 / l as f64 ;
    avg
}

fn median(median: &mut Vec<i32>) -> i32{
    let mid = median.len()/2; 
    median.sort();
    //if even
    let mut med = median[mid];
    //if odd
    if median.len() % 2 == 0{
         med = (med + &median[mid-1]) / 2; 
    }
    med
}

fn mode(mode: &Vec<i32>) -> Option<i32> {

    let mut map = HashMap::new();
    for item in mode{
        let count = map.entry(item).or_insert(0); //returns address of the item's value
        *count += 1; //modify item's value at the given address
    }
    //iter over the map, get max value by key
    let x = map.iter().max_by_key(|v| v.1).unwrap();
    //if x.0 == 1, no mode since max only occurs once
    if map.get(*x.0).unwrap() == &1{
        return None
    }
    Some(**x.0)
}

fn pig(pig: &String){
    println!("Original: {}",pig);
    let s = convert_to_pl(pig);
    println!("Pig Latin: {}\n",s);
}

fn convert_to_pl(s: &String) -> String{
    //
    let mut result = String::new();
    for word in s.split_whitespace(){

        //allocate a default ! as starting char
        let mut c = '!';

        //grab first char and break
        for ch in word.chars(){
            c = ch;
            break;
        }

        //if the char is a vowell then save the 'word + hay' to the string
        if c == 'a'|| c =='e' || c =='i' || c =='o' || c =='u'|| c =='A' || 
        c =='E' || c =='I' || c =='O' || c =='U' {
            let s = format!("{}-hay ",word);
            let s: &str = &s[..];
            result.push_str(&s);
            
        }
        else{

            //if the first char is not a vowel
            let mut first =  String::new();
            let mut remaining = String::new();
            let mut flag = false;

            //iterate through the UTF-8 chars 
            for ch in word.chars(){
                //add the first char to first
                if flag == false{
                    first = first + &format!("{}",ch); 
                }
                //add the remaining chars to remaining
                if flag == true{
                    remaining = remaining + &format!("{}",ch);
                }
                //after 1st iter, set flag to true
                flag = true;
            }

            //format the string, convert to str, push to result
            let s = format!("{}-{}ay ",remaining,first);
            let s: &str = &s[..]; //convert to &str to push it
            result.push_str(s);
        }
        
    }
    //return result
    result.to_lowercase()
}

fn input_string(mystr: &mut String){
    io::stdout().flush().unwrap();
        //read stdin 
        io::stdin()
        .read_line(mystr)
        .expect("Failed to read line");
}

fn yes(yesno: String)-> bool {
    let yesno: &str = &yesno[..]; //convert to &str
    match yesno.trim() {
        "Y" => return true,
        "y" => return true,
        _ => return false,
    };
}
