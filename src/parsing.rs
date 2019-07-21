extern crate num;
use num::ToPrimitive;

pub struct PositionedBuffer {
    data: Vec<u8>,
    pub pos: usize,
}

impl PositionedBuffer {
    pub fn new(buff: Vec<u8>, pos: usize) -> PositionedBuffer {
        PositionedBuffer { data: buff, pos }
    }

    pub fn read_u8(&mut self) -> u8 {
        //println!("Pos: {} u8", self.pos);
        self.pos += 1;
        self.data[self.pos - 1]
    }

    pub fn read_u16(&mut self) -> u16 {
        //println!("Pos: {} u16", self.pos);
        self.pos += 2;

        self.data[self.pos - 2] as u16 | (self.data[self.pos - 1] as u16) << 8
    }

    pub fn read_u32(&mut self) -> u32 {
        //println!("Pos: {} u32", self.pos);
        self.pos += 4;

        self.data[self.pos - 4] as u32
            | (self.data[self.pos - 3] as u32) << 8
            | (self.data[self.pos - 2] as u32) << 16
            | (self.data[self.pos - 1] as u32) << 24
    }

    pub fn read_bool(&mut self) -> bool {
        //print!("B");
        if self.read_u8() != 0 {
            true
        } else {
            false
        }
    }

    pub fn read_u64(&mut self) -> u64 {
        //println!("Pos: {} u64", self.pos);
        let mut raw = [0; 8];
        for offset in 0..8 {
            raw[offset] = self.data[self.pos + offset];
        }
        self.pos += 8;
        u64::from_le_bytes(raw)
    }

    pub fn read_f64(&mut self) -> f64 {
        //print!("F");
        f64::from_bits(self.read_u64())
    }

    pub fn read_f32(&mut self) -> f32 {
        //print!("F");
        f32::from_bits(self.read_u32())
    }
    pub fn print_location(&self) -> Vec<String> {
        //println!("Vector {}", self.pos);
        Vec::new()
    }

    pub fn read_u128(&mut self) -> u128 {
        //println!("Pos: {} u128", self.pos);
        let mut raw = [0; 16];
        for offset in 0..16 {
            raw[offset] = self.data[self.pos + offset];
        }
        self.pos += 16;
        u128::from_le_bytes(raw)
    }

    pub fn read_pstring(&mut self) -> String {
        //println!("Pos: {} string", self.pos);
        // A single prefixed byte determines the strings length
        // max length is therefor 255
        let size: usize = self.data[self.pos] as usize;

        let mut string = String::with_capacity(size);
        for offset in 1..(size + 1) {
            string.push(self.data[self.pos + offset] as char)
        }
        self.pos += size + 1;

        string
    }

    /// Read a list of generic types
    ///
    /// * `reader` - A function that reads the specified type
    ///
    /// # Example
    /// `
    /// // Setup the data
    /// let data: Vec<u8> = vec!
    ///     [
    ///         0x03, 0x00, 0x00, 0x00, // u32 denoting size
    ///         1, // First u8
    ///         2,
    ///         3, // Last u8
    ///     ];
    ///
    /// let mut posbuff = PositionedBuffer::new(data, 0);
    ///
    /// // Use the read_8 method to read items
    /// // read_u8 -> u8 therefor read_list -> Vec<u8>
    /// let result = posbuff.read_list(&mut PositionedBuffer::read_u8);
    ///
    /// assert_eq!(result, vec![1, 2, 3]);
    /// `
    pub fn read_list<T, G: ToPrimitive>(
        &mut self,
        reader: &mut FnMut(&mut Self) -> T,
        sizer: &mut FnMut(&mut Self) -> G,
    ) -> Vec<T> {
        let size = ToPrimitive::to_usize(&sizer(self)).unwrap();
        let mut items = Vec::with_capacity(size);

        for _ in 0..size {
            let v: T = reader(self);
            items.push(v);
        }

        items
    }
}
