pub fn bubble_sort(v: &mut Vec<i64>) -> Vec<i64> {
    let mut unsorted = true;
    let mut current;
    let mut other;
    let mut temp;
    let length = v.len();

    while unsorted {
        unsorted = false;
        for i in range(0, length-1) {
            current = v[i];
            other = v[i+1];
            if current > other {
                temp = other;
                other = current;
                current = temp;
                unsorted = true;
            }
        }
    }

    v
}
