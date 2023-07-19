use crate::Printable;
use crate::PrintableAppSuccess;
use crate::PrintableResult;
use crate::PrintableTask;

fn make_printable_task_with_number(number: i32) -> PrintableTask<'static> {
    PrintableTask {
        number,
        ..Default::default()
    }
}

#[test]
fn max_index_digits_for_empty_list() {
    let result: PrintableResult = Ok(Default::default());
    assert_eq!(result.max_index_digits(), 1);
}

#[test]
fn max_index_digits_for_list_of_one_single_digit_number() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![make_printable_task_with_number(1)],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 1);
}

#[test]
fn max_index_digits_for_list_of_single_digit_numbers() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![
            make_printable_task_with_number(1),
            make_printable_task_with_number(2),
            make_printable_task_with_number(3),
        ],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 1);
}

#[test]
fn max_index_digits_for_task_at_index_zero() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![make_printable_task_with_number(0)],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 1);
}

#[test]
fn max_index_digits_for_task_with_negative_number() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![make_printable_task_with_number(-1)],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 2);
}

#[test]
fn max_index_digits_for_task_with_three_digit_number() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![make_printable_task_with_number(100)],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 3);
}

#[test]
fn max_index_digits_for_task_with_negative_three_digit_number() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![make_printable_task_with_number(-100)],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 4);
}

#[test]
fn max_index_digits_for_list_of_three_digit_numbers() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![
            make_printable_task_with_number(1),
            make_printable_task_with_number(20),
            make_printable_task_with_number(300),
        ],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 3);
}

#[test]
fn max_index_digits_for_list_of_negative_three_digit_numbers() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![
            make_printable_task_with_number(-1),
            make_printable_task_with_number(-20),
            make_printable_task_with_number(-300),
        ],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 4);
}

#[test]
fn max_index_digits_for_list_of_mixed_digit_numbers() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![
            make_printable_task_with_number(-1),
            make_printable_task_with_number(20),
            make_printable_task_with_number(-300),
        ],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 4);
}

#[test]
fn max_index_digits_for_list_containing_negative_five_digit_numbers() {
    let result: PrintableResult = Ok(PrintableAppSuccess {
        tasks: vec![
            make_printable_task_with_number(-1),
            make_printable_task_with_number(20),
            make_printable_task_with_number(-300),
            make_printable_task_with_number(4000),
            make_printable_task_with_number(-50000),
        ],
        ..Default::default()
    });
    assert_eq!(result.max_index_digits(), 6);
}

#[test]
fn max_index_digits_for_app_failure() {
    let result: PrintableResult = Err(vec![]);
    assert_eq!(result.max_index_digits(), 1);
}
