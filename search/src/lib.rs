#![feature(test)]
extern crate test;

fn suffix_findall(ref_string :&str,
              a : &[usize],
              pattern : &str) -> [usize; 2] {
    let mut l = 0;
    let mut r = a.len();
    while l < r {
        let mid = (l+r) / 2;
        if pattern > &ref_string[a[mid]..a[mid] + pattern.len()] {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    let s = l;
    let mut r = a.len();
    while l < r {
        let mid = (l+r) / 2;
        if pattern >= &ref_string[a[mid]..a[mid] + pattern.len()] {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    return [s, r];
}

pub fn search<'a>(ref_string :&'a str,
              a : &[usize],
              pattern : &str) -> Vec<&'a str> {
    let lr = suffix_findall(ref_string, a, pattern);
    let l = lr[0];
    let r = lr[1];
    let len = pattern.len();
    let mut res = Vec::new();
    for i in l..r {
        res.push(&ref_string[a[i]..a[i] + len])
    }
    return res;
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::test::Bencher;

    #[test]
    fn test_suffix_array_hest() {
        // 0"hest "
        // 1"est "
        // 2"st "
        // 3"t "
        // 4" "

        // 4" "
        // 1"est "
        // 0"hest "
        // 2"st "
        // 3"t "
        let text = "hest ";
        let suffix_array = [4,1,0,2,3];

        assert_eq!(suffix_findall(text, &suffix_array, "e"), [1,2]);
    }

    #[test]
    fn test_suffix_array_banana() {
        let text = "banana ";
        let suffix_array = [6,5,3,1,0,4,2];
        assert_eq!(suffix_findall(text, &suffix_array, "ba"), [4,5]);
        assert_eq!(suffix_findall(text, &suffix_array, "na"), [5,7]);
        assert_eq!(suffix_findall(text, &suffix_array, "a"), [1,4]);
    }

    fn builtin<'a>(text : &'a str, p : &str) -> Vec<&'a str> {
        text.matches(p).collect()
    }

    #[test]
    fn test_something() {
        let text = "banana ";
        let suffix_array = [6,5,3,1,0,4,2];
        let p = "a";
        assert_eq!(builtin(text, p), search(text, &suffix_array, p));
    }

    #[bench]
    fn bench_suffix(b: &mut Bencher) {
        let text = "bananaananananananananananananan ";
        let suffix_array = [6,5,3,1,0,4,2];

        b.iter(|| search(text, &suffix_array, "a"));
    }


    #[bench]
    fn bench_builtin(b: &mut Bencher) {
        let text = "bananaananananananananananananan ";

        b.iter(|| builtin(text, "a"));
    }
}
