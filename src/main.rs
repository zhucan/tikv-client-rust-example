use clap::{App, Arg};
use tikv_client::{Config, RawClient as Client};

pub type Value = Vec<u8>;
#[tokio::main]
async fn main() {
    let matches = App::new("TiKV client")
        .about("A client to test")
        .author("louis koo")
        .version("unknown")
        .arg(
            Arg::with_name("pd-client")
                .required(true)
                .long("pd-client")
                .takes_value(true)
                .help("Set the pd address"),
        )
        .get_matches();

    let mut address: &str = "";
    if let Some(pd_client) = matches.value_of("pd-client") {
        println!("pd client: {}", pd_client);
        address = pd_client
    }

    let txn = Client::new_with_config(vec![address], Config::default())
        .await
        .expect("Could not create client");

    txn.put("key".to_owned(), "value".to_owned())
        .await
        .expect("Could not set key value");

    let value = txn
        .get("key".to_owned())
        .await
        .expect("Could not get value");
    assert_eq!(value, Some(Value::from("value".to_owned())));
    println!("Get key 'key' returned value {:?}.", value);

    let value1 = txn
        .get("key1".to_owned())
        .await
        .expect("Could not get value");
    assert_eq!(value1, Some(Value::from("value".to_owned())));
    println!("Hello, world!");
}
