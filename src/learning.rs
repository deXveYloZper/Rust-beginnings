fn strings () {
let string = String::from("Hello World");
let slice = &string;
let slice = "hello";
let slice2 = &string[0..5];

println!(slice2, slice);
}

fn borrowing_and_ownesrship(){
    let a = vec![1,2];
    let b = a;

    println!("{a:?}");
    println!(a:?);
}

//Structs

struct Person{
    name: String,
    age: u32,
}

struct Points(i32, i32);


impl Person{
    fn new(name: String, age: u32)-> Self{
        Person {name, age}
    }
    fn say_name(&self){
        println!("Hi, my name is {}", self.name)
    }

    fn change_age(&mut self, age:u32){
        self.age = age;
    }
}

fn main(){
    let mut bob = Person::new("bob".to_string(), 30);

    bob.say_name();
    bob.change_age(28);

    let x = point.0;
    let y = point.1;
}