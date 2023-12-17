use web_framework::func1;
use web_framework::root;
use pretty_assertions::assert_eq;
use crate::routes::r1;


macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }

#[test]
fn test_add() {
    func1();
    let a = aw!(root());

    assert_eq!(a,"Hello, World!");
    assert_eq!(2,2);
}


#[test]
fn test_add2() {
    let a = r1::func2();

    assert_eq!(a,"func2");
    assert_eq!(2,2);
}