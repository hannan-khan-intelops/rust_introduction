// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//tuple struct
struct TupColor (u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("traditional Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TupColor(255, 0, 0);
    tc.0 = 222;
    println!("tuple color: {} {} {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("John", "doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("{}", p.get_full_name());
    p.set_last_name("JacobJingleHeimerSchmidt");
    println!("{}", p.get_full_name());
    print!("As tuple: {:?}", p.to_tuple());
}