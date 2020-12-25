pub(crate) struct StartEndArrayIterator<'a, T: Sized> {
    current: &'a T,
    end: &'a T,
}

impl<'a, T: Sized> StartEndArrayIterator<'a, T> {
    pub unsafe fn new(start: &'a T, end: &'a T) -> Self {
        Self { current: start, end }
    }
}

impl<'a, T: Sized> Iterator for StartEndArrayIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current as *const _) as usize >= (self.end as *const _) as usize {
            None
        } else {
            self.current = unsafe { &*(self.current as *const T).offset(1) };

            Some(self.current)
        }
    }
}