mod image_charts;
mod uri_properties;

fn main(){
    let image_charts = image_charts::ImageCharts::create();
    let uri = image_charts::ImageCharts::cht(image_charts, "svg".to_string()).chs("300x300".to_string()).to_uri();
    println!("{}", uri);
}