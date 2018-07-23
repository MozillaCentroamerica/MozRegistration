table! {
    personas (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        lastname -> Varchar,
        country -> Varchar,
        province -> Nullable<Varchar>,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        sex -> Varchar,
        comment1 -> Nullable<Text>,
        comment2 -> Nullable<Text>,
        comment3 -> Nullable<Text>,
        age -> Nullable<Integer>,
        mozilla_news -> Integer,
    }
}
