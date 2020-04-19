pub trait EnumType: Sized + From<i32> {
    fn values() -> &'static [Self];

    fn value(&self) -> i32;
}
