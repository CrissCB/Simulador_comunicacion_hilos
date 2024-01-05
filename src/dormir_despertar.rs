use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn iniciar(sleep: i32){
    // Creamos un canal para la comunicación entre procesos
    let (sender, receiver) = mpsc::channel();

    // Creamos un nuevo hilo que duerme y luego envía un mensaje
    let handle = thread::spawn(move || {
        println!("Hilo: dormido");
        
        // Dormimos el hilo durante 3 segundos
        thread::sleep(Duration::from_secs(sleep as u64));

        // Enviamos un mensaje al receptor
        sender.send("Tiempo en proceso: ".to_string() + &sleep.to_string() + "s").unwrap();

        println!("Hilo: despierto");
    });

    // Esperamos a recibir el mensaje del canal
    let mensaje = receiver.recv().unwrap();
    println!("Mensaje recibido: {}", mensaje);

    // Esperamos a que el hilo termine
    handle.join().unwrap();
}