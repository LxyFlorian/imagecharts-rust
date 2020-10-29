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

    //Format: &amp;lt;data_series_1_label&amp;gt;|...|&amp;lt;data_series_n_label&amp;gt;. The text for the legend entries. Each label applies to the corresponding series in the chd array. Use a + mark for a space. If you do not specify this parameter, the chart will not get a legend. There is no way to specify a line break in a label. The legend will typically expand to hold your legend text, and the chart area will shrink to accommodate the legend.
    pub fn chdl(self, chdl: String) -> Self {
        Self::clone(self, "chdl".to_string(), chdl.to_string())
    }

    //Specifies the color and font size of the legend text. &lt;color&gt;,&lt;size&gt;
    pub fn chdls(self, chdls: String) -> Self {
        Self::clone(self, "chdls".to_string(), chdls.to_string())
    }

    //Solid or dotted grid lines
    pub fn chg(self, chg: String) -> Self {
        Self::clone(self, "chg".to_string(), chg.to_string())
    }

    /**
     can specify the colors of a specific series using the chco parameter.
	*       Format should be &amp;lt;series_2&amp;gt;,...,&amp;lt;series_m&amp;gt;, with each color in RRGGBB format hexadecimal number.
	*       The exact syntax and meaning can vary by chart type; see your specific chart type for details.
	*       Each entry in this string is an RRGGBB[AA] format hexadecimal number.
	*       If there are more series or elements in the chart than colors specified in your string, the API typically cycles through element colors from the start of that series (for elements) or for series colors from the start of the series list.
	*       Again, see individual chart documentation for details.
	*/
    pub fn chco(self, chco: String) -> Self {
        Self::clone(self, "chco".to_string(), chco.to_string())
    }

    //Chart title
    pub fn chtt(self, chtt: String) -> Self {
        Self::clone(self, "chtt".to_string(), chtt.to_string())
    }

    //Format should be &#34;&lt;color&gt;,&lt;font_size&gt;[,&lt;opt_alignment&gt;,&lt;opt_font_family&gt;,&lt;opt_font_style&gt;]&#34;, opt_alignement is not supported
    pub fn chts(self, chts: String) -> Self {
        Self::clone(self, "chts".to_string(), chts.to_string())
    }

    //Specify which axes you want (from: &#34;x&#34;, &#34;y&#34;, &#34;t&#34; and &#34;r&#34;). You can use several of them, separated by a coma; for example: &#34;x,x,y,r&#34;. Order is important.
    pub fn chxt(self, chxt: String) -> Self {
        Self::clone(self, "chxt".to_string(), chxt.to_string())
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
