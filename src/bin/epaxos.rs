use std::thread;
use std::time::Duration;

enum Message {
    Request(RequestMessage),
    RequestReply(RequestReplyMessage),
    Read(ReadMessage),
    ReadReply(ReadReplyMessage),
    RequestAndRead(RequestAndReadMessage),
    RequestAndReadReply(RequestAndReadReplyMessage),
    PreAccept(PreAcceptMessage),
    PreAcceptOk(PreAcceptOkMessage),
    Accept(AcceptMessage),
    AcceptOk(AcceptOkMessage),
    Commit(CommitMessage),
}
struct RequestMessage {
    command: String,
    params: Vec
}

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
