use std::mem;

// NOTE: copy from https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_cached_key
pub(crate) fn sort_by_cached_key<T, K, F>(list: &mut Vec<T>, desc: bool, f: F)
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    macro_rules! sort_by_key {
        ($t:ty, $slice:ident, $f:ident) => {{
            let mut indices: Vec<_> = $slice
                .iter()
                .map($f)
                .enumerate()
                .map(|(i, k)| (k, i as $t))
                .collect();

            if desc {
                indices.sort_unstable_by(|a, b| b.cmp(a));
            } else {
                indices.sort_unstable_by(|a, b| a.cmp(b));
            }

            for i in 0..$slice.len() {
                let mut index = indices[i].1;
                while (index as usize) < i {
                    index = indices[index as usize].1;
                }
                indices[i].1 = index;
                $slice.swap(i, index as usize);
            }
        }};
    }

    let sz_u8 = mem::size_of::<(K, u8)>();
    let sz_u16 = mem::size_of::<(K, u16)>();
    let sz_u32 = mem::size_of::<(K, u32)>();
    let sz_usize = mem::size_of::<(K, usize)>();

    let len = list.len();
    if len < 2 {
        return;
    }
    if sz_u8 < sz_u16 && len <= (u8::MAX as usize) {
        return sort_by_key!(u8, list, f);
    }
    if sz_u16 < sz_u32 && len <= (u16::MAX as usize) {
        return sort_by_key!(u16, list, f);
    }
    if sz_u32 < sz_usize && len <= (u32::MAX as usize) {
        return sort_by_key!(u32, list, f);
    }
    sort_by_key!(usize, list, f)
}
