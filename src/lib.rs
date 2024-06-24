pub use intermix_macros::Intermix;

#[cfg(test)]
mod tests {
    use super::*;

    struct Foo {
        name: String,
        age: i32,
        height: i32,
    }

    impl Foo {
        fn name(&self) -> &String {
            &self.name
        }
        fn age(&self) -> i32 {
            self.age
        }
        fn height(&self) -> i32 {
            self.height
        }
    }

    struct Bar {
        top_speed: f32,
        acceleration: f32,
    }

    impl Bar {
        fn top_speed(&self) -> f32 {
            self.top_speed
        }
        fn acceleration(&self) -> f32 {
            self.acceleration
        }
    }

    #[derive(Intermix)]
    struct Baz {
        #[mixin(name = "title:&String", age = "i32")]
        foo: Foo,
        #[mixin(top_speed = "f32")]
        bar: Bar,
        color: String,
    }

    impl Baz {
        pub fn math(&self) -> f32 {
            self.foo.height() as f32 * self.bar.acceleration()
        }
        pub fn color(&self) -> &String {
            &self.color
        }
    }

    #[test]
    fn it_works() {
        let baz = Baz {
            foo: Foo {
                name: "Tim".into(),
                age: 32,
                height: 60,
            },
            bar: Bar {
                top_speed: 20.0f32,
                acceleration: 5.0f32,
            },
            color: "Blue".into(),
        };
        assert_eq!(baz.foo.name, "Tim");
        assert_eq!(baz.title(), "Tim");
        assert_eq!(baz.age(), 32);
        assert_eq!(baz.top_speed(), 20.0f32);
        assert_eq!(baz.math(), 300.0f32);
        assert_eq!(baz.color(), "blue");
    }
}
