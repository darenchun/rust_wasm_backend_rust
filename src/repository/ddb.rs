use mysql::prelude::*;
use mysql::*;
use std::env;

/*
// ** database connection **
env : Process environment variable for 'Rust' in enum syntax
set_var :
- key : information key name
- value : value for the key
※ [warning!]
   please set your own key and value sets for database connection in your taste.
   This is a connection example for "planetScale".
   Consider making a file for conn-info and importing it
*/
pub fn activate_connection_mysql() {

    println!("activate connection started...");
    
    // 'env' entries for database connection in url format
    
    env::set_var("DATABASE_URL", "mysql://lm66bi6axucq:pscale_pw_iVa2XUgA4Ac98tE6OI9qKONiIOgKu6jefEtPIS8bSrc@cpuydv8x6tzk.ap-northeast-2.psdb.cloud/test_database_01");
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    println!("url : {:?}", &url);

    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
    let _conn = pool.get_conn().unwrap();
    println!("Successfully connected to PlanetScale!");

    /*

    // for connection testing and simple query example

    let mut conn = pool.get_conn().unwrap();

    println!("test query start");

    conn.query_iter("select * from 'your test table'")
        .unwrap()
        .for_each(|row| {
            let r: (String) = from_row(row.unwrap());
            println!("{}", r.0);
        });
    */
}
