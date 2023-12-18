pub mod sort_algorithms {
    use yew::Callback;
    use crate::components::Array;

    type SwapCallback = dyn FnOnce();
    pub(crate) fn insertion_sort(array: &mut Array, swap_callback: Callback<Box<SwapCallback>, ()>) {
        let mut j = 0;
        let n = array.len() as i32;
        for i in 1..n {
            j = i;
            while j > 0 && array[j as usize - 1] > array[j as usize] {
                array.swap(j as usize, j as usize - 1);
                swap_callback.emit(Box::new(|| ()));
                j -= 1;
            }
        }
    }
}