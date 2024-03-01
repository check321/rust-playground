mod closure_2;
mod closure_3;

#[derive(Debug)]
struct User {
    name: String,
    score: u64,
}

fn sort_score(users: &mut Vec<User>) {
    users.sort_by_key(|u| u.score)
}

fn main() {
    let u_1 = User {
        name: "Nick".to_string(),
        score: 95,
    };

    let u_2 = User {
        name: "Alex".to_string(),
        score: 87,
    };

    let u_3 = User {
        name: "Sara".to_string(),
        score: 92,
    };


    let u_4 = User {
        name: "Sally".to_string(),
        score: 100,
    };

    let mut users = vec![u_1,u_2,u_3,u_4];
    sort_score(&mut users);
    println!("{:?}",users);
}