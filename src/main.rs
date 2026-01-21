use minijinja::Environment;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read},
};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut env_file = None;

    if let Some(pos) = args.iter().position(|arg| arg == "--env-file" || arg == "-e") {
        if pos + 1 < args.len() {
            env_file = Some(args.remove(pos + 1));
            args.remove(pos);
        }
    }

    let template = if let Some(path) = args.first() {
        fs::read_to_string(path).expect("Failed to read template file")
    } else {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read template from stdin");
        input
    };

    let mut env = Environment::new();
    env.add_template("template", &template)
        .expect("Failed to add template");

    let tmpl = env
        .get_template("template")
        .expect("Failed to get template");

    let mut ctx = env::vars().collect::<HashMap<_, _>>();

    if let Some(path) = env_file {
        let content = fs::read_to_string(path).expect("Failed to read environment file");
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                ctx.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    let output = tmpl.render(&ctx).expect("Failed to render template");
    println!("{}", output);
}
