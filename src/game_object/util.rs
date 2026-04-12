pub fn mut_two<T> (first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index < second_index);
    let (first_slice, second_slice) = items.split_at_mut(second_index);
    (&mut first_slice[first_index], &mut second_slice[0])
}
