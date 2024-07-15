fn main() {
    let num:u8 = 5;

    println!("this is stored in {}", num);

    let mut num1:u8 = 5;
    println!("this is stored in {}", num1);

    num1=16;
    // rust is immutable
    println!("this is stored in {}", num1);


    // string literal it is also muteable if we use mut write now its in &STR and its unumutable 
    let sentence: &str = "Hi Siggmaa's";
    println!("this is a string {}", sentence);

    let mut sentence2: String = String::from("Hi Siggmaa's two");
    sentence2.push_str("Whats up guy's");
    println!("this is a string {}", sentence2);

    // Tupple
    let emp_info:(&str, u8) = ("Sarim", 24);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;
    println!("employee name is {} and his age is {}", emp_name, emp_age);

    // destructring
    let (empl_name, empl_age) = emp_info;
    println!("employee name is {} and his age is {}", empl_name, empl_age);

    print_value(5);

    let num4:u8 = 10;
    let num5:u8 = 40;
    
    let result:u8 = add(num4, num5);
    print!("The sum of num4 and num5 is {}", result);

}

fn print_value(item:u8){
    print!("item value is {} item", item);
}

fn add(item4:u8, item5:u8)->u8{
    return item4+item5;
}