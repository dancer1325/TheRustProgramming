struct Person {
    name: String,
}
impl Person {
    // lifetime elision
    // fn get_ref_name(&self) -> &String{              //self      It's the struct in this case. This function compiles without specifying the lifetime by annotations
    fn get_ref_name<'a>(&'a self) -> &'a String{                 // Rust infers that the code is in fact
        // <'a>         Function's lifetime
        // &'a self     &self's lifetime, which it's function's input
        // &'a String   &String's lifetime, which it's function's return
        &self.name
    }
}
// struct Company{
struct Company <'z> {                // 'z      z indicates the company's lifetime
    name: String,
    // ceo: &Person,                    // Reference to the person
    ceo: &'z Person,                // Indicate that ceo's lifetime
}

pub fn demo() {
    // &'static str;               // static is a lifetime TODO: Check what's wrong in this line

    let boss = Person { name: String::from("Alfredo Toledano") };
    let guaperasCompany = Company { name: String::from("GuaperasCompany"), ceo: &boss };         // error[E0106]     Rust is against possible invalid references --> We need to specify the lifetime

    let mut z: &String;
    {
        let person = Person { name: String::from("Alfredo Toledano") };
        z = person.get_ref_name();
    }
}