use std::{array, char};

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

    ibaratnya buku yang tidak bisa diubah isinya, tapi bisa diganti bukunya

    method nya tidak ada yang mengubah nilainya langsung, tapi menghasilkan nilai baru
*/
#[test]
fn string_test() {
    let name: &str = " Rifky ";
    let trimmed_name = name.trim();

    print!("the name is {}", name);
    println!("the name is {}", trimmed_name);

    let mut username = "Rifky"; // masih ada di stack
    username = "John"; // username menunjuk "John" di stack, bukan mengubah "Rifky", "Rifky" masih ada di memori
    println!("the username is {}", username);
}

// String
/*
    String adalah tipe data yang bisa mengembang ukurannya
    String adalah tipe data yang disimpan di heap
    String memiliki banyak method yang bisa digunakan
    String bisa diubah
    Ketika dibuat dalam bentuk immutable, maka String tidak bisa berkembang
    tetapi tetap disimpan di heap

    ibaratnya buku yang bisa diubah isinya, dan bisa diganti bukunya

    method nya ada yang membuat string baru, ada yang mengubah string aslinya
*/
#[test]
fn string_type() {
    let mut name: String = String::from("Rifky");
    name.push_str("John"); // disini name Rifky sudah dihapus, dan diganti dengan RifkyJohn

    println!("the name is {}", name);

    let tomy = name.replace("Rifky", "tommy"); // method menghasilkan nilai baru
    println!("the name is {}", name);
}

// Ownership
/*
    Rust menggunakan ownership untuk melakukan data management di memory
    Ownership adalah salah satu fitur unik di rust yang mungkin jarang ada di bahasa
    pemrograman lain
    Ownership wajib dimengerti, karena akan berdampak ke hampir semua fitur di rust
    Ownership adalah fitur yang digunakan oleh rust untuk menjadikan rust
    menjadi bahasa pemrograman yang aman dalam mengelola data di memory,
    tanpa harus adanya fitur garbage collection atau manual memory management
    Karena Ownership adalah konsep yang baru untuk kebanyakan programmer,
    maka kadang kita butuh waktu untuk memahaminya
*/

// Ownership Rules
/*
    Rust memiliki 3 aturan ownership
    1. Setiap nilai di rust memiliki variable yang disebut owner
    2. Satu nilai hanya memiliki satu owner dalam satu waktu
    3. Ketika owner keluar dari scope nya, maka nilai tersebut akan dihapus
*/
#[test]
fn ownership_rules() {
    // a tidak bisa diakses disini, karena belum dideklarasikan
    let a = 10; // a adalah owner dari 10, a bisa diakses mulai dari sini

    { // b tidak bisa diakses disini, karena belum dideklarasikan
        let b = 20; // b adalah owner dari 20, b bisa diakses mulai dari sini
        println!("the b is {}", b);
    } // b sudah keluar dari scope nya, b dihapus, b tidak bisa diakses lagi
    
    println!("the a is {}", a);
} // a sudah keluar dari scope nya, a dihapus, a tidak bisa diakses lagi

// Data Copy
/*
    Sesuai aturan di ownership rules, setiap value harus dimiliki oleh satu owner
    pada satu waktu
    Ketika kita berinteraksi dengan data, maka data akan dimiliki hanya oleh satu owner
    Semua data yang bersifat fixed size (yand disimpan di Stack), ketika kita
    tambahkan ke variable berbeda (owner baru), maka hasilnya adalah data akan di
    copy, sehingga variable baru (owner baru), akan memiliki data hasil copy dari
    variable lama (owner lama)
    Oleh karena itu, tiap data akan selalu dimiliki oleh satu owner pada satu waktu
*/
#[test]
fn data_copy() {
    let a = 10; // a adalah owner dari 10
    let b = a; // b adalah owner dari 10 yang baru, 10 dari a di copy ke b
    println!("the a is {}", a);
    println!("the b is {}", b);
}

// Ownership Movement "Untuk tipe data yang disimpan di heap"
/*
    Namun data copy tidak terjadi untuk tipe data yang disimpan di heap
    Seperti aturan di ownership, dalam satu waktu value hanya dimiliki satu owner
    Maka ketika kita coba buat variable baru (owner baru) dari variable lama (owner lama),
    maka yang terjadi bukanlah copy, melainkan movement atau transfer ownership dari
    owner lama ke owner baru
    Setelah proses transfer selesai, secara otomatis owner lama akan dianggap tidak
    valid lagi digunakan
*/
#[test]
fn ownership_movement() {
    let a = String::from("Rifky"); // a adalah owner dari String Rifky
    let b = a; // b adalah owner dari String Rifky, kepemilikan Rifky dari a dipindahkan ke b
    
    // println!("the a is {}", a); // error, a sudah tidak valid lagi
    println!("the b is {}", b);
}

