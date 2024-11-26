// // KHÔNG ĐỌC

// // Đây chỉ là phần dùng thử Future, Polling vs Context, chưa liên quan chuẩn giao thức

// use futures::task::noop_waker;
// use std::collections::VecDeque;
// use std::future::Future;
// use std::io::{self, Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::pin::Pin;
// use std::sync::{Arc, Mutex};
// use std::task::{Context, Poll};

// // Định nghĩa ClientFuture quản lý kết nối TCP của mỗi client
// struct ClientFuture {
//     id: usize,
//     stream: TcpStream,
//     buffer: Vec<u8>,
//     state: usize,
// }

// impl Future for ClientFuture {
//     type Output = ();

//     fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
//         match self.state {
//             0 => {
//                 // Đọc dữ liệu từ client
//                 let mut temp_buf = [0u8; 1024];
//                 match self.stream.read(&mut temp_buf) {
//                     Ok(size) if size > 0 => {
//                         self.buffer.extend_from_slice(&temp_buf[..size]);
//                         println!("Client {}: Received data: {:?}", self.id, &self.buffer);
//                         self.state = 1; // Chuyển sang bước gửi phản hồi
//                         Poll::Pending
//                     }
//                     Ok(_) => {
//                         // Client đóng kết nối
//                         println!("Client {}: Connection closed.", self.id);
//                         Poll::Ready(())
//                     }
//                     Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//                         // Không có dữ liệu, chờ tiếp
//                         Poll::Pending
//                     }
//                     Err(e) => {
//                         println!("Client {}: Error: {:?}", self.id, e);
//                         Poll::Ready(())
//                     }
//                 }
//             }
//             1 => {
//                 // Gửi phản hồi đến client
//                 let response = b"Message received!";
//                 match self.stream.write(response) {
//                     Ok(_) => {
//                         println!("Client {}: Sent response.", self.id);
//                         self.buffer.clear(); // Xóa dữ liệu cũ
//                         self.state = 0; // Quay lại trạng thái đọc
//                         Poll::Pending
//                     }
//                     Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Poll::Pending,
//                     Err(e) => {
//                         println!("Client {}: Error while sending response: {:?}", self.id, e);
//                         Poll::Ready(())
//                     }
//                 }
//             }
//             _ => Poll::Ready(()),
//         }
//     }
// }

// // Định nghĩa Broker để quản lý các client
// struct Broker {
//     clients: VecDeque<Pin<Box<ClientFuture>>>,
//     next_id: usize,
// }

// impl Broker {
//     fn new() -> Self {
//         Broker {
//             clients: VecDeque::new(),
//             next_id: 1,
//         }
//     }

//     // Thêm một client mới vào broker
//     fn add_client(&mut self, stream: TcpStream) {
//         stream
//             .set_nonblocking(true)
//             .expect("Failed to set non-blocking");
//         let client = ClientFuture {
//             id: self.next_id,
//             stream,
//             buffer: Vec::new(),
//             state: 0,
//         };
//         self.clients.push_back(Box::pin(client));
//         self.next_id += 1;
//     }

//     // Poll tất cả các client
//     fn poll_clients(&mut self) {
//         let waker = noop_waker();
//         let mut context = Context::from_waker(&waker);

//         let mut i = 0;
//         while i < self.clients.len() {
//             if let Some(mut client) = self.clients.pop_front() {
//                 match client.as_mut().poll(&mut context) {
//                     Poll::Ready(response) => {
//                         //println!("{}", response);
//                     }
//                     Poll::Pending => {
//                         // Nếu chưa hoàn thành, thêm lại vào cuối hàng đợi
//                         self.clients.push_back(client);
//                     }
//                 }
//             }
//             i += 1;
//         }
//     }

//     // Kiểm tra xem còn client nào không
//     fn has_clients(&self) -> bool {
//         !self.clients.is_empty()
//     }
// }

// fn main() -> io::Result<()> {
//     // Tạo TcpListener
//     let listener: TcpListener = TcpListener::bind("0.0.0.0:1885")?;
//     listener
//         .set_nonblocking(true)
//         .expect("Failed to set listener to non-blocking");
//     let broker = Arc::new(Mutex::new(Broker::new()));

//     println!("Broker started. Listening on 127.0.0.1:8080");

//     // Vòng lặp chính
//     loop {
//         // Chấp nhận kết nối mới
//         // await
//         if let Ok((stream, addr)) = listener.accept() {
//             println!("New connection from {:?}", addr);
//             let mut broker = broker.lock().unwrap();
//             broker.add_client(stream);
//         }

