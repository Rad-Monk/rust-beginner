use calc::add;
use calc::div;
use calc::mul;
use calc::sub;

#[test]
fn test_add() {
    assert_eq!(add(5, 5), 10)
}

#[test]
fn test_sub() {
    assert_eq!(sub(5, 5), 0)
}

#[test]
fn test_mul() {
    assert_eq!(mul(5, 5), 25)
}

#[test]
fn test_div() {
    assert_eq!(div(5, 5), 1)
}
