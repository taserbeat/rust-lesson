#[derive(Debug)]
/** ユーザー */
struct User {
    /** ユーザー名 */
    username: String,

    /** メールアドレス */
    email: String,

    /** サインイン回数 */
    sign_in_count: u64,

    /** 有効性を表すフラグ */
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) {
        println!("area: {}", self.width * self.height)
    }
}

pub fn run() {
    // User構造体
    let mut user1 = User {
        username: String::from("admin"),
        email: String::from("admin@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("super@example.com");
    println!("{:#?}", user1);

    let user2 = new_user(String::from("user2"), String::from("user2@example.com"));
    println!("{:#?}", user2);

    // Rectangle構造体
    let rect = Rectangle::new(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect);
}

fn new_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
