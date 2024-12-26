struct User{
    name:String,
    age:u10,
    active:bool
}
fn main(){
    let user=User{
        name:Aditya,
        age:30,
        active:true
    };
    print!("{} is {} yrs old",user.name,user.age);
}