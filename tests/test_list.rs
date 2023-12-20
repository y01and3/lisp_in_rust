use lisp_in_rust::eval::num::{int::Int, number::Number, real::Real};
use lisp_in_rust::eval::{expr::Expr, list::List};

#[test]
fn test_list_car_cdr() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = Expr::List(List::new(Some(num.clone()), None));
    let listf = List::new(
        Some(Expr::List(List::NIL)),
        Some(Expr::List(List::new(
            Some(Expr::List(List::new(
                Some(Expr::List(List::NIL)),
                Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
            ))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        ))),
    );
    assert_eq!(listf.car(), Some(&Expr::List(List::NIL)));
    assert_eq!(
        listf.cdr(),
        Some(&Expr::List(List::new(
            Some(Expr::List(List::new(
                Some(Expr::List(List::NIL)),
                Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
            ))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        )))
    );
}
#[test]
fn test_list_atom() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = List::new(Some(num.clone()), Some(num.clone()));

    assert_eq!(list.atom(), false);
    assert_eq!(List::NIL.atom(), true);
}

#[test]
fn test_list_equal() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = Expr::List(List::new(Some(num.clone()), None));
    let listf = List::new(
        Some(Expr::List(List::NIL)),
        Some(Expr::List(List::new(
            Some(Expr::List(List::new(
                Some(Expr::List(List::NIL)),
                Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
            ))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        ))),
    );
    assert_eq!(listf.equal(listf.clone()), true);
    assert_eq!(listf.equal(List::NIL), false);
    assert_eq!(listf.equal(List::new(Some(num.clone()), None)), false);
    assert_eq!(List::NIL.equal(List::new(None, None)), true);
}

#[test]
fn test_nth() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = Expr::List(List::new(Some(num.clone()), None));
    let listf = List::new(
        Some(Expr::List(List::NIL)),
        Some(Expr::List(List::new(
            Some(Expr::List(List::new(Some(num.clone()), None))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        ))),
    );
    assert_eq!(listf.nth(1), Some(list));
}

#[test]
fn test_nthcdr() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = Expr::List(List::new(Some(num.clone()), None));
    let listf = List::new(
        Some(Expr::List(List::NIL)),
        Some(Expr::List(List::new(
            Some(Expr::List(List::new(Some(num.clone()), None))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        ))),
    );
    let listr = Some(Expr::List(List::new(
        Some(Expr::List(List::new(Some(num.clone()), None))),
        Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
    )));
    assert_eq!(listf.nthcdr(1), listr);
}

#[test]
fn test_last() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = Expr::List(List::new(Some(num.clone()), None));
    let listf = List::new(
        Some(Expr::List(List::NIL)),
        Some(Expr::List(List::new(
            Some(Expr::List(List::new(Some(num.clone()), None))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        ))),
    );
    assert_eq!(List::last(&listf), List::new(Some(num.clone()), None));
}

#[test]
fn test_stack() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let mut list = List::new(Some(num.clone()), None);
    let list_e = Expr::List(list.clone());
    list.push(Some(num.clone()));
    assert_eq!(list, List::new(Some(num.clone()), Some(list_e.clone())));
    assert_eq!(list.pop(), Some(num.clone()));
    assert_eq!(list, List::new(Some(num.clone()), None));
}

#[test]
fn test_length() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = Expr::List(List::new(Some(num.clone()), None));
    let listf = List::new(
        Some(Expr::List(List::NIL)),
        Some(Expr::List(List::new(
            Some(Expr::List(List::new(
                Some(Expr::List(List::NIL)),
                Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
            ))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        ))),
    );
    assert_eq!(listf.length(), 4);
}

#[test]
fn test_list_iter() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = Expr::List(List::new(Some(num.clone()), None));
    let listf = List::new(
        Some(Expr::List(List::NIL)),
        Some(Expr::List(List::new(
            Some(Expr::List(List::new(
                Some(Expr::List(List::NIL)),
                Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
            ))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        ))),
    );
    assert_eq!(
        listf
            .clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        vec!["NIL", "(NIL (1) . 1)", "1", "1"]
    );
}

#[test]
fn test_list_display() {
    let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
    let list = Expr::List(List::new(Some(num.clone()), None));
    let listf = List::new(
        Some(Expr::List(List::NIL)),
        Some(Expr::List(List::new(
            Some(Expr::List(List::new(
                Some(Expr::List(List::NIL)),
                Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
            ))),
            Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
        ))),
    );
    assert_eq!(listf.to_string(), "(NIL (NIL (1) . 1) 1 1)");
}
