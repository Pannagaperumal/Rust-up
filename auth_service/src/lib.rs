#![allow(dead_code,unused_variables)]

mod database{
    enum Status{
        Connected,
        Intrupted
    }
    fn connect_to_db()-> Status{
        Status::Connected
    }

    fn get_user(){
        //get user from db
    }
}

mod auth_utils{
    fn login(cred:Credentials){
    //tries to login
    get_user();
}
    
    mod models{
        pub struct Credentials{
            pub username: String,
            pub password: String,
        }

    }
}



pub fn authenticate(cred:Credentials){
    if let Status::Connected = connect_to_db(){
        print!("Authenticating the user");
        1;
    }
}