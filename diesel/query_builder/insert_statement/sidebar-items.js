initSidebarItems({"struct":[["BatchInsertStatement","The result of calling `insert(records).into(some_table)` when `records` is a slice or a `Vec`. When calling methods from `ExecuteDsl` or `LoadDsl`. When the given slice is empty, this struct will not execute any queries. When the given slice is not empty, this will execute a single bulk insert on backends which support the `DEFAULT` keyword, and one query per record on backends which do not (SQLite)."],["DefaultInsertStatement",""],["IncompleteDefaultInsertStatement","The structure returned by `insert_default_values`. The only thing that can be done with it is call `into`."],["IncompleteInsertStatement","The structure returned by `insert`. The only thing that can be done with it is call `into`."],["Insert",""],["InsertStatement",""]],"trait":[["IntoInsertStatement",""],["UndecoratedInsertRecord","Marker trait to indicate that no additional operations have been added to a record for insert. Used to prevent things like `insert(&vec![user.on_conflict_do_nothing(), user2.on_conflict_do_nothing()])` from compiling."]]});