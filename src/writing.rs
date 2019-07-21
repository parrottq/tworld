use std::io::{Seek, SeekFrom, Write};

extern crate num;
use num::FromPrimitive;

pub trait PrimitiveWriting {
    fn write_void(&mut self, _: &usize) {}
    fn write_bool(&mut self, value: &bool);
    fn write_u8(&mut self, value: &u8);
    fn write_u16(&mut self, value: &u16);
    fn write_u32(&mut self, value: &u32);
    fn write_u64(&mut self, value: &u64);
    fn write_u128(&mut self, value: &u128);
    fn write_usize(&mut self, value: &usize);
    fn write_list<T, G: FromPrimitive>(
        &mut self,
        items: &Vec<T>,
        writer: &mut FnMut(&mut Self, &T),
        counter: &mut FnMut(&mut Self, &G),
    );
    fn write_string(&mut self, text: &String);
    fn write_f32(&mut self, value: &f32);
    fn write_f64(&mut self, value: &f64);
    fn current_pos(&mut self) -> usize;
    fn set_current_pos(&mut self, value: u64);
}

impl PrimitiveWriting for std::fs::File {
    fn write_void(&mut self, _: &usize) {}

    fn write_bool(&mut self, value: &bool) {
        self.write(&[if *value { 1 } else { 0 }]).unwrap();
    }

    fn write_u8(&mut self, value: &u8) {
        self.write(&[*value]).unwrap();
    }

    fn write_u16(&mut self, value: &u16) {
        self.write(&value.to_le_bytes()).unwrap();
    }

    fn write_u32(&mut self, value: &u32) {
        self.write(&value.to_le_bytes()).unwrap();
    }

    fn write_u64(&mut self, value: &u64) {
        self.write(&value.to_le_bytes()).unwrap();
    }

    fn write_u128(&mut self, value: &u128) {
        self.write(&value.to_le_bytes()).unwrap();
    }

    fn write_usize(&mut self, value: &usize) {
        self.write(&value.to_le_bytes()).unwrap();
    }

    fn write_list<T, G: FromPrimitive>(
        &mut self,
        items: &Vec<T>,
        writer: &mut FnMut(&mut Self, &T),
        counter: &mut FnMut(&mut Self, &G),
    ) {
        let num: G = FromPrimitive::from_usize(items.len()).unwrap();
        counter(self, &num);
        for item in items.iter() {
            writer(self, item);
        }
    }

    fn write_string(&mut self, text: &String) {
        self.write_u8(&(text.len() as u8));

        self.write(&(text.bytes().map(|x| x as u8).collect::<Vec<u8>>()))
            .unwrap();
    }

    fn write_f32(&mut self, value: &f32) {
        self.write_u32(&value.to_bits());
    }

    fn write_f64(&mut self, value: &f64) {
        self.write_u64(&value.to_bits());
    }

    fn current_pos(&mut self) -> usize {
        self.seek(SeekFrom::Current(0)).unwrap() as usize
    }

    fn set_current_pos(&mut self, value: u64) {
        self.seek(SeekFrom::Start(value)).unwrap();
    }
}
