
// KHÔNG ĐỌC

// Đây chỉ là phần dùng thử Future, Polling vs Context, chưa liên quan chuẩn giao thức 




use futures::task::noop_waker;
use std::collections::VecDeque;
use std::future::Future;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

// Định nghĩa ClientFuture quản lý kết nối TCP của mỗi client
struct ClientFuture {
    id: usize,
    stream: TcpStream,
    buffer: Vec<u8>,
    state: usize,
}

impl Future for ClientFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            0 => {
                // Đọc dữ liệu từ client
                let mut temp_buf = [0u8; 1024];
                match self.stream.read(&mut temp_buf) {
                    Ok(size) if size > 0 => {
                        self.buffer.extend_from_slice(&temp_buf[..size]);
                        println!("Client {}: Received data: {:?}", self.id, &self.buffer);
                        self.state = 1; // Chuyển sang bước gửi phản hồi
                        Poll::Pending
                    }
                    Ok(_) => {
                        // Client đóng kết nối
                        println!("Client {}: Connection closed.", self.id);
                        Poll::Ready(())
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // Không có dữ liệu, chờ tiếp
                        Poll::Pending
                    }
                    Err(e) => {
                        println!("Client {}: Error: {:?}", self.id, e);
                        Poll::Ready(())
                    }
                }
            }
            1 => {
                // Gửi phản hồi đến client
                let response = b"Message received!";
                match self.stream.write(response) {
                    Ok(_) => {
                        println!("Client {}: Sent response.", self.id);
                        self.buffer.clear(); // Xóa dữ liệu cũ
                        self.state = 0; // Quay lại trạng thái đọc
                        Poll::Pending
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Poll::Pending,
                    Err(e) => {
                        println!("Client {}: Error while sending response: {:?}", self.id, e);
                        Poll::Ready(())
                    }
                }
            }
            _ => Poll::Ready(()),
        }
    }
}

// Định nghĩa Broker để quản lý các client
struct Broker {
    clients: VecDeque<Pin<Box<ClientFuture>>>,
    next_id: usize,
}

impl Broker {
    fn new() -> Self {
        Broker {
            clients: VecDeque::new(),
            next_id: 1,
        }
    }

    // Thêm một client mới vào broker
    fn add_client(&mut self, stream: TcpStream) {
        stream
            .set_nonblocking(true)
            .expect("Failed to set non-blocking");
        let client = ClientFuture {
            id: self.next_id,
            stream,
            buffer: Vec::new(),
            state: 0,
        };
        self.clients.push_back(Box::pin(client));
        self.next_id += 1;
    }

    // Poll tất cả các client
    fn poll_clients(&mut self) {
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut i = 0;
        while i < self.clients.len() {
            if let Some(mut client) = self.clients.pop_front() {
                match client.as_mut().poll(&mut context) {
                    Poll::Ready(response) => {
                        //println!("{}", response);
                    }
                    Poll::Pending => {
                        // Nếu chưa hoàn thành, thêm lại vào cuối hàng đợi
                        self.clients.push_back(client);
                    }
                }
            }
            i += 1;
        }
    }

    // Kiểm tra xem còn client nào không
    fn has_clients(&self) -> bool {
        !self.clients.is_empty()
    }
}

fn main() -> io::Result<()> {
    // Tạo TcpListener
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080")?;
    listener
        .set_nonblocking(true)
        .expect("Failed to set listener to non-blocking");
    let broker = Arc::new(Mutex::new(Broker::new()));

    println!("Broker started. Listening on 127.0.0.1:8080");

    // Vòng lặp chính
    loop {
        // Chấp nhận kết nối mới
        // await
        if let Ok((stream, addr)) = listener.accept() {
            println!("New connection from {:?}", addr);
            let mut broker = broker.lock().unwrap();
            broker.add_client(stream);
        }

        // Poll tất cả client hiện có
        {
            let mut broker = broker.lock().unwrap();
            broker.poll_clients();
        }

        // Tạm dừng ngắn để tránh tiêu tốn CPU
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
