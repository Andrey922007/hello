use std::io;

fn main() {
    loop {
        println!("Введите номер элемента ряда Фибоначи");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Ошибка чтения n");
        let n:u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if n==1 || n==0 {
            println!("Сам посчитаешь!");
        }
        else {
            let mut i:u32 = 2;
            let mut fib1:u128 = 0;
            let mut fib2:u128 = 1;
            let mut fibn:u128 = 0;
            while i<=n {
                i+=1;
                fibn=fib1+fib2;
                fib1=fib2;
                fib2=fibn;
            }
            println!("Число фибоначи номер {} равно: {}", n, fibn);
        }    
    }        
}
