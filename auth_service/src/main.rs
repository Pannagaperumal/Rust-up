use auth_service::Credentials;
use auth_service::authenticate;

fn main() {
    let cred = Credentials{
        username:String::from("admin@gmail.com"),
        password: String::from("Pass@123")
    };
    authenticate(cred);
}
