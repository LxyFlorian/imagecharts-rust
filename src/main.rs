mod image_charts;

fn main(){
    let image_charts = image_charts::ImageCharts::create();
    let uri = image_charts::ImageCharts::cht(image_charts, "bvg".to_string())
    .chs("300x300".to_string())
    .chd("a:60,40".to_string())
    .to_uri();
    println!("{}", uri);
}