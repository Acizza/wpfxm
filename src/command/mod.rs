pub mod add;
pub mod cfg;
pub mod clean;
pub mod exec;
pub mod hook;
pub mod ls;
pub mod new;
pub mod rm;
pub mod run;

use hashbrown::HashMap;

fn parse_env_var_arg<S>(arg: S) -> Option<(String, String)>
where
    S: AsRef<str>,
{
    let split = arg.as_ref().splitn(2, '=').collect::<Vec<_>>();

    if split.len() < 2 {
        return None;
    }

    let name = split[0].to_string();
    let value = split[1].to_string();

    Some((name, value))
}

fn parse_env_var_args(args: Option<Vec<String>>) -> HashMap<String, String> {
    let args = match args {
        Some(args) => args,
        None => return HashMap::new(),
    };

    let mut env_vars = HashMap::new();

    for arg in args {
        let (name, value) = try_opt_cont!(parse_env_var_arg(arg));
        env_vars.insert(name, value);
    }

    env_vars
}

fn parse_true_false_arg<S>(arg: S) -> bool
where
    S: AsRef<str>,
{
    let arg = arg.as_ref().to_ascii_lowercase();

    match arg.as_ref() {
        "true" | "1" => true,
        _ => false,
    }
}
