use {
    crate::*,
    gluesql_core::{
        error::TranslateError,
        prelude::{Payload::ShowVariable, PayloadVariable, Value::*},
    },
};

test_case!(dictionary, async move {
    let tables = |v: Vec<&str>| {
        Ok(ShowVariable(PayloadVariable::Tables(
            v.into_iter().map(ToOwned::to_owned).collect(),
        )))
    };

    assert!(matches!(
        run!("SHOW VERSION;"),
        ShowVariable(PayloadVariable::Version(_))
    ));

    test!("SHOW TABLES", tables(Vec::new()));

    run!("CREATE TABLE Foo (id INTEGER, name TEXT NULL, type TEXT NULL);");
    test!("SHOW TABLES", tables(vec!["Foo"]));

    run!("CREATE TABLE Zoo (id INTEGER PRIMARY KEY);");
    run!("CREATE TABLE Bar (id INTEGER UNIQUE, name TEXT NOT NULL DEFAULT 'NONE');");

    test!("SHOW TABLES", tables(vec!["Bar", "Foo", "Zoo"]));

    test!(
        "SHOW WHATEVER",
        Err(TranslateError::UnsupportedShowVariableKeyword("WHATEVER".to_owned()).into())
    );

    test!(
        "SHOW ME THE CHICKEN",
        Err(
            TranslateError::UnsupportedShowVariableStatement("SHOW ME THE CHICKEN".to_owned())
                .into()
        )
    );

    test!(
        "SELECT * FROM GLUE_TABLES",
        Ok(select!(
            TABLE_NAME;
            Str;
            "Bar".to_owned();
            "Foo".to_owned();
            "Zoo".to_owned()
        ))
    );

    test!(
        "SELECT * FROM GLUE_TABLE_COLUMNS",
        Ok(select!(
            TABLE_NAME       | COLUMN_NAME      | COLUMN_ID | NULLABLE | KEY                      | DEFAULT;
            Str              | Str              | I64       | Bool     | Str                      | Str;
            "Bar".to_owned()   "id".to_owned()    1           true       "UNIQUE".to_owned()        "".to_owned();
            "Bar".to_owned()   "name".to_owned()  2           false      "".to_owned()              "'NONE'".to_owned();
            "Foo".to_owned()   "id".to_owned()    1           true       "".to_owned()              "".to_owned();
            "Foo".to_owned()   "name".to_owned()  2           true       "".to_owned()              "".to_owned();
            "Foo".to_owned()   "type".to_owned()  3           true       "".to_owned()              "".to_owned();
            "Zoo".to_owned()   "id".to_owned()    1           false      "PRIMARY KEY".to_owned()   "".to_owned()
        ))
    );
});
