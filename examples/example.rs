extern crate imagecharts;

fn main(){
    let mut query_params = imagecharts::QueryParams::created();

    query_params = imagecharts::chs(query_params, "258x258".to_string())
    .chs(query_params, "500x500".to_string());

    imagecharts::to_uri();
}