fn main() {
    // implicit lifetime
    /*
    let r;
    {
        let x = 1;
        r = &x;
    }
    println!("r is {}", r);
    */

    // explicit lifetime
    let str1 = String::from("long_str");
    {
        let str2 = String::from("short");
        let result = longer_str(&str1, &str2);

        println!("result is {}", result);
    }

    // explicit lifetime check 1
    /*
    let result;
    {
        let str2 = String::from("short");
        result = longer_str(&str1, &str2);
    }
    println!("result is {}", result);
    */

    /* different lifetime */
    let result2;
    {
        let str2 = String::from("short");
        result2 = first_str(&str1, &str2);
    }
    println!("result2 is {}", result2);

    /* struct lifetime */
    #[derive(Debug)]
    struct LifeStruct <'a> {
        x: &'a i32,
    }

    let x1 = 5;
    let mut ls = LifeStruct {x: &x1};
    /*
    {
        let x2 = 6;
        ls.x = &x2;
    }
    */
    println!{"ls is {:?}", ls};

    /* static lifetime */
    let s1 = "hello";
    let mut ret = static_str(&s1);
    println!("ret is {}", ret);
    /*
    let s = String::from("hello2");
    ret = static_str(&s);
    println!("ret is {}", ret);
    */
}

/*
fn longer_str(x:&String, y:&String) -> &String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

fn longer_str<'a>(x:&'a String, y:&'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_str<'a, 'b>(x:&'a String, _y:&'b String) -> &'a String {
    x
}

fn static_str(s:&'static str) -> &str {
    s
}
