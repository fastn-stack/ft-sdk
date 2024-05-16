fn main() {
    let mut c: Conn<true> = Conn {};
    let mut c2 = Conn::<false> {};

    needs_true(&mut c);
    // needs_true(&mut c2);
    needs_false(&mut c2);
    needs_false(&mut c);
}

fn needs_true(c: &mut Conn<true>) {
    println!("needs_true");
    needs_false(c);
}

fn needs_false(_c: &mut Conn<false>) {
    println!("needs_false");
}

fn needs_false2<const W: bool>(_c: &mut Conn<W>) {
    println!("needs_false");
}

fn needs_false3(_c: &mut Conn) {
    println!("needs_false");
}

struct Conn<const W: bool = false> {}

impl std::ops::Deref for Conn<true> {
    type Target = Conn<false>;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl std::ops::DerefMut for Conn<true> {
    fn deref_mut(&mut self) -> &mut Conn<false> {
        todo!()
    }
}
