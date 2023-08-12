use std::sync::{Arc, Mutex}; // multiple producer, single consumer

//Threads
/* fn main() {

  let name = String::from("Felisteu");
  // let name_copy = name.clone();
  println!("Hello {name}!");

  let (tx, rx) = mpsc::channel(); // transmissor e receptor
  let tx2 = tx.clone(); // clonando o transmissor

  //gerenciador de threads
  // let join_handle = thread::spawn(move || {
  thread::spawn(move || {
    println!("{name} se juntou a partida !");
    // enviar mensagem para a thread principal
    for count in 0..3 {
      let mensagem = String::from(format!(
        "Ola aqui de dentro da thread {}",
        char::from_digit(count, 10).unwrap()
      ));
      tx.send(mensagem).unwrap();
      thread::sleep(Duration::from_secs(2));
    }
  });
  thread::spawn(move || {
    // enviar mensagem para a thread principal
    for count in 0..3 {
      let mensagem = String::from(format!(
        "Ola aqui de dentro da segunda thread {}",
        char::from_digit(count, 10).unwrap()
      ));
      tx2.send(mensagem).unwrap();
      thread::sleep(Duration::from_secs(2));
    }
  });

  // let mensagem_recebida = rx.recv().unwrap();
  // println!("Mensagem recebida: {}", mensagem_recebida);
  for mensagem_recebida in rx {
    println!("Mensagem recebida: {}", mensagem_recebida);
  }

  //espera a thread terminar
  // join_handle.join().unwrap();

  // println!("Goodbye {name_copy}!");

  println!("All completed");
}
 */

//Mutex
fn main() {
  // Mutex("hola")
  // lock = bloqueador
  // lock("hola")}
  // liberar lock
  // thread safe
  let id = Arc::new(Mutex::new(99));
  let mut handles = vec![];
  for _ in 0..3 {
    let num_clone = Arc::clone(&id);
    let handle = std::thread::spawn(move || {
      let mut num = num_clone.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }
  for handle in handles {
    handle.join().unwrap();
  }

  println!("id: {:?}", id);
}
