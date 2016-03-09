extern crate mysql;
extern crate rustc_serialize;

use rustc_serialize::json;
use std::default::Default;
use mysql as my;
use mysql::conn::Opts;

#[derive(Debug, PartialEq, Eq, RustcEncodable)]
struct IPSpace {
    id: i32,
    shop_id: i32,
    domain: Option<String>,
    count: i64,
}

fn main() {
    let opts = Opts {
        user: Some(env!("MYSQL_USER").to_string()),
        pass: Some(env!("MYSQL_PASSWORD").to_string()),
        db_name: Some(env!("MYSQL_DB").to_string()),
        ..Default::default()
    };

    let pool = my::Pool::new(opts).unwrap();

    // Let's select payments from database
    let selected_ips: Vec<IPSpace> =
    pool.prep_exec("SELECT * from ip_space LIMIT 10", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, shop_id, domain, count) = my::from_row(row);
            IPSpace {
                id: id,
                shop_id: shop_id,
                domain: domain,
                count: count,
            }
        }).collect()
    }).unwrap();

    println!("{:?}", json::encode(&selected_ips).unwrap());
}