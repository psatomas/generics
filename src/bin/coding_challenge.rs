#[derive(Debug)]

enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
} 

impl ChatMessage<DigitalContent> {
    fn consume_entertaiment(&self) {
        println!("watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T>  {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }    
}
fn main() {
    let message = ChatMessage {
        content: "Hi, lol",
        time: String::from("2025-07-15"),
    };
    println!("{}", message.retrieve_time());
    
    let notification = ChatMessage {
        content: "what's your favorite pizza topping?",
        time: String::from("2025-07-16"),
    };
    println!("{}", notification.retrieve_time());
    
    let audio = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("2025-07-17"),
    };
    audio.consume_entertaiment();
    println!("{}", audio.retrieve_time());
}