pub mod changeset;
pub mod target;

pub use self::changeset::{Changeset, AsChangeset};
pub use self::target::{UpdateTarget, IntoUpdateTarget};

use backend::Backend;
use expression::{Expression, SelectableExpression, NonAggregate};
use query_builder::*;
use query_builder::returning_clause::*;
use query_source::Table;
use result::Error::QueryBuilderError;
use result::QueryResult;

/// The type returned by [`update`](/diesel/fn.update.html). The only thing you can do
/// with this type is call `set` on it.
#[derive(Debug)]
pub struct IncompleteUpdateStatement<T, U>(UpdateTarget<T, U>);

impl<T, U> IncompleteUpdateStatement<T, U> {
    #[doc(hidden)]
    pub fn new(t: UpdateTarget<T, U>) -> Self {
        IncompleteUpdateStatement(t)
    }
}

impl<T, U> IncompleteUpdateStatement<T, U> {
    pub fn set<V>(self, values: V) -> UpdateStatement<T, U, V::Changeset, NoReturningClause> where
        T: Table,
        V: changeset::AsChangeset<Target=T>,
        UpdateStatement<T, U, V::Changeset, NoReturningClause>: AsQuery,
    {
        UpdateStatement {
            table: self.0.table,
            where_clause: self.0.where_clause,
            values: values.as_changeset(),
            returning: NoReturningClause,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UpdateStatement<T, U, V, Ret = NoReturningClause> {
    table: T,
    where_clause: U,
    values: V,
    returning: Ret,
}

impl<T, U, V, Ret, DB> QueryFragment<DB> for UpdateStatement<T, U, V, Ret> where
    DB: Backend,
    T: Table,
    T::FromClause: QueryFragment<DB>,
    U: QueryFragment<DB>,
    V: changeset::Changeset<DB>,
    Ret: QueryFragment<DB>,
{
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        if self.values.is_noop() {
            return Err(QueryBuilderError(
                "There are no changes to save. This query cannot be built".into()
            ));
        }

        out.unsafe_to_cache_prepared();
        out.push_sql("UPDATE ");
        self.table.from_clause().walk_ast(out.reborrow())?;
        out.push_sql(" SET ");
        self.values.walk_ast(out.reborrow())?;
        self.where_clause.walk_ast(out.reborrow())?;
        self.returning.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl_query_id!(noop: UpdateStatement<T, U, V, Ret>);

impl<T, U, V> AsQuery for UpdateStatement<T, U, V, NoReturningClause> where
    T: Table,
    UpdateStatement<T, U, V, ReturningClause<T::AllColumns>>: Query,
{
    type SqlType = <Self::Query as Query>::SqlType;
    type Query = UpdateStatement<T, U, V, ReturningClause<T::AllColumns>>;

    fn as_query(self) -> Self::Query {
        self.returning(T::all_columns())
    }
}

impl<T, U, V, Ret> Query for UpdateStatement<T, U, V, ReturningClause<Ret>> where
    T: Table,
    Ret: Expression + SelectableExpression<T> + NonAggregate,
{
    type SqlType = Ret::SqlType;
}

impl<T, U, V> UpdateStatement<T, U, V, NoReturningClause> {
    /// Specify what expression is returned after execution of the `update`.
    /// # Examples
    ///
    /// ### Updating a single record:
    ///
    /// ```rust
    /// # #[macro_use] extern crate diesel;
    /// # include!("src/doctest_setup.rs");
    /// #
    /// # table! {
    /// #     users {
    /// #         id -> Integer,
    /// #         name -> VarChar,
    /// #     }
    /// # }
    /// #
    /// # #[cfg(feature = "postgres")]
    /// # fn main() {
    /// #     use self::users::dsl::*;
    /// #     let connection = establish_connection();
    /// let updated_name = diesel::update(users.filter(id.eq(1)))
    ///     .set(name.eq("Dean"))
    ///     .returning(name)
    ///     .get_result(&connection);
    /// assert_eq!(Ok("Dean".to_string()), updated_name);
    /// # }
    /// # #[cfg(not(feature = "postgres"))]
    /// # fn main() {}
    /// ```
    pub fn returning<E>(self, returns: E) -> UpdateStatement<T, U, V, ReturningClause<E>> where
        T: Table,
        UpdateStatement<T, U, V, ReturningClause<E>>: Query,
    {
        UpdateStatement {
            table: self.table,
            where_clause: self.where_clause,
            values: self.values,
            returning: ReturningClause(returns),
        }
    }
}
