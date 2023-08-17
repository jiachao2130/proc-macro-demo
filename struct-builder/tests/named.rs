use struct_builder::StructBuilder;

#[derive(Debug, StructBuilder)]
struct Person {
    name: String,
    age: u8,
    introduction: String,
}

#[test]
fn builder() {
    let me = Person::builder()
        .name("CC".to_owned())
        .age(29)
        .introduction("Hi, I'm CC.".to_owned())
        .build()
        .unwrap();

    assert_eq!(me.name, "CC");
    assert_eq!(me.age, 29);
    assert_eq!(me.introduction, "Hi, I'm CC.");
}
