fn main() {
  scalar_type();
  compound_type();
  string_type();

}

fn scalar_type() {
  let _a = 100;
  let b = 3.14;
  let c: u32 = 12_345;
  let d: f32 = 3.14;
  let e: bool = true;
  let mut f = 100;
  let g: char = 'A';
  const PI: f64 = 3.141592;

  // shadowing
  let a = "Hello";
  f = f + 1;

  println!("{}, world!", a);
  println!("{} {} {} {} {} {} {}", b, c, d, e ,f ,g, PI);
}

fn compound_type() {
  let arr: [i32; 3] = [1, 2, 3];
  let tuple: (i32, char, bool) = (1, 'A', true);
  // Destructuring
  let (a, b, c) = tuple;

  println!("{:?}", arr);
  println!("{}", tuple.1);
  println!("{} {} {}", a, b, c);
}

fn string_type() {
  let s: &'static str = "hello";
  let x: String = s.to_owned();
  // let x = String::from("hello"); // 동일

  let sub: &str = &s[1..4];
  let mut s1 = String::new();
  s1.push('H');
  s1.push_str(" Ellow");

  println!("{}", x);
  println!("{}", sub);
  println!("{}", s1);

  // 문자열 함수에는 contains, replace, split_whitespace 등등 다양하게 있다.
}