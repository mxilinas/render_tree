#[cfg(test)]
mod tests {
    use draw::*;

    use crate::{get_center, rect, get_bounds};

    use super::*;

    #[test]
    fn test_get_center() {
        let mut r0 = rect().with_xy(0.0, 3.0);
        let mut r1 = rect().with_xy(4.0, 1.0);

        r0.display_list.add(r1);
        let result0 = get_center(&r0);

        assert_eq!(result0.0, 2.0);
        assert_eq!(result0.1, 2.0);
    }

    #[test]
    fn test_get_bounds() {
        let mut r0 = rect().with_xy(25.0, 3.0);
        let mut r1 = rect().with_xy(40.0, 12.0);
        let mut r2 = rect().with_xy(15.0, 8.0);
        let mut r3 = rect().with_xy(8.0, 8.0);

        r1.display_list.add(r0);
        r2.display_list.add(r1);
        r3.display_list.add(r2);

        let result0 = get_bounds(&r3);

        assert_eq!(result0.right, 40.0);
        assert_eq!(result0.left, 8.0);
        assert_eq!(result0.top, 3.0);
        assert_eq!(result0.bottom, 12.0);
    }
}
