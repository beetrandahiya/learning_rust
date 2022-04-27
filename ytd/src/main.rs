extern crate rafy;
use rafy::Rafy;

fn main() {
    let content = Rafy::new("https://www.youtube.com/watch?v=4I_NYya-WWg").unwrap();
    println!("{}", content.videoid);
    println!("{}", content.title);
    println!("{}", content.author);
    println!("{}", content.likes);

    let streams = content.streams;
    for stream in streams {
       println!("{}", stream.extension);
       println!("{}", stream.url);
    }
}