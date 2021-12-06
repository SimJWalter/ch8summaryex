pub(crate) mod data_capture {
    use std::collections::HashMap;
    use std::io::{self, Write};

    #[derive(Debug)]
    pub struct DataAggregator {
        depts: HashMap<i32, String>,
        employees: Vec<Employee>,
    }

    impl DataAggregator {
        pub fn new() -> Self {
            Self {
                depts: HashMap::new(),
                employees: Vec::new(),
            }
        }

        fn print_head() {
            println!("d: add department");
            println!("e: add employee");
            println!("p: dump data stdout");
            println!("q: break loop");
        }

        fn input_dept(&mut self) {
            let mut newdept = format!("");
            while newdept.len() == 0 {
                print!("add department (name): ");
                io::stdout().flush().expect("");
                io::stdin()
                    .read_line(&mut newdept)
                    .expect("line read failed");
                newdept = newdept.trim().to_string();
                if newdept.len() == 0 {
                    println!("{}", "no good try again");
                }
            }

            self.depts.insert(
                match self.depts.keys().max() {
                    Some(i) => i + 1,
                    None => 0,
                },
                newdept.clone(),
            );
            println!("added department: {}", newdept);
        }

        fn input_employee(&mut self) {
            let mut newempl = format!("");
            while newempl.len() == 0 {
                print!("add employee (name): ");
                io::stdout().flush().expect("");
                io::stdin()
                    .read_line(&mut newempl)
                    .expect("line read failed");
                newempl = newempl.trim().to_string();
                if newempl.len() == 0 {
                    println!("{}", "no good try again");
                }
            }

            println!("departments");
            for (did, dname) in &self.depts {
                println!("{} : {}", did, dname);
            }

            let mut seldept = "".to_string();
            while seldept.len() == 0 {
                print!("specify department id: ");
                io::stdout().flush().expect("");
                io::stdin()
                    .read_line(&mut seldept)
                    .expect("line read failed");
                seldept = seldept.trim().to_string();
                let i_seldept = match seldept.trim().to_string().parse::<i32>() {
                    Ok(i) => i,
                    Err(_) => {
                        seldept = "".to_string();
                        0
                    }
                };

                if seldept.len() == 0 {
                    println!("{}", "no good try again");
                } else if !self.depts.keys().any(|&k| k == i_seldept) {
                    println!("{}", "no department by that id here");
                } else {
                    self.employees.push(Employee {
                        name: (&newempl).to_string(),
                        dept: i_seldept,
                    });
                    println!(
                        "Employee [{}] added to Department [{}]",
                        newempl,
                        self.depts.get(&i_seldept).unwrap()
                    );
                }
            }
        }

        fn dump_this(&self) {
            dbg!(self);
        }

        pub fn input_loop(&mut self) {
            loop {
                Self::print_head();
                print!("action: ");
                io::stdout().flush().expect("");
                let mut act = String::new();
                io::stdin().read_line(&mut act).expect("line read failed");
                match act.chars().take(1).last() {
                    Some('d') => {
                        self.input_dept();
                    }
                    Some('e') => {
                        if self.depts.len() > 0 {
                            self.input_employee();
                        } else {
                            println!("no departments for employees yet");
                        }
                    }
                    Some('p') => {
                        self.dump_this();
                    }
                    Some('q') => break,
                    _ => {}
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Employee {
        name: String,
        dept: i32,
    }
}
