use feed_rs::parser;
use std::ffi::CStr;
use libc::c_char;
use libc::c_ulong;
use libc::time_t;

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
    data: *mut Vec<FeedEntry>, // for save entries,do not use at c side
}

fn set_string(src:&String, dst:&mut FeedString) {
    dst.data = src.as_bytes().as_ptr() as *const c_char;
    dst.size = src.len() as c_ulong;
}


fn parse_url(url: &str) -> Result<feed_rs::model::Feed, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    let feed_from_xml = parser::parse(resp.as_bytes()).unwrap();
    Ok(feed_from_xml)
}

#[no_mangle]
pub extern fn get_feed(url: *const c_char) -> *mut feed_rs::model::Feed{
    unsafe{
        let cstr = CStr::from_ptr(url).to_str().unwrap();
        let feed = parse_url(cstr);
        match feed {
            Ok(result) => {
                Box::into_raw(Box::new(result))
            }
            Err(e) => {
                println!("Error: {}", e);
                println!("Caused by: {}", e.source().unwrap());
                std::ptr::null_mut()
            }
        }
    }
}

#[no_mangle]
pub extern fn release_feed(feed: *mut*mut feed_rs::model::Feed) {
    unsafe{
        Box::from_raw(*feed);
        *feed = std::ptr::null_mut();
    }
}

#[no_mangle]
pub extern fn get_feedinfo(feed: *mut feed_rs::model::Feed) -> *const FeedInfo{
    let mut info = FeedInfo{
        size:0,
        entries:std::ptr::null_mut(),
        updated:0,
        title:FeedString{
            data:std::ptr::null_mut(),
            size:0
        },
        data:std::ptr::null_mut(),
    };

    unsafe{
        let feed = &*feed;
        info.size = feed.entries.len() as c_ulong;
        let mut entries = Vec::new();
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
        info.data = Box::into_raw(Box::new(entries));
        info.entries = (*info.data).as_mut_ptr();
        Box::into_raw(Box::new(info))
    }
}

#[no_mangle]
pub extern fn release_feedinfo(info: *mut*mut FeedInfo) {
    unsafe{
        let infor = Box::from_raw(*info);
        Box::from_raw(infor.data);

        *info = std::ptr::null_mut();
    }
}
