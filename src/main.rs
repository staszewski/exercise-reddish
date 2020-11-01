use redisish::parse;

fn main() {
    let c = parse("PUBLISH TestMessage");
    println!("{:?}", c)
}
