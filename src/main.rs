// fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
//     f(value)
// }

// fn square(value: i32) -> i32 {
//     value * value
// }

// fn cube(value: i32) -> i32 {
//     value * value * value
// }

// fn main() {
//     println!("apply square: {}", apply(2, square));
//     println!("apply cube: {}", apply(2, cube));
// } 

// fn pi() -> f64 {
//     3.1415926
// }

// fn not_pi() {
//     3.1415926;
// }

// fn main() {
//     let is_pi = pi();
//     let is_unit = not_pi();
//     let is_unit2 = {
//         pi();
//     };

//     println!("is_pi: {:?}, is_unit: {:?}, is_unit2: {:?}", is_pi, is_unit, is_unit2); // is_pi: 3.1415926, is_unit: (), is_unit2: ()
// }

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    let alice = User {id: UserId(1), name: "Alice".into(), gender: Gender::Female};
    let bob = User {id: UserId(2), name: "Bob".into(), gender: Gender::Male};

    let topic = Topic {id: TopicId(1), name: "rust".into(), owner: UserId(1)};

    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello, world!".into()));

    println!("event1: {:?}", event1);
    println!("event2: {:?}", event2);
    println!("event3: {:?}", event3);

    // event1: Join((UserId(1), TopicId(1)))
    // event2: Join((UserId(2), TopicId(1)))
    // event3: Message((UserId(1), TopicId(1), "Hello, world!"))
}