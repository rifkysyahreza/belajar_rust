fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("hello test");
}

#[test]
fn variable_test() {
    let x = 5;
    println!("the x is {}", x);
}

// mutable variable
/*  pada dasarnya variable di rust adalah immutable, 
jika ingin membuat variable mutable, bisa menggunakan keyword mut
*/
#[test]
fn mutable_test() {
    let mut x = 5;
    println!("the x is {}", x);
    x = 6;
    println!("the x is {}", x);
}

#[test]
fn static_typing_test() {
    let name = "John";
    println!("the name is {}", name);

    // name = 10; //error
}

#[test]
fn shadowing_test(){
    let name = "Rifky"; // nilai awal
    println!("the name is {}", name);

    let name = "John"; // nilai baru yang menutupi nilai awal
    println!("the name is {}", name);
}

/*
this is multiline comment
this is multiline comment
*/

// this is single line comment

// Scalar Types
/*
    Integer: bilangan bulat
    Floating-Point: bilangan desimal
    Booleans: true/false
    Characters: karakter
*/
// Compound Types
/*
    Tuples: kumpulan nilai dengan berbagai tipe data
    Arrays: kumpulan nilai dengan tipe data yang sama
*/

// rust tidak perlu mendeklarasikan tipedata, karena rust sudah bisa menebak tipedata dari nilai yang diberikan
// jika ingin mendeklarasikan tipedata, bisa menggunakan sintaks seperti ini
// let x: i32 = 5;

// Integer
// i8, i16, i32 (default), i64, i128, isize (signed)(bisa negatif) angka = bit nya kecuali isize (mengikuti OS nya)
// u8, u16, u32, u64, u128, usize (unsigned)(tidak bisa negatif) angka = bit nya kecuali usize (mengikuti OS nya)

// Floating-Point
// f32, f64 (default) (angka desimal) angka = bit nya

#[test]
fn number() {
    let a: i8 = 10;
    println!("the number is {}", a);

    let b: f64 = 10.5;
    println!("the number is {}", b);
}

// Konversi Tipe Data
/*
    Rust bisa konversi tipe data dari tipe data Number
    yang ukurannya lebih kecil ke tipe data Number yang ukurannya lebih besar

    Perlu diperhatikan jika melakukan konversi dari besar ke 
    kecil, maka akan terjadi overflow jika nilai yang dikonversi
    lebih besar dari nilai maksimal tipe data yang dituju

    Untuk melakukan konversi, bisa menggunakan sintaks as
*/
#[test]
fn number_conversion(){
    let a: i32 = 10;
    let b: i64 = a as i64;
    println!("the number is {}", b);

    let c: i64 = 1000;
    let d: i32 = c as i32;
    println!("the number is {}", d);

    // let e: i64 = 1000000000; //overflow
    // let f: i8 = e as i8;
    // println!("the number is {}", f);
}

// Augmented Assignment
/*
    Rust mendukung augmented assignment seperti di bahasa pemrograman lain
    seperti +=, -=, *=, /=, %=
*/
#[test]
fn augmented_assignment(){
    let mut a = 10;
    a += 5;
    println!("the number is {}", a);

    let mut b = 10;
    b -= 5;
    println!("the number is {}", b);
}

// Boolean
#[test]
fn boolean_test(){
    let a = true;
    let b: bool = false;
    println!("the boolean is {}", a);
    println!("the boolean is {}", b);
}

// Comparison Operators
/*
    Rust mendukung operator perbandingan seperti di bahasa pemrograman lain
    seperti ==, !=, >, <, >=, <=
*/
#[test]
fn comparison_operator(){
    let a = 10;
    let b = 5;
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a > b: {}", a > b);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a <= b: {}", a <= b);
}

// Logical Operators
/*
    Rust mendukung operator logika seperti di bahasa pemrograman lain
    seperti &&, ||, !
*/
#[test]
fn logical_operator(){
    let a = true;
    let b = false;
    println!("a && b: {}", a && b);
    println!("a || b: {}", a || b);
    println!("!a: {}", !a);
}

// Character
/*
    Rust mendukung tipe data karakter
    karakter di rust menggunakan tanda petik satu
    karakter di rust menggunakan tipe data char
*/
#[test]
fn character_test(){
    let a = 'a';
    let b: char = 'b';
    println!("the character is {}", a);
    println!("the character is {}", b);
}

// Tipe data compound

// Tuple
/*
    Tuple adalah kumpulan nilai dengan berbagai tipe data
    Tuple di rust menggunakan tanda kurung
    Tuple di rust bisa memiliki berbagai tipe data
    Tuple di rust bisa memiliki nilai yang sama
    Jumlah data di Tuple sudah final
    Tuple di rust bisa diakses menggunakan index
*/
#[test]
fn tuple_test(){
    let a = (1, 2.5, 'a', true);
    println!("the tuple is {:?}", a);

    let data_explisit: (i32, f64, bool) = (10, 10.5, true);
    println!("the tuple is {:?}", data_explisit);

    let (x, y, z, w) = a;
    println!("the x is {}", x);
    println!("the y is {}", y);
    println!("the z is {}", z);
    println!("the w is {}", w);

    println!("the x is {}", a.0);
    println!("the y is {}", a.1);
    println!("the z is {}", a.2);
    println!("the w is {}", a.3);
}