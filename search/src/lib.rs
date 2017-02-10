

pub fn search(ref_string :&str,
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

pub fn find_substring(full : &str, pattern : &str) -> Option<usize> {
    full.find(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_substring() {
        let res = find_substring("abc", "bc");
        assert_eq!(res, Some(1));
    }

    // #[test]
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

        assert_eq!(search(text, &suffix_array, "e"), [1,1]);
    }

    #[test]
    fn test_suffix_array_banana() {
        let text = "banana ";
        let suffix_array = [6,5,3,1,0,4,2];
        assert_eq!(search(text, &suffix_array, "ba"), [4,4]);
        assert_eq!(search(text, &suffix_array, "na"), [5,7]);
        assert_eq!(search(text, &suffix_array, "a"), [1,4]);
    }

}
