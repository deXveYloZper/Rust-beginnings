fn strings () {
let string = String::from("Hello World");
let slice = &string;
let slice = "hello";
let slice2 = &string[0..5];

println!(slice2, slice);
}