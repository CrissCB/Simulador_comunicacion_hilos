use std::thread;
use std::time::Duration;

struct GlobalCounter {
    value: i32,
}

pub fn iniciar(type_one:i32, type_two:i32){
    let mut counter: GlobalCounter = GlobalCounter {value: 0,};

    let handle1 = thread::spawn(move || {
        for _ in 0..type_one{
            counter.value += 1;
            println!("hilo 1: {}", counter.value);
            thread::sleep(Duration::from_secs(2));
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..type_two{
            counter.value += 2;
            println!("hilo 2: {}", counter.value);
            thread::sleep(Duration::from_secs(3));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}