/* Defining a structure */
/* Its like a tuple to group different types of data together along with the benefit of naming each piece of data in it */
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/* Adding Debug trait here allows the compiler
to provide a basic implementation of Debug trait */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/* Implementation blocks will have the functions & methods associated with struct */
impl Rectangle {
    /* First argument in the method is always self which represent the current accessing struct instance.
       Here in general we take a reference. But we can also take mutable reference or ownership only if necessary.
    */

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold_other_rectangle(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/* We can have multiple implementation blocks for same struct */
impl Rectangle {
    /* We can define this along with methods as well */
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /* Instantiating Struct */
    /* A mutable struct */
    let mut user1 = User {
        email: String::from("ravi@gmail.com"),
        username: String::from("ravi123"),
        sign_in_count: 1,
        active: true,
    };

    /* Acessing struct data */
    let _name = user1.username;

    /* We can also modify the struct value if the instantiated variable is mutable */
    user1.username = String::from("ravi@outlook.com");

    let user2 = build_user(String::from("dhoni@email.com"), String::from("Dhoni"));

    /* We can create another instance using existance instances */

    let user3: User = User {
        username: String::from("user 3"),
        email: String::from("user3@email.com"),
        ..user2
    };
    println!(
        "Struct User: {}, {}, {}, {}",
        user3.username, user3.email, user3.sign_in_count, user3.active
    );

    /* Tuple Structs: Struct without named fields
        These are useful when we wanted our entire tuple to have a
        name & be of different than other tuples with similar typed data in it.
        - It means Here Each struct you define is its own type, even though the fields within the struct might have the same types
    */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    /* Here we defined 2 types of tuple structs, even they have same fields
       their purpose is different. One specifies color and other specifies Point
       So that we can assign only color struct value to color type but not point type.
    */

    /* Calculating area */
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 30,
    };
    /* By default primitive types implement display traits but struct will not.
    So we need to implement ourself to print them which can be done with
    format specifier ":?" while printing. But still for custom type
    struct we need to implemt Debug trait as well for each struct type */
    println!("rect: {:#?}", rect);
    println!(
        "Function which takes Rectangle Ref: The area of Rectangle is {} square pixels",
        area(&rect)
    );

    /* Instead defining function seperately to calculate the area of Rectangle
       We can define it within the struct as a method and use it
       which can be more meaningful
    */
    println!(
        "Struct Instance Method defined in struct: The area of Rectangle is {} square pixels",
        rect.area()
    );

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    println!(
        "rect can hold rect2? {}",
        rect.can_hold_other_rectangle(&rect2)
    );
    println!(
        "rect2 can hold rect? {}",
        rect2.can_hold_other_rectangle(&rect)
    );


    let _rect3 = Rectangle::square(20);

}

fn build_user(email: String, username: String) -> User {
    /* Returning a new instance of the user */
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
