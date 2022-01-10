use std::collections::HashMap;
fn main() {
    let mut arguments = std::env::args().skip(1);
                      // args is itrorator
                      //  skip first one  
    let key = arguments.next().unwrap();
                            // unrap() crash program
    let val = arguments.next().expect("Please Enter key");
                            // expect() Nice error message

    let content = format!("{}\t{}\n",key,val);

    std::fs::write("note.db",content).unwrap();
    
    println!("Key is :{}, value is {}",key,val);

    let database = Database::create().expect("DB crashed,,,!");

}

struct Database{
    map : std::collections::HashMap<String, String>,
}

impl Database {
    fn create() -> Result<Database, std::io::Error>{

        // let content = match std::fs::read_to_string("note.db") {
        //     Ok(r) => r,
        //     Err(err) => {
        //         return Err(err);
        //     }
        // };

        let content =  std::fs::read_to_string("note.db")?;

        let mut dict = HashMap::new();
        for line in content.lines(){
                    // iterator
            let mut collect = line.splitn(2,'\t');
            let key = collect.next().expect("No key found");
            let val = collect.next().expect("No value found");
            dict.insert(key.to_owned(), val.to_owned());
        }

        Ok(
            Database{map: dict}
        )
    }
}