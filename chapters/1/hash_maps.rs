use std::collections::HashMap;

fn main(){
  let mut general_map: HashMap<&str, i8> = HashMap::new();
  general_map.insert("test", 25);
  let outcome: Option<&i8> = general_map.get("test");
  // the get method does not return an i8 type, despite us inserting an i8 type into the hash map. It returns an Option enum, because the get method could fail if we pass in a key that does not exist, so we have to unwrap the option to get the value
  match general_map.get("test"){
    None => println!("it failed"),
    Some(result) => println!("Here is the result: {}", result)
  }
  println! ("{}", outcome.unwrap());
}