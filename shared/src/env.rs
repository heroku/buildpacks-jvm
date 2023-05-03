use libcnb::layer::LayerData;
use libcnb::layer_env::Scope;
use libcnb::Env;

pub fn extend_build_env<T>(value: LayerData<T>, env: &mut Env) -> LayerData<T> {
    *env = value.env.apply(Scope::Build, env);
    value
}
