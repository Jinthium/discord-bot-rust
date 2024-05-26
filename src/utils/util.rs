pub struct Util {}

impl Util {
    pub fn get_env_var(var_name: &str) -> String {
        return std::env::var(var_name).expect(format!("Couldnt find environment variable: {var_name}").as_str())
    }
}