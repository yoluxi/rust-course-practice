use std::{collections::HashMap, fmt::Display, vec};

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(1);

    let v = vec![1, 2, 3];

    // 确认使用下标 不确认使用get
    let third = v[2];
    println!("{:?}", v);
    match v.get(2) {
        Some(third) => println!("第三个元素{}", third),
        None => println!("什么都没y"),
    }

    let v = vec![1, 2, 3];
    for i in v {
        println!("{}", i);
    }
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10
    }
    println!("{:?}", v);

    show_ip();
    show_ip_struct();

    hashmap();
    // hashmap_ownership();
    hashmap_op();
}

// 存储不同类型的元素
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn show_addr(ip: IpAddr) {
    println!("{:?}", ip);
}

fn show_ip() {
    let ips = vec![
        IpAddr::V4("12.1.1.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in ips {
        show_addr(ip)
    }
}

trait TIpAddr {
    fn display(&self);
}

struct V4(String);
impl TIpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String);
impl TIpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn show_ip_struct() {
    let v: Vec<Box<dyn TIpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V4("127.0.0.1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

// -----------------  HashMap  -----------------
fn hashmap() {
    let mut my_gems = HashMap::new();
    my_gems.insert("宝石", 1);
    my_gems.insert("宝石", 2);
    my_gems.insert("宝石", 3);

    let teams_list = vec![
        ("中国队".to_string(), 1),
        ("日本队".to_string(), 2),
        ("中国队".to_string(), 3),
    ];

    // 如果key是一样的 会被过滤
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);
}

fn hashmap_ownership() {
    let name = String::from("test");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);

    std::mem::drop(name);
    // println!("{}", name);
    // println!("{}", age);
}

fn hashmap_op() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    // 查询
    let team_name = String::from("Red");
    let score: Option<&i32> = scores.get(&team_name);
    println!("{:?}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新
    let old = scores.insert("Blue".to_string(), 20);
    assert_eq!(old, Some(10));

    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    let v = scores.entry("Yellow".to_string()).or_insert(10);
    hashmap_print(&scores)
}

fn hashmap_print<T, K>(hashmap: &HashMap<T, K>)
where
    T: Display,
    K: Display,
{
    for (key, value) in hashmap {
        println!("{}: {}", key, value);
    }
}
