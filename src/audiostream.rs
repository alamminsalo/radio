extern crate std;
extern crate gst;

use std::str;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn parseString(data: *const c_char) -> String {
    unsafe {
        let buf = CStr::from_ptr(data).to_bytes();
        String::from(str::from_utf8(buf).unwrap())
    }
}

fn parseTagsGetTitleIfPresent(tags: *mut gst::ffi::GstTagList) -> String {
    let mut retval = String::new();

    unsafe {
        let constTags = tags as *const gst::ffi::GstTagList;
        let size = gst::ffi::gst_tag_list_n_tags(constTags);

        for i in 0..size {
            let c_name: *const c_char = gst::ffi::gst_tag_list_nth_tag_name(constTags, i as u32);
            let name = parseString(c_name);
            
            if name == "title" {
                let ptr = CString::new("").unwrap().into_raw() as *mut *mut c_char;

                gst::ffi::gst_tag_list_get_string(constTags, c_name, ptr);

                retval = parseString(*ptr);

                //Return pointer ownership
                CString::from_raw(*ptr);
            }
        }
    }

    retval
}

pub fn open(uri: &str, titleCallback: &Fn(&str)) {

    gst::init();
    if uri.len() <= 0 {
        panic!("no uri supplied");
    };

    let mut playbin = gst::PlayBin::new("audio_player").expect("Couldn't create playbin");
    playbin.set_uri(uri.as_ref());
    let mut mainloop = gst::MainLoop::new();
    let mut bus = playbin.bus().expect("Couldn't get pipeline bus");
    let bus_receiver = bus.receiver();

    mainloop.spawn();
    playbin.play();

    let mut p_title: String = String::new();

    for message in bus_receiver.iter(){
        match message.parse(){
            gst::Message::TagParsed{ref msg, ref tags} => {
                let title = parseTagsGetTitleIfPresent(*tags);

                if title.len() > 0 && p_title != title {
                    p_title = title;
                    titleCallback(&p_title);
                }
            }

            gst::Message::StateChangedParsed{ref old, ref new, ..} => {
//                println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new);
            }

            gst::Message::ErrorParsed{ref error, ref debug, ..} => {
                println!("error msg from element `{}`: {}, {}. Quitting", message.src_name(), error.message(), debug);
                break;
            }

            gst::Message::Eos(_) => {
                println!("Quitting..");
                break;
            }

            _ => {}
        }
    }
    mainloop.quit();
}