// Clone
/*
    Jika kita ingin membuat copy dari data yang disimpan di heap, kita bisa menggunakan
    method clone
    Method clone akan membuat copy dari data yang disimpan di heap
    Method clone akan membuat owner baru dari data yang disimpan di heap
    Method clone akan membuat data baru dari data yang disimpan di heap
*/
#[test]
fn clone_test() {
    let a = String::from("Rifky"); // a adalah owner dari String Rifky
    let b = a.clone(); // b adalah owner dari String Rifky yang baru, kepemilikan Rifky dari a di copy ke b

    println!("the a is {}", a);
    println!("the b is {}", b);
}

// If Expression
#[test]
fn conditional() {
    let a = 10;

    if a > 5 {
        println!("a is greater than 5");
    } else {
        println!("a is less than 5");
    }
}

// Else if Expression
#[test]
fn else_if_expression() {
    let a = 10;

    if a > 5 {
        println!("a is greater than 5");
    } else if a < 5 {
        println!("a is less than 5");
    } else {
        println!("a is equal to 5");
    }
}

// Let Statement
#[test]
fn let_statement() {
    let a = 10;
    // // manual assign
    // let result: &str;

    // if a > 5 {
    //     result = "a is greater than 5";
    // } else if a < 5 {
    //     result = "a is less than 5";
    // } else {
    //     result = "a is equal to 5";
    // }

    let result: &str = if a > 5 {
        "a is greater than 5"
    } else if a < 5 {
        "a is less than 5"
    } else {
        "a is equal to 5"
    };

    println!("the result is {}", result);
}

// Loop
#[test]
fn loop_expression() {
    let mut count = 0;
    
    loop {
        count += 1;
        
        if count >= 5 {
            break;
        }
        
        if count == 3 {
            continue;
        }
        
        println!("the count is {}", count);
    }
}

// Loop with return value
#[test]
fn loop_with_return_value() {
    let mut count = 0;
    
    let result = loop {
        count += 1;
        
        if count >= 5 {
            break count * 2;
        }
        
        if count == 3 {
            continue;
        }
        
        println!("the count is {}", count);
    };

    println!("this is count {}", count);
    println!("the result is {}", result);
}

// Loop Label
/*
    Kadang kita sering membuat Loop didalam loop, dan ketika ingin menghentikan
    loop paling atas dari loop yang ada di dalam, maka hal itu tidak bisa dilakukan
    Loop memiliki fitur Label, dimana kita bisa memberi nama pada loop
    Keuntungannya memberi label pada loop adalah kita bisa menghentikan loop
    yang ingin kita hentikan dengan cara menyebutkan nama labelnya
*/
#[test]
fn loop_label() {
    let mut count = 0;

    'outer: loop {
        let mut a = 1;

        loop{
            if count > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", count, a, count * a);
            a += 1;
            if a > 10 {
                break;
            }
        }
        
        count += 1;
    }
}

// While Loop
/*
    While loop adalah loop yang akan terus berjalan selama kondisi yang diberikan
    While loop adalah loop yang akan berhenti jika kondisi yang diberikan sudah tidak terpenuhi
*/
#[test]
fn while_loop() {
    let mut count = 0;

    while count <= 10 {

        if count%2 == 0 {
            println!("the count is {} genap", count);
        }
        
        count += 1;
    }
}

// For Loop
/*
    For loop adalah loop yang akan berjalan berdasarkan range yang diberikan
    For loop adalah loop yang akan berhenti jika range yang diberikan sudah selesai
*/
#[test]
fn array_loop() {
    let array = [10, 20, 30, 40, 50];

    for value in array {
        println!("the value is {}", value);
    }

    let range = 0..array.len();
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    // range exclusive
    for value in range {
        println!("the value is {} from range for loop", array[value]);
    }

    for value in 0..array.len() {
        println!("the value is {} from range for loop but simplier", array[value]);
    }

    // range inclusive
    let range_two = 0..=array.len()-1;
    println!("Start: {}", range_two.start());
    println!("End: {}", range_two.end());

    for value in range_two {
        println!("the value is {} from range for loop inclusive", array[value]);
    }
}

// Function with parameter
#[test]
fn test_goodbye() {
    say_goodbye("Rifky", "John");
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye, {} {}", first_name, last_name);
}

// Return Value
/*
    Saat membuat function, kadang kita ingin mengembalikan hasil eksekusi yang
    dilakukan di dalam function, atau kita sebut Return value
    Jika sebuah function ingin mengembalikan value, kita bisa sebutkan ketika ketika
    deklarasi function menggunakan tanda -> lalu diikuti dengan tipe data kembalian
    valuenya
    baris eksekusi terakhir di function akan dianggap sebagai kembalian value-nya
    Atau jika kita ingin mengembalikan value sebelum baris eksekusi terakhir, kita
    bisa gunakan kata kunci return, dan diikuti dengan value yang akan dikembalikan
*/
fn factorial(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_factorial() {
    let result = factorial(5);
    println!("the result is {}", result);
}

