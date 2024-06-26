use super::*;

pub(crate) struct RecipeContext<'src: 'run, 'run> {
  pub(crate) config: &'run Config,
  pub(crate) dotenv: &'run BTreeMap<String, String>,
  pub(crate) module_source: &'run Path,
  pub(crate) scope: &'run Scope<'src, 'run>,
  pub(crate) search: &'run Search,
  pub(crate) settings: &'run Settings<'src>,
}
