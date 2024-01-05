use std::sync::{Mutex, Arc};
use std::thread;

struct GlobalCounter {
    value: Mutex<u32>,
}

pub fn iniciar(type_one:i32, type_two:i32) {
    // Creamos la variable global y la envolvemos en un Arc (Atomic Reference Count)
    // para que pueda ser compartida entre hilos de manera segura.
    let counter = Arc::new(GlobalCounter {
        value: Mutex::new(0),
    });

    // Creamos un vector para almacenar los identificadores de los hilos.
    let mut handles = vec![];

    // Creamos varios hilos tipo 1 que incrementan la variable global.
    for _ in 0..type_one {
        // Clonamos el Arc para cada hilo tipo 1.
        let counter_clone_one = Arc::clone(&counter);

        // Creamos un nuevo hilo tipo 1 que incrementa la variable global.
        let handle_type_one = thread::spawn(move || {
            let mut value = counter_clone_one.value.lock().unwrap();
            *value += 1;
            println!("hilo 1: {}", *value);
        });
        
        // Almacenamos el identificador del hilo en el vector.
        handles.push(handle_type_one);
    }
    
    for _ in 0..type_two {
        // Clonamos el Arc para cada hilo tipo 2.
        let counter_clone_two = Arc::clone(&counter);
        // Creamos un nuevo hilo tipo 1 que incrementa la variable global.
        let handle_type_two = thread::spawn(move || {
            let mut value = counter_clone_two.value.lock().unwrap();
            *value += 2;
            println!("hilo 2: {}", *value);
        });

        handles.push(handle_type_two);
    }

    // Esperamos a que todos los hilos terminen.
    for handle in handles {
        handle.join().unwrap();
    }

    // Imprimimos el valor final de la variable global.
    let final_value = counter.value.lock().unwrap();
    println!("Valor final: {}", *final_value);
}
