use std::{str::FromStr, sync::Arc};
use std::env;
use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() {

    let server = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(get_index))
        .route("/gcd", web::post().to(post_gcd))
    });

    println!("Service on http://localhost:3000...");
    server
    .bind("127.0.0.1:3000")
    .expect("failed to open port at 3000")
    .run()
    .await
    .expect("error running the server");

    // let mut numbers = Vec::new();
    
    // for arg in env::args().skip(1) {
    //     numbers.push(u64::from_str(&arg).expect(
    //         "failed in parsing input"))
    // }

    // if numbers.len() == 0 {
    //     eprint!("Usage: gcd number1 number2 . . .");
    //     std::process::exit(1);
    // }

    // let mut d = numbers[0];
    // for num in &numbers[1..] {
    //     d = gcd(d, *num);
    // }
    // println!("GCD for {:?} is {}", numbers, d);
}


async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        r#"<title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

use serde::Deserialize;
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response =
        format!("The greatest common divisor of the numbers {} and {} \
                 is <b>{}</b>\n",
                form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}


fn gcd(mut m:u64, mut n:u64) -> u64 {
    assert!(m!= 0 && n != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// #[test]
// fn test_gcd() {
//     assert_eq!(gcd(50,10), 10);
//     assert_eq!(gcd(5,10), 5);
//     assert_eq!(gcd(1,10), 1);
//     assert_eq!(gcd(11,22), 11);
// }