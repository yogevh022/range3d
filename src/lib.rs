#[derive(Clone)]
pub struct Range3D {
    x: isize,
    y: isize,
    z: isize,
    max_x: isize,
    max_y: isize,
    max_z: isize,
}

impl Range3D {
    pub fn new(
        min_x: isize,
        min_y: isize,
        min_z: isize,
        max_x: isize,
        max_y: isize,
        max_z: isize,
    ) -> Self {
        assert!(max_x < isize::MAX);
        assert!(max_y < isize::MAX);
        assert!(max_z < isize::MAX);
        Self {
            x: min_x,
            y: min_y,
            z: min_z,
            max_x,
            max_y,
            max_z,
        }
    }
}

impl Iterator for Range3D {
    type Item = (isize, isize, isize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.max_x {
            return None;
        }
        let out = (self.x, self.y, self.z);

        self.z = unsafe { self.z.unchecked_add(1) };
        if self.z >= self.max_z {
            self.z = 0;
            self.y = unsafe { self.y.unchecked_add(1) };
            if self.y >= self.max_y {
                self.y = 0;
                self.x = unsafe { self.x.unchecked_add(1) };
            }
        }

        Some(out)
    }
}

impl ExactSizeIterator for Range3D {
    fn len(&self) -> usize {
        (self.max_x - self.x) as usize
            * (self.max_y - self.y) as usize
            * (self.max_z - self.z) as usize
    }
}