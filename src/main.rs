use sqlite;
use sqlite::{Connection, State};
use std::time::SystemTime;

fn setup_db(connection: &Connection) {
    connection.execute("
        PRAGMA journal_mode = OFF;
        PRAGMA synchronous = 0;
        PRAGMA cache_size = 1000000;
        PRAGMA locking_mode = EXCLUSIVE;
        PRAGMA temp_store = MEMORY;
    ").expect("Pragma setup");

    connection.execute("CREATE TABLE numbers (num INTEGER);")
        .expect("Create table");
}

fn start_transaction(connection: &Connection) {
    connection.execute("BEGIN;")
        .expect("Transaction start");
}

fn commit_transaction(connection: &Connection) {
    connection.execute("COMMIT;")
        .expect("Transaction commit");
}

fn insert_data(connection: &Connection) {
    let num_rows_to_insert = 10_000_000;
    let batch_size = 100;
    let mut i = 0;

    // insert x batches
    for _ in 0..(num_rows_to_insert/batch_size) {
        // create string for batch
        let mut stmt = "INSERT INTO numbers VALUES".to_owned();
        for _ in 0..batch_size {
            stmt.push_str(format!(" ({}),", i).as_str());
            i += 1;
        }

        // strip the final ,
        stmt.pop();

        // end the statement
        stmt.push_str(";");

        // execute the statement
        connection.execute(stmt).expect("Execute batch");
    }
}

fn main() {
    let connection = sqlite::open(":memory:").unwrap();

    setup_db(&connection);
    start_transaction(&connection);

    let before_insert = SystemTime::now();
    insert_data(&connection);
    let after_insert = SystemTime::now();

    commit_transaction(&connection);

    // validate our data was inserted
    let mut statement = connection
        .prepare("SELECT COUNT(*) FROM numbers")
        .expect("Statement compile");

    let mut num_rows: i64 = 0;
    while let State::Row = statement.next().unwrap() {
        num_rows = statement.read::<i64>(0).unwrap();
    }

    println!("rows inserted: {}", num_rows);

    let ms = after_insert.duration_since(before_insert).unwrap().as_millis();
    let s: f64 = (ms as f64) / 1000.0;
    println!("Insert time: {}ms", ms);

    println!("Rows/second: {}", (num_rows as f64) / s);
}

/*
Tested on M1 Macbook Pro
Batch size : rows per second
50    : 1_290_988
100   : 1_310_272 <-- peak
250   : 1_221_597
500   : 1_182_732
1_000 : 1_162_250
10_000: 1_110_494
50_000:  867_603
 */
