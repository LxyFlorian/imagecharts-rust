mod image_charts;

fn to_uri(properties: image_charts::ImageCharts) -> String{
    //initialize the vector of string
    let mut final_list = Vec::new();
    //for each key/value in queries, add key=value in final vector
    for (key, value) in properties.query{
        final_list.push(key.to_string() + "=" + &value.to_string());
    }
    //create the URI -> TODO : Use a library to create a clean uri
    properties.protocol + &properties.host + &properties.port.to_string() + &properties.pathname + &"?".to_string() + &final_list.join("&")
}

fn main() {
    //image chart api properties :
    let mut conn_properties = image_charts::ImageCharts::get();

    //test
    conn_properties.query.insert("cht".to_string(), "bvg".to_string());
    conn_properties.query.insert("chs".to_string(), "300x300".to_string());
    let uri = to_uri(conn_properties);
    println!("{}", uri);
}
