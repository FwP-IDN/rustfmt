use foo::a;

fn x() {
    println!("hello");
    use foo::c;
    use foo::b2;
    use foo::b;


    use foo::c2;

    {
        let a = 23;

        use foo::c3;
    }
}

use foo::d;

mod z {
    fn y() {
        println!("world");
    }

    use foo::e;

    use foo::e2;
}

use foo::f;

use foo::f2;
