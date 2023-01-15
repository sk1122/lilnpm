use futures::{future::join_all};

#[tokio::main]
async fn main() {
    let r = vec![
        "@wagpay/sdk",
        "@wagpay/id",
        "@fetcch/id",
        "@wagpay/12",
        "@wagpay/fds",
        "@fetcch/fds",
        "@wagpay/fdsf",
        "@wagpay/rewrew",
        "@fetcch/ffdsfd",
        "@wagpay/sfdsf",
        "@wagpay/ifdsf",
        "@fetcch/ifdsfdf",
    ];
    let v = vec![
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
        "https://registry.npmjs.org/@wagpay/sdk/-/sdk-0.0.1.tgz",
    ];

    let mut responses = vec![];
    
    for i in 0..3 {
        responses.push(download::download_package(r[i], v[i]));
    }

    // let results = join_all(responses).await;

    // println!("{:?}", results);
}

pub mod download;