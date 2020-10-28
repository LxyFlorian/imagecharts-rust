use std::collections::HashMap;
use std::cell::RefCell;

pub struct ImageCharts {
    pub query: RefCell<HashMap<String, String>>
}

impl ImageCharts {
    pub fn create() -> Self {
        ImageCharts { query: RefCell::new(HashMap::new()) }
    }

    fn clone(self, key: String, value: String) -> Self{
        self.query.borrow_mut().insert(key.to_string(), value.to_string());
        self
    }
    
    //Chart type
    pub fn cht(self, cht: String) -> Self {
        Self::clone(self, "cht".to_string(), cht.to_string())
    }

    //Chart Data
    pub fn chd(self, chd: String) -> Self {
        Self::clone(self, "chd".to_string(), chd.to_string())
    }

    //You can configure some charts to scale automatically to fit their data with chds=a. The chart will be scaled so that the largest value is at the top of the chart and the smallest (or zero, if all values are greater than zero) will be at the bottom. Otherwise the &#34;&amp;lg;series_1_min&amp;gt;,&amp;lg;series_1_max&amp;gt;,...,&amp;lg;series_n_min&amp;gt;,&amp;lg;series_n_max&amp;gt;&#34; format set one or more minimum and maximum permitted values for each data series, separated by commas. You must supply both a max and a min. If you supply fewer pairs than there are data series, the last pair is applied to all remaining data series. Note that this does not change the axis range; to change the axis range, you must set the chxr parameter. Valid values range from (+/-)9.999e(+/-)199. You can specify values in either standard or E notation.
    pub fn chds(self, chds: String) -> Self {
        Self::clone(self, "chds".to_string(), chds.to_string())
    }
    
    
    //How to encode the data in the QR code. &#39;UTF-8&#39; is the default and only supported value. Contact our team if you wish to have support for Shift_JIS and/or ISO-8859-1.
    pub fn choe(self, choe: String) -> Self {
        Self::clone(self, "choe".to_string(), choe.to_string())
    }

    //QRCode error connection level and optionnal margin
    pub fn chld(self, chld: String) -> Self {
        Self::clone(self, "chld".to_string(), chld.to_string())
    }

    //can specify the range of values that appear on each axis independently, using the chxr parameter. Note that this does not change the scale of the chart elements (use chds for that), only the scale of the axis labels.
    pub fn chxr(self, chxr: String) -> Self {
        Self::clone(self, "chxr".to_string(), chxr.to_string())
    }

    //Some clients like Flowdock/Facebook messenger and so on, needs an URL to ends with a valid image extension file to display the image, use this parameter at the end your URL to support them. Valid values are &#34;.png&#34;, &#34;.svg&#34; and &#34;.gif&#34;
    pub fn chof(self, chof: String) -> Self {
        Self::clone(self, "chof".to_string(), chof.to_string())
    }

    //Maximum chart size for all charts except maps is 998,001 pixels total (Google Image Charts was limited to 300,000), and maximum width or length is 999 pixels.
    pub fn chs(self, chs: String) -> Self {
        Self::clone(self, "chs".to_string(), chs.to_string())
    }

    //Get the full Image-Charts API url (signed and encoded if necessary, not yet in rust)
    pub fn to_uri(self) -> String {
        let mut final_list = Vec::new();
        //for each key/value in queries, add key=value in final vector
        for (key, value) in self.query.into_inner(){
            final_list.push(key.to_string() + "=" + &value.to_string());
        }
        //create the URI -> TODO : Use a library to create a clean uri
        "https://image-charts.com/chart?".to_owned() + &final_list.join("&")
    }
}
