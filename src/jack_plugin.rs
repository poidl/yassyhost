#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

extern crate libc;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
// use plugin;
use midi;
use midi::*;
use yassy::plugin;
use std::str;
use jack::*;

#[repr(C)]
pub struct jack_plugin<'a> {
    name: &'a str,
    pub client: *mut jack_client_t,
    pub in_port: port,
    pub output: port,
    plugin: plugin::SynthPlugin,
//     pub output: *mut f32,
    // ports: &'a [port]
}

pub struct port {
    pub handle: *mut jack_port_t,
    pub data: *mut f32
}

impl<'a> jack_plugin<'a> {
    pub fn new(hostname: &str) -> jack_plugin {
        let mut h = jack_plugin{
            name: hostname,
            client: ptr::null_mut(),
            in_port: port{handle: ptr::null_mut(), data: ptr::null_mut() },
            output: port{handle: ptr::null_mut(), data: ptr::null_mut() },
            plugin: plugin::SynthPlugin::new()
        };
        let jo = JackNullOption;
        let js = JackNullStatus;
        let nameptr = CString::new(h.name).unwrap().as_ptr();
        unsafe {
            h.client = jack_client_open(nameptr, jo, &js);
        }
        if h.client == 0 as *mut jack_client_t {
            println!("jack server not running?");
            // TODO return error
        };
        h
    }
    pub fn connect(&mut self) {
        unsafe {
            let portname = CString::new("midi_in").unwrap().as_ptr();
            let porttype = CString::new("8 bit raw midi").unwrap().as_ptr();
            self.in_port.handle = jack_port_register(self.client, portname, porttype, JackPortIsInput, 32768u64);

            let portname = CString::new("audio_out").unwrap().as_ptr();
            let porttype = CString::new("32 bit float mono audio").unwrap().as_ptr();
            self.output.handle = jack_port_register(self.client, portname, porttype, JackPortIsOutput, 32768u64);
        }
    }
    pub fn midievent(&mut self, msg: &u8) {
        let mm = msg as midi::MidiMessage;
        self.plugin.midievent(mm)
    }
}

// pub trait isJackSynthPlugin: {
//     fn connect_port(&mut self, u32, *mut libc::c_void);
//     fn midievent(&mut self, msg: &u8) ;
//     fn set_fs(&mut self, f64);
//     fn get_amp(&mut self) -> f32;
//     fn map_params(&mut self, u32, *mut libc::c_void);
// }

//
// #[repr(C)]
// pub struct JackSynthPlugin {
//     pub in_port: *mut jack_port_t,
//     pub output: *mut f32,
//     pub plugin: plugin::SynthPlugin,
// }
//
// impl  JackSynthPlugin {
//     pub fn new() -> JackSynthPlugin {
//         // let np = ptr::null();
//         let mut jackplugin = JackSynthPlugin {
//             in_port: ptr::null_mut(),
//             output: ptr::null_mut(),
//             plugin: plugin::SynthPlugin::new(),
//         };
//         // TODO
//         jackplugin.in_port = jackplugin.plugin.midi_in as *mut jack_port_t;
//         jackplugin.output = jackplugin.plugin.audio_out;
//         jackplugin
//     }
// }
//

// impl isLv2SynthPlugin for Lv2SynthPlugin {
//     fn connect_port(&mut self, port: u32, data: *mut libc::c_void) {
//         match port {
//             0 => unsafe{self.in_port = data  as *const lv2::LV2_Atom_Sequence},
//             1 => unsafe{self.output = data as *mut f32 },
//             _ => self.map_params(port,data)
//         }
//     }
//     fn map_params(&mut self, port: u32, data: *mut libc::c_void) {
//         let nparams = 1;
//         let iport = port - 2; //TODO: don't hardcode number of input/output ports
//         if (iport <= nparams-1) {
//             println!("connecting port: {}", port);
//             unsafe{self.plugin.synth.params[iport as usize]= &*(data  as *mut f32) };
//             // println!("param: {}",  *(self.synth.params[0]));
//         } else {
//             panic!("Not a valid PortIndex: {}", iport)
//         }
//     }
//     fn midievent(&mut self, msg: &u8) {
//         let mm = msg as midi::MidiMessage;
//         self.plugin.midievent(mm)
//     }
//     fn set_fs(&mut self, fs: f64) {
//         self.plugin.set_fs(fs);
//     }
//     fn get_amp(&mut self) -> f32 {
//         self.plugin.get_amp()
//     }
// }
