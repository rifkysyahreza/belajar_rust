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

    // destructuring tuple

    // index ke 3 tidak terpakai
    let (x, y, z, _) = a;
    
    let (x, y, z, w) = a;
    println!("the x is {}", x);
    println!("the y is {}", y);
    println!("the z is {}", z);
    println!("the w is {}", w);

    println!("the x is {}", a.0);
    println!("the y is {}", a.1);
    println!("the z is {}", a.2);
    println!("the w is {}", a.3);

    // mutable tuple
    let mut b = (1, 2.5, 'a', true);
    b.0 = 10;
    println!("the tuple is {:?}", b);
}

// Unit Type
/*
    Unit type adalah tipe data yang tidak memiliki nilai
    Unit type di rust menggunakan tanda kurung
    Unit type biasanya digunakan untuk fungsi yang tidak mengembalikan nilai
*/
fn unit() {
    println!("Hello, world!");
}

#[test]
fn unit_type_test(){
    let a = ();
    println!("the unit type is {:?}", a);

    let result = unit();
    println!("the unit type is {:?}", result);
}

// Array
/*
    Array adalah kumpulan nilai dengan tipe data yang sama
    Array di rust menggunakan tanda kurung siku
    Array di rust memiliki jumlah data yang final
    Array di rust bisa diakses menggunakan index
*/
#[test]
fn array_test(){
    let a = [1, 2, 3, 4, 5];
    println!("the array is {:?}", a);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the array is {:?}", b);

    let c = [0; 5];
    println!("the array is {:?}", c);

    let d = [1, 2, 3, 4, 5];
    println!("the array is {:?}", d[0]);
    println!("the array is {:?}", d[1]);
    println!("the array is {:?}", d[2]);
    println!("the array is {:?}", d[3]);
    println!("the array is {:?}", d[4]);

    // mutable array
    let mut e = [1, 2, 3, 4, 5];
    e[0] = 10;
    println!("the array is {:?}", e);

    let length = e.len();
    println!("the array length is {:?}", length);
}

// Two-Dimensional Array
#[test]
fn two_dimensional_array_test(){
    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    println!("the matrix is {:?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[0][2]);

    let length = matrix.len();
    println!("the matrix length is {}", length);

    let length_inner_array = matrix[0].len();
    println!("the matrix length is {}", length_inner_array); 
}

// Constants
/*
    Rust mendukung konstanta
    Konstanta di rust menggunakan keyword const
    Konstanta di rust menggunakan huruf besar
    Konstanta di rust harus mendeklarasikan tipedata
    Konstanta di rust harus mendeklarasikan nilai
    Konstanta di rust tidak bisa menggunakan mutable
*/
#[test]
fn constant_test(){
    const MAX_POINTS: u32 = 100_000;
    println!("the constant is {}", MAX_POINTS);
}

// Variable Scope
/*
    Rust mendukung variable scope
    Variable scope adalah ruang lingkup variable
    Variable scope di rust menggunakan kurung kurawal
    Variable scope di rust bisa menggunakan variable yang sama
    Variable scope di rust bisa menggunakan variable yang berbeda
*/
#[test]
fn variable_scope_test(){
    let a = 10; // variable a di scope ini

    { // inner scope
        let b = 20;
        println!("the b is {}", b);

        println!("the b is {}", a); // variable a di scope ini
    }

    println!("the a is {}", a);
}

//============= Memory Management =============

// Garbage Collection
/*
    Rust tidak menggunakan garbage collection
    Rust menggunakan sistem ownership
    Rust menggunakan sistem ownership untuk mengelola memory
    Rust menggunakan sistem ownership untuk menghindari memory leak
    Rust menggunakan sistem ownership untuk menghindari double free
*/

// Stack and Heap
/*
    Rust menggunakan stack dan heap untuk mengelola memory
    Stack adalah memory yang cepat dan kecil
    Stack digunakan untuk data yang ukurannya sudah diketahui
    Stack digunakan untuk data yang ukurannya kecil
    Stack digunakan untuk data yang tidak bisa dihapus
    Stack digunakan untuk data yang tidak bisa diubah
    Stack digunakan untuk data yang tidak bisa diakses secara acak
    Stack digunakan untuk data yang bisa diakses secara cepat
    Stack digunakan untuk data yang bisa diakses secara berurutan
    Stack digunakan untuk data yang bisa diakses secara sekuensial
    Stack digunakan untuk data yang bisa diakses secara linear
    Stack digunakan untuk data yang bisa diakses secara langsung
    Stack digunakan untuk data yang bisa diakses secara sejajar
    Stack digunakan untuk data yang bisa diakses secara berurutan
*/
/*
    Heap adalah memory yang lambat dan besar
    Heap digunakan untuk data yang ukurannya belum diketahui
    Heap digunakan untuk data yang ukurannya besar
    Heap digunakan untuk data yang bisa dihapus
    Heap digunakan untuk data yang bisa diubah
    Heap digunakan untuk data yang bisa diakses secara acak
    Heap digunakan untuk data yang bisa diakses secara lambat
    Heap digunakan untuk data yang bisa diakses secara tidak berurutan
    Heap digunakan untuk data yang bisa diakses secara tidak sekuensial
    Heap digunakan untuk data yang bisa diakses secara tidak linear
    Heap digunakan untuk data yang bisa diakses secara tidak langsung
    Heap digunakan untuk data yang bisa diakses secara tidak sejajar
    Heap digunakan untuk data yang bisa diakses secara tidak berurutan
    Anggap saja stack adalah tumpukan buku, sedangkan heap adalah rak buku
*/
#[test]
fn stack_and_heap() {
    function_a();
    function_b();
}

fn function_a(){
    let a = 10; // stack
    let b = String::from("Hello"); // heap

    println!("{} {}", a, b);
}

fn function_b() {
    let a = 20; // stack
    let b = String::from("World"); // heap

    println!("{} {}", a, b);
}

// Drop Function
/*
    Saat variable keluar dari scope nya, yang artinya variable sudah tidak digunakan lagi
    Rust akan memanggil drop function
    Drop function adalah fungsi yang akan menghapus data di heap
    Dan jika rust function() sudah selesai dieksekusi, maka function()
    tersebut akan dihapus pula dari stack frame
*/

// String Type
/*
    Rust memiliki tipe data text yang fixed size, yaitu &str (string slice)
    dan yang bisa mengembang ukurannya, yaitu String
    &str karena ukurannya fixed size, jadi Rust akan menyimpannya di stack,
    sedangkan String karena ukurannya bisa mengembang, Rust akan menyimpannya di heap
*/

// Immutable str
/*
    Karena ukuran &str fixed size, maka &str tidak bisa diubah/immutable
    ketika kita buat variable mutable, dan mengubah data &str, sebenarnya yang
    dilakukan adalah mengganti isi variable, bukan mengubah isi dari &str nya
    &str memiliki banyak method yang bisa digunakan, dan beberapa method
    tersebut mengembalikan &str baru atau ada yang String
*/