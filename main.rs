// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run
use std::fmt;

struct A {
    abc: Option<String>
}

struct B {
    def: String
}

impl fmt::Display for B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

impl A {
    pub fn abc(&mut self, abc: String) -> &mut Self {
        self.abc = Some(abc);
        self
    }

    pub fn build(&mut self) -> B {
        B {
            def: self.abc.as_ref().unwrap().clone()
        }
    }
}

fn main() {
    // let optional = None;
    // let output = test_error(optional);
    // println!("result: {}", output.is_err());
    let mut a = A {
        abc: None
    };
    a.abc("abc".to_owned());
    println!("b {}", a.build());
}
