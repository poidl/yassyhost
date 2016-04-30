#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate websocket;
extern crate yassy;
extern crate midi;

mod jack;
mod jack_plugin;
use jack::*;

use std::thread;
// use websocket::{Server, Message, Sender, Receiver};
// use websocket::message::Type;
// use websocket::header::WebSocketProtocol;

use std::ffi::CString;
use std::ffi::CStr;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::ptr;
use std::time::Duration;
// use jack_plugin;
// use yassy::plugin;


extern fn process(jack_nframes_t: u32, ptr: *mut libc::c_void) -> isize {
    unsafe {
        let plugin = ptr as *mut plugin;

        let inport = &(*plugin).in_port;
        let outport = &(*plugin).output;
        let buf = jack_port_get_buffer(inport.handle, jack_nframes_t);
        let out = jack_port_get_buffer (outport.handle, jack_nframes_t) as *mut f32;
        let event_count = jack_midi_get_event_count(buf);

        let mut event = JackMidiEvent{ time: 0, size: 0, buffer: std::ptr::null_mut() as *mut libc::c_uchar};

        let ievent = 0;
        jack_midi_event_get(&mut event, buf, ievent);

        for i in 0..jack_nframes_t {
            if (event.time  == i)  & (ievent < event_count) {
                (*plugin).midievent(&*event.buffer);
                jack_midi_event_get(&mut event, buf, ievent);
            }
            let amp = (*plugin).get_amp();
            *out.offset(i as isize) = amp;
        }


    }
    0
}

pub type plugin<'a> = jack_plugin::jack_plugin<'a>;

fn main() {
    let mut p = plugin::new("yassyhost");
    p.set_fs();
    p.connect();
    let cbpluginptr = (&p as *const plugin) as *const libc::c_void;
    unsafe {
        let rval = jack_set_process_callback(p.client, process, cbpluginptr);
        let ga = jack_activate(p.client);
    }
    let five = Duration::new(5, 0);
    while true {
        std::thread::sleep(five)
    }

}
