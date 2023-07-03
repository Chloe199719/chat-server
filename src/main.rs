use tokio::{net::{TcpListener}, io::{ AsyncWriteExt, BufReader, AsyncBufReadExt, AsyncReadExt}, sync::broadcast};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // let stream = TcpStream::connect("localhost:8080").await.unwrap();
    let (tx , _rx) = broadcast::channel(100);
   loop {
        let  (mut socket, addr) = listener.accept().await.unwrap();
      
        let tx = tx.clone();
        let mut rx =tx.subscribe();

        tokio::spawn(async move{
            let (mut reader, mut writer) = socket.split();
            writer.write_all(b"Enter your name: ").await.unwrap();
            // get name
            let mut  name = [0u8; 16];
            let bytes_read = reader.read(&mut name).await.unwrap();
            let name = String::from_utf8_lossy(&name[..bytes_read]).to_string();





            let mut reader = BufReader::new(reader);

            let mut line = String::new();
            loop {
            tokio::select! {
                result = reader.read_line(&mut line) => {
                    match result {
                        Ok(0) => break,
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                           
                        }
                    }
                    tx.send((line.clone(),addr, name.clone())).unwrap();
                    line.clear();
                }
                result = rx.recv() => {
                    let (msg ,other_addr, name) = result.unwrap();
                    let msg = format!("{}: {}",name.trim(),msg);
                    if addr != other_addr {
                    writer.write_all(&msg.as_bytes()).await.unwrap();
                    }
                }
            }
        
        }
        });
       
   }
}




// use tokio::{net::{TcpListener, TcpStream}, io::{ AsyncWriteExt, BufReader, AsyncBufReadExt, AsyncReadExt}, sync::broadcast};


// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("localhost:8080").await.unwrap();
//     let stream = TcpStream::connect("localhost:8080").await.unwrap();
//     let (tx , _rx) = broadcast::channel(100);
//    loop {
//         let  (mut socket, addr) = listener.accept().await.unwrap();
  
//         let tx = tx.clone();
//         let mut rx =tx.subscribe();

//         tokio::spawn(async move{
//             let (mut reader, mut writer) = socket.split();
//             writer.write_all(b"Enter your name: ").await.unwrap();
//             // get name
//             let mut  name = [0u8; 16];
//             let bytes_read = reader.read(&mut name).await.unwrap();
//             let name = String::from_utf8_lossy(&name[..bytes_read]).to_string();





//             let mut reader = BufReader::new(reader);

//             let mut line = String::new();
//             loop {
//             tokio::select! {
//                 result = reader.read_line(&mut line) => {
//                     match result {
//                         Ok(0) => break,
//                         Ok(_) => {}
//                         Err(e) => {
//                             eprintln!("failed to read from socket; err = {:?}", e);
                           
//                         }
//                     }
//                     tx.send((line.clone(),addr, name.clone())).unwrap();
//                     line.clear();
//                 }
//                 result = rx.recv() => {
//                     let (msg ,other_addr, name) = result.unwrap();
//                     let msg = format!("{}: {}",name.trim(),msg);
//                     if addr != other_addr {
//                     writer.write_all(&msg.as_bytes()).await.unwrap();
//                     }
//                 }
//             }
        
//         }
//         });
       
//    }
// }
