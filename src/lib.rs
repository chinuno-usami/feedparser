use feed_rs::parser;
use std::ffi::CStr;
use libc::c_char;
use libc::c_ulong;
use libc::time_t;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct WrapError(String);

impl fmt::Display for WrapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Feed parse error: {}", self.0)
        }
}

impl Error for WrapError {}


#[repr(C)]
pub struct FeedString {
    data: *const c_char,
    size: c_ulong,
}

#[repr(C)]
pub struct FeedEntry {
    updated: time_t,
    published: time_t,
    id: FeedString,
    title: FeedString,
    link: FeedString,
    summary: FeedString,
}

#[repr(C)]
pub struct FeedInfo {
    size: c_ulong,
    entries: *mut FeedEntry,
    updated: time_t,
    title: FeedString,
    datav: *mut Vec<FeedEntry>, // for save entries,do not use at c side
    dataf: *mut feed_rs::model::Feed,
}

fn set_string(src:&String, dst:&mut FeedString) {
    dst.data = src.as_bytes().as_ptr() as *const c_char;
    dst.size = src.len() as c_ulong;
}


fn parse_url(url: &str) -> Result<feed_rs::model::Feed, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?;
    if resp.status().is_success(){
        let body = resp.text()?;
        let feed_from_xml = parser::parse(body.as_bytes());//.unwrap();
        match feed_from_xml {
            Ok(_) => {
                Ok(feed_from_xml.unwrap())
            },
            _ => {
                Err(Box::new(WrapError(format!("body:{}", body).into())))
            }
        }
    } else {
        Err(Box::new(WrapError(format!("request failed with status:{:?}", resp.status()).into())))
    }
}

#[no_mangle]
pub extern fn feedparser_parse_url(url: *const c_char) -> *mut FeedInfo{
    unsafe{
        let cstr = CStr::from_ptr(url).to_str().unwrap();
        let feed = parse_url(cstr);
        match feed {
            Ok(result) => {
                let feedp = Box::new(result);
                let mut infop = Box::new(FeedInfo{
                    size:0,
                    entries:std::ptr::null_mut(),
                    updated:0,
                    title:FeedString{
                        data:std::ptr::null_mut(),
                        size:0
                    },
                    datav:std::ptr::null_mut(),
                    dataf:std::ptr::null_mut(),
                });
                let info = &mut *infop;

                let feed = &*feedp;
                info.size = feed.entries.len() as c_ulong;
                let mut entriesp = Box::new(Vec::new());
                let entries = &mut *entriesp;
                for entry in feed.entries.iter() {
                    let mut entry_info = FeedEntry{
                        updated: 0,
                        published: 0,
                        id: FeedString{
                            data:std::ptr::null_mut(),
                            size:0
                        },
                        title: FeedString{
                            data:std::ptr::null_mut(),
                            size:0
                        },
                        link: FeedString{
                            data:std::ptr::null_mut(),
                            size:0
                        },
                        summary: FeedString{
                            data:std::ptr::null_mut(),
                            size:0
                        },
                    };

                    set_string(&entry.id, &mut entry_info.id);
                    if let Some(updated) = &entry.updated {
                        entry_info.updated = updated.timestamp();
                    }
                    if let Some(published) = &entry.published {
                        entry_info.published = published.timestamp();
                    }
                    if let Some(title) = &entry.title {
                        set_string(&title.content, &mut entry_info.title);
                    }
                    if let Some(summary) = &entry.summary {
                        set_string(&summary.content, &mut entry_info.summary);
                    }
                    if !entry.links.is_empty(){
                        let link = &entry.links[0];
                        set_string(&link.href, &mut entry_info.link);
                    }
                    entries.push(entry_info);

                }
                match &feed.updated {
                    Some(date) => {
                        info.updated = date.timestamp();
                    },
                    None => ()
                }
                match &feed.title{
                    Some(title) => {
                        let content = &title.content;
                        set_string(&content, &mut info.title);
                    },
                    None => ()
                }
                info.datav = Box::into_raw(entriesp);
                info.entries = (*info.datav).as_mut_ptr();
                info.dataf = Box::into_raw(feedp);
                Box::into_raw(infop)
            }
            Err(e) => {
                println!("Error: {}", e);
                std::ptr::null_mut()
            }
        }
    }

}

fn release_feed(feed: *mut*mut feed_rs::model::Feed) {
    unsafe{
        Box::from_raw(*feed);
        *feed = std::ptr::null_mut();
    }
}


#[no_mangle]
pub extern fn feedparser_release_feedinfo(info: *mut*mut FeedInfo) {
    unsafe{
        let mut infor = Box::from_raw(*info);
        release_feed(&mut infor.dataf);
        Box::from_raw(infor.datav);

        *info = std::ptr::null_mut();
    }
}
