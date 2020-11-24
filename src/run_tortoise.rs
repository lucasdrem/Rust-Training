pub fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
  if v1 >= v2 {
    None
  } else {
    let d = (3600 * g) / (v2 - v1);

    println!("{:?}", vec![d / 3600, d / 60 % 60, d % 60]);

    Some(vec![d / 3600, d / 60 % 60, d % 60])
  }
}