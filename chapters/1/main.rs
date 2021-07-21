fn print(input_string: String) {
    println! ("{}", input_string);
  }
  
  // fn print2(input_string: str) {
  //   println! ("{}", input_string);
  // }
  
  fn main() {
    let test_string = String::from("Hello, world!");
    // let test_string2 = "Hello, world";
    print(test_string);
  
    let int_array: [i32; 3] = [1, 2, 3];
  
  for i in int_array.iter() {
    println! ("{}", i);
  }
  
  let mut str_vector: Vec<&str> = vec!["one", "two", "three"];
  str_vector.push("four");
  
  for i in str_vector.iter() {
    println! ("{}", i);
  }
  
  let second_int_array: [i32; 3] = [1,2,3];
  let _two = second_int_array[1];
  
  }