#[cfg(test)]
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
