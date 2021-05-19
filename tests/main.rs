#![allow(unused)]

use ::fix_hidden_lifetime_bug::fix_hidden_lifetime_bug;

#[fix_hidden_lifetime_bug]
async fn foo<'a> (a: &'static (), b: &'_ (), c: &'_ ()) {
    /* … */
}

#[fix_hidden_lifetime_bug]
async fn bar<'a> (_: &(), _b: Box<dyn Send>) {
    /* … */
}

#[fix_hidden_lifetime_bug]
fn baz<'a, 'b> (it: &'a mut &'b ()) -> impl 'a + Sized {
    it
}

struct Foo<'inv>(
    fn(&()) -> &mut &'inv (),
);

#[fix_hidden_lifetime_bug]
impl<'b> Foo<'b> {
    #[fix_hidden_lifetime_bug]
    fn baz(&self, _: &()) -> impl '_ + Sized {
        self
    }
}
