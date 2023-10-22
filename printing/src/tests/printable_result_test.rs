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

fn success_with_task_numbers<const N: usize>(
    numbers: [i32; N],
) -> PrintableResult<'static> {
    let tasks = numbers
        .iter()
        .map(|&number| make_printable_task_with_number(number))
        .collect();
    Ok(PrintableAppSuccess {
        tasks,
        ..Default::default()
    })
}

#[test]
fn max_index_digits_for_empty_list() {
    let result = Ok(Default::default());
    assert_eq!(result.max_index_digits(), 1);
}

#[test]
fn max_index_digits_for_list_of_one_single_digit_number() {
    let result = success_with_task_numbers([1]);
    assert_eq!(result.max_index_digits(), 1);
}

#[test]
fn max_index_digits_for_list_of_single_digit_numbers() {
    let result = success_with_task_numbers([1, 2, 3]);
    assert_eq!(result.max_index_digits(), 1);
}

#[test]
fn max_index_digits_for_task_at_index_zero() {
    let result = success_with_task_numbers([0]);
    assert_eq!(result.max_index_digits(), 1);
}

#[test]
fn max_index_digits_for_task_with_negative_number() {
    let result = success_with_task_numbers([-1]);
    assert_eq!(result.max_index_digits(), 2);
}

#[test]
fn max_index_digits_for_task_with_three_digit_number() {
    let result = success_with_task_numbers([100]);
    assert_eq!(result.max_index_digits(), 3);
}

#[test]
fn max_index_digits_for_task_with_negative_three_digit_number() {
    let result = success_with_task_numbers([-100]);
    assert_eq!(result.max_index_digits(), 4);
}

#[test]
fn max_index_digits_for_list_of_three_digit_numbers() {
    let result = success_with_task_numbers([1, 20, 300]);
    assert_eq!(result.max_index_digits(), 3);
}

#[test]
fn max_index_digits_for_list_of_negative_three_digit_numbers() {
    let result = success_with_task_numbers([-1, -20, -300]);
    assert_eq!(result.max_index_digits(), 4);
}

#[test]
fn max_index_digits_for_list_of_mixed_digit_numbers() {
    let result = success_with_task_numbers([-1, 20, -300]);
    assert_eq!(result.max_index_digits(), 4);
}

#[test]
fn max_index_digits_for_list_containing_negative_five_digit_numbers() {
    let result = success_with_task_numbers([-10000, 20000, -30000]);
    assert_eq!(result.max_index_digits(), 6);
}

#[test]
fn max_index_digits_for_app_failure() {
    let result: PrintableResult = Err(vec![]);
    assert_eq!(result.max_index_digits(), 1);
}
