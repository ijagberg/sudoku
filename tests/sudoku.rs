#![cfg(test)]

use sudoku::Sudoku;

fn get_solvable_9x9_test_instance() -> Sudoku {
    let mut instance = Sudoku::new(9, 3, 3).unwrap();
    instance.populate_from_str(
        r#"_ _ 3 _ 2 _ 6 _ _
           9 _ _ 3 _ 5 _ _ 1
           _ _ 1 8 _ 6 4 _ _
           _ _ 8 1 _ 2 9 _ _
           7 _ _ _ _ _ _ _ 8
           _ _ 6 7 _ 8 2 _ _
           _ _ 2 6 _ 9 5 _ _
           8 _ _ 2 _ 3 _ _ 9
           _ _ 5 _ 1 _ 3 _ _"#,
    );
    instance
}

#[test]
fn is_solved() {
    assert_eq!(false, get_solvable_9x9_test_instance().is_solved());
}

#[test]
fn can_place() {
    let instance = get_solvable_9x9_test_instance();
    assert_eq!(false, instance.can_place_value(0, 0, 3));
}

#[test]
fn col_iter() {
    let instance = get_solvable_9x9_test_instance();
    let values_in_col: Vec<_> = instance.col_iter(2).collect();
    assert_eq!(
        vec![
            Some(3),
            None,
            Some(1),
            Some(8),
            None,
            Some(6),
            Some(2),
            None,
            Some(5)
        ],
        values_in_col
    );
}

#[test]
fn row_iter() {
    let instance = get_solvable_9x9_test_instance();
    let values_in_row: Vec<_> = instance.row_iter(8).collect();
    assert_eq!(
        vec![
            None,
            None,
            Some(5),
            None,
            Some(1),
            None,
            Some(3),
            None,
            None
        ],
        values_in_row
    );
}

#[test]
fn sec_iter() {
    let instance = get_solvable_9x9_test_instance();
    let values_in_sec: Vec<_> = instance.sec_iter(5, 4).collect();
    assert_eq!(
        vec![
            Some(1),
            None,
            Some(2),
            None,
            None,
            None,
            Some(7),
            None,
            Some(8)
        ],
        values_in_sec
    );
}