//         // Poll tất cả client hiện có
//         {
//             let mut broker = broker.lock().unwrap();
//             broker.poll_clients();
//         }

//         // Tạm dừng ngắn để tránh tiêu tốn CPU
//         std::thread::sleep(std::time::Duration::from_millis(10));
//     }
// }
use futures::future::Future;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::sleep;

pub enum OrOutput7<T1, T2, T3, T4, T5, T6, T7> {
    One(T1),
    Two(T2),
    Three(T3),
    Four(T4),
    Five(T5),
    Sixth(T6),
    Seventh(T7),
}

pin_project! {
    /// Future for the [`or()`] function and the [`FutureExt::or()`] method.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Or7<F1, F2, F3, F4, F5, F6, F7> {
        #[pin]
        future1: F1,
        #[pin]
        future2: F2,
        #[pin]
        future3: F3,
        #[pin]
        future4: F4,
        #[pin]
        future5: F5,
        #[pin]
        future6: F6,
        #[pin]
        future7: F7,
    }
}

pub fn or7<T1, T2, T3, T4, T5, T6, T7, F1, F2, F3, F4, F5, F6, F7>(
    future1: F1,
    future2: F2,
    future3: F3,
    future4: F4,
    future5: F5,
    future6: F6,
    future7: F7,
) -> Or7<F1, F2, F3, F4, F5, F6, F7>
where
    F1: Future<Output = T1>,
    F2: Future<Output = T2>,
    F3: Future<Output = T3>,
    F4: Future<Output = T4>,
    F5: Future<Output = T5>,
    F6: Future<Output = T6>,
    F7: Future<Output = T7>,
{
    Or7 {
        future1,
        future2,
        future3,
        future4,
        future5,
        future6,
        future7,
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, F1, F2, F3, F4, F5, F6, F7> Future
    for Or7<F1, F2, F3, F4, F5, F6, F7>
where
    F1: Future<Output = T1>,
    F2: Future<Output = T2>,
    F3: Future<Output = T3>,
    F4: Future<Output = T4>,
    F5: Future<Output = T5>,
    F6: Future<Output = T6>,
    F7: Future<Output = T7>,
{
    type Output = OrOutput7<T1, T2, T3, T4, T5, T6, T7>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if let Poll::Ready(t) = this.future1.poll(cx) {
            return Poll::Ready(OrOutput7::One(t));
        }
        if let Poll::Ready(t) = this.future2.poll(cx) {
            return Poll::Ready(OrOutput7::Two(t));
        }
        if let Poll::Ready(t) = this.future3.poll(cx) {
            return Poll::Ready(OrOutput7::Three(t));
        }
        if let Poll::Ready(t) = this.future4.poll(cx) {
            return Poll::Ready(OrOutput7::Four(t));
        }
        if let Poll::Ready(t) = this.future5.poll(cx) {
            return Poll::Ready(OrOutput7::Five(t));
        }
        if let Poll::Ready(t) = this.future6.poll(cx) {
            return Poll::Ready(OrOutput7::Sixth(t));
        }
        if let Poll::Ready(t) = this.future7.poll(cx) {
            return Poll::Ready(OrOutput7::Seventh(t));
        }
        Poll::Pending
    }
}

async fn task_1() -> i32 {
    sleep(Duration::from_secs(3)).await;
    1
}

async fn task_2() -> i32 {
    sleep(Duration::from_secs(2)).await;
    2
}

async fn task_3() -> i32 {
    sleep(Duration::from_secs(1)).await;
    3
}

#[tokio::main]
async fn main() {
    let future1 = task_1();
    let future2 = task_2();
    let future3 = task_3();
    let future4 = task_1();
    let future5 = task_2();
    let future6 = task_3();
    let future7 = task_1();

    let combined_future = or7(
        future1, future2, future3, future4, future5, future6, future7,
    );

    match combined_future.await {
        OrOutput7::One(result) => println!("First future completed with result: {}", result),
        OrOutput7::Two(result) => println!("Second future completed with result: {}", result),
        OrOutput7::Three(result) => println!("Third future completed with result: {}", result),
        OrOutput7::Four(result) => println!("Fourth future completed with result: {}", result),
        OrOutput7::Five(result) => println!("Fifth future completed with result: {}", result),
        OrOutput7::Sixth(result) => println!("Sixth future completed with result: {}", result),
        OrOutput7::Seventh(result) => {
            println!("Seventh future completed with result: {}", result)
        }
    }
}
