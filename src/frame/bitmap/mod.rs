use crate::frame::UnderlyingComponent;

#[cfg(test)]
mod test;

pub struct BitMap<'a, const XY: usize> {
    width: u16,
    height: u16,
    map: &'a [UnderlyingComponent; XY],
}

pub struct BitMapIter<'a, const XY: usize> {
    x_pointer: u16,
    y_pointer: u16,
    bitmap: &'a BitMap<'a, XY>,
}

impl<'a, const XY: usize> Iterator for BitMapIter<'a, XY> {
    type Item = &'a UnderlyingComponent;
    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = Option::<Self::Item>::None;
        if self.x_pointer < self.bitmap.width {
            if self.y_pointer < self.bitmap.height {
                ret = self.bitmap.get(self.x_pointer, self.y_pointer);
                self.x_pointer += 1;        
                if self.x_pointer == self.bitmap.width {
                    self.x_pointer = 0;
                    self.y_pointer += 1;
                }
            }
        }
        ret
    }
}

impl<'a, const XY: usize> BitMap<'a, XY> {
    
    pub fn from_slice(width: u16, height: u16, slice: &'a [UnderlyingComponent; XY]) -> Self {
        Self {
            width,
            height,
            map: slice,
        }
    }

    // #[cfg(feature = "std")]
    // pub fn from_vec(width: usize, height: usize, vec: Vec<UnderlyingComponent>) -> Self {
    //     Self {
    //         width,
    //         height,
    //         map: vec.clone(),
    //     }
    // }

    pub fn get(&self, x: u16, y: u16) -> Option<&UnderlyingComponent> {
        if x < self.width && y < self.height {
            let (x, y) = (x as u32, y as u32);
            self.map.get((x * y + x) as usize)
        } else {
            None
        }
    }

    /// Builds a new [BitMapIter] to iterate this [BitMap]
    pub fn iter(&'a self) -> BitMapIter<'a, XY> {
        BitMapIter {
            x_pointer: 0,
            y_pointer: 0,
            bitmap: self,
        }
    }

}