use web_framework::func1;
use web_framework::root;
use pretty_assertions::assert_eq;


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