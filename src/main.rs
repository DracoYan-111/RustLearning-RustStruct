//定义struct
struct User {
    user_name: String,
    user_age: u64,
    active: bool,
}

fn main() {
    //===================struct===================
    //实例化struct
    let a = User {
        user_name: String::from("YanLong"),
        active: true,
        user_age: 21,
    };
    //实例化可变的struct
    let mut b = User {
        user_name: String::from("YanLong"),
        active: true,
        user_age: 21,
    };
    //对可变的struct属性进行更改
    b.user_age = 100;
}

//===================将struct作为返回值===================
fn build_user(_user_age: u64, _user_name: String) -> User {
    User {
        user_age: _user_age,
        user_name: _user_name,
        active: true,
    }
}
