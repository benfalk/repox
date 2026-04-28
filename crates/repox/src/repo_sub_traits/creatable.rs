use crate::{Entity, Repo};

/// Marker trait bundling implemented type with entity
///
/// There are instances in which we cannot build an entity
/// because we are missing data from the repository.  Probably
/// the most common time we see this is with auto incrementing
/// IDs.  Whatever the case; this sets up a contract with a
/// repoistory stating the following:
///
/// ```rust
/// # use ::repox::{Creatable, Entity, Repo, CreateWith};
///
/// #[derive(Debug, Clone, Entity)]
/// struct Widget {
///     pub id: u32,
///     pub name: String,
/// }
///
/// #[derive(Debug, Clone)]
/// struct WidgetParams {
///     pub name: String,
/// }
///
/// // Bundles the WidgetParams with the Widget so the repository
/// // can process any number scenarios.
/// //               v============o
/// impl Creatable<Widget> for WidgetParams {}
///
/// // Requires the above implementation o==================v
/// pub trait WidgetBuilder: Repo + CreateWith<Widget, WidgetParams> {}
/// ```
///
/// WidgetBuilder will know have a [`create_with`] method that accepts
/// the `WidgetParams` and on success will return a `Widget`.
///
/// [`create_with`]: Repo::create_with
/// ---
pub trait Creatable<T: Entity>: Send + 'static {}

/// Trait contract for the Repo [`create_with`] method
///
/// A child trait marker for the repository.  This is the actual trait you'll
/// likely need to implement for your repository.  Currently this gives **all**
/// of the responsibility to the repo to construct the final entity from some
/// data structure.
///
/// NOTE: I am woefully aware this opens up the problem of code duplication
/// with other repositories that all have the same pattern.  I have an idea
/// to "enchance" the [`Creatable`] trait with other options to aid in this...
/// but I need to think on it a bit longer.
///
/// [`Creatable`]: Creatable
/// [`create_with`]: Repo::create_with
/// ---
pub trait CreateWith<T: Entity, P: Creatable<T>>: Repo {
    fn exec(
        &self,
        payload: P,
    ) -> impl Future<Output = Result<T, ::anyhow::Error>> + Send;
}
