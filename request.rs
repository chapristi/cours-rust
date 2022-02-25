extern crate reqwest; // 0.11.9


fn main(){



    let body = reqwest::get("api.chapristi.tech/api/users")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);
}