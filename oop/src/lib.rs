mod complex;
use complex::Complex;

mod part_ord;
use part_ord::{Status::*, *};

mod iter_;
use iter_::Archiver;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complex() {
        let mut complex = Complex::new(8, 7);
        complex.add(&Complex::new(9, 8));
        complex.sub(&Complex::from((7, 6)));
        assert_eq!(complex, Complex::new(10, 9));
    }

    #[test]
    fn test_part_ord() {
        let mut statuses = vec![Done, InProgress, Planned, Planned, Done, InProgress];
        sort_status(&mut statuses);
        assert_eq!(statuses, vec![Done, Done, InProgress, InProgress, Planned, Planned])
    }

    #[test]
    fn test_iter() {
        let mut archiver = Archiver::new();
        archiver.push(90); // value = 90
        archiver.push(67); // value = 67, archive = [90]
        archiver.push(88); // value = 88, archive = [90, 67]
        // double each value in the archive
        archiver.archive = archiver.clone().into_iter().map(|x| x * 2).collect(); // archive =
                                                                                  // [180, 134]
        assert_eq!(archiver, Archiver{
            value: Some(88),
            archive: vec![180, 134]
        })
    }
}
