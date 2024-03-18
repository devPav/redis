use std::collections::HashMap;

use redis::Commands;
fn main() {
    let val = start_redis();
    println!("{:?}", val);
}
fn start_redis() -> redis::RedisResult<HashMap<String, usize>> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    redis::cmd("SET").arg("new_key").arg(42).query(&mut con)?;
    dbg!(redis::cmd("GET").arg("new_key").query::<u8>(&mut con)?);

    let list_key = vec![String::from("Hand good"), String::from("Board dry")];
    let list_val = vec![150_u8, 25];
    con.set(vec![55u8, 56], vec![1u8, 2])?;
    dbg!(con.get::<bool, Vec<u8>>(true));

    con.set("my_key", vec![1u8, 2])?;
    con.get("my_key")?;

    con.hset("0#preflop#1|2|3|4", "hands", 15_000_usize)?;
    con.hset("0#preflop#1|2|3|4", "winrate", 15_usize)?;
    con.hgetall("0#preflop#1|2|3|4")
}
