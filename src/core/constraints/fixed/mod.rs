/// represent id have to equal to value
struct Fixed<V: PartialEq> {
    id: usize,
    value: V,
}
