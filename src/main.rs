use warp::Filter;
use std::fs;

#[tokio::main]
async fn main() {
    let keys = warp::any().map(|| {
        let mut contents: String  = "".to_string();

        for entry in fs::read_dir("/usr/src/app/keys").expect("yeet") {
            let entry = entry.expect("Could not read this entry");
            let path = entry.path();

            let curContents = fs::read_to_string(path)
                .expect("Something went wrong reading file.");

            contents.push_str(&curContents);
        }
        
        contents
    });

    warp::serve(keys)
        .run(([0, 0, 0, 0], 8080))
        .await;
}