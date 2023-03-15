use axum::{
    routing::{get, post},
    extract::ConnectInfo,
    http::StatusCode,
    Form, Router,
    response::Html,
};
use askama::Template;

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use axum_sessions::{
    async_session::MemoryStore, extractors::{ReadableSession, WritableSession}, SessionLayer,
};
use anyhow::{Context, Result};
use tower_http::trace::{self, TraceLayer};
use tracing::{info, warn, error};

static CONFIGURATION: once_cell::sync::OnceCell<Configuration> = once_cell::sync::OnceCell::new();

struct Configuration {
    flag: String,
    passwords: Passwords, // Passwords are not shell escaped, so they should not contain single quotes
    sources: Sources,
    show_sources: bool,
    timeout_ms: u64,
}

#[derive(Serialize)]
struct Sources {
    sources: [String; 4]
}

struct Passwords {
    levels: std::collections::HashMap<Box<str>, u32>,
    problem_passwords: Vec<Box<str>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_ansi(false)
        .compact()
        .init();


    /* * * * * * * * * * */
    /* Settings / Config */
    /* * * * * * * * * * */

    // Load flag from compile time environment
    let flag = std::option_env!("FLAG")
        .unwrap_or("utflag{LGvb7PJXG5JDwhsEW7xp}").into();

    // TODO: configurable cookie secret and bind address
    let bind_addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    let secret = b"6^{\xba\xc32\xa5@8)\x8a^@\x07\xd7y\x99v\xab\xe7\xb4\x82\x97\xe8\x7f\xb1\xe0)\xe9\x12/\xcb\
        \x18<\x17)W\x16i\xeb\x92{\xdc\xb7\x15H \xdc\xdb\x02\xfc\xbf\x0bq\x00V\xeb\r)\x18s\xdd\x95\xaf\xa4\xf6\
        \xa1\x1eNp\x8fK\xdf\r\x9a\'\x9bXx\xd2\xaf\xf8k\'\x13oGc\xe7A\xa1#I}l\x86\x8a\"\x00,\x83\x92\xc2%)M\
        \xea\xbe\x13\xd1r\xf3\xa0\x87\xbb<\xdcSq\x0c{b;\xe4\xb6\xd1\xdc\xb0";

    let mut sources = Sources {
        sources: Default::default(),
    };
    let passwords = [
        "PuXqj7n4WNZzStnWbtPv",
        "Krdi9yQuY8mHoteZDCF5",
        "E46Dnqb5enAMgGArbruu",
        "5F4p7aLgQ5Nfn5YM8s68"
    ];
    let passwords = Passwords {
        problem_passwords: passwords.into_iter()
            .map(|s| s.into())
            .collect(),
        levels: passwords.into_iter()
            .map(|s| s.into())
            .zip(1..)
            .collect(),
    };

    for i in 0..4 {
        let path = format!("../problems/problem{}.py", i);
        let text = std::fs::read_to_string(&path)
            .with_context(|| anyhow::anyhow!("Error opening problem source {:?}", path))?;
        let text = text.lines().filter(|l| !l.starts_with("#!"))
            .fold(String::new(), |a, b| a + b + "\n");
        sources.sources[i] = text.trim().to_string();
    }

    let config = Configuration {
        passwords,
        sources,
        flag,
        show_sources: false,
        timeout_ms: 1000,
    };

    CONFIGURATION.set(config).ok().unwrap();


    /* * * * * * * * * * */
    /*  Initialization   */
    /* * * * * * * * * * */

    run_isolated_alt_setup().await
        .with_context(|| anyhow::anyhow!("Error setting up isolated runner"))?;

    let store = MemoryStore::new();
    let session_layer = SessionLayer::new(store, secret)
        .with_cookie_name("misc-calculator-session")
        .with_secure(false);

    let app = Router::new()
        .route("/", get(index))
        .route("/", post(post_index))
        .layer(session_layer)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .include_headers(false)
                    .level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .include_headers(false)
                    .level(tracing::Level::INFO))
        );

    tracing::info!("listening on {}", bind_addr);

    axum::Server::bind(&bind_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    level: u32,
    run_result: Option<RunResult<'a>>,
    notification: Option<Notification<'a>>,

    sources: &'a Sources,
    flag: &'a str,
    show_sources: bool,
}


#[derive(Serialize)]
struct RunResult<'a> {
    level: u32,
    output: &'a str,
}

#[derive(Serialize)]
struct Notification<'a> {
    success: bool,
    message: &'a str,
}


#[axum::debug_handler]
async fn index(
    session: ReadableSession,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    let level: u32 = session.get("level").unwrap_or(0);
    let config = CONFIGURATION.get().expect("Failed to load configuration");

    IndexTemplate {
        level,
        run_result: None,
        notification: None,
        sources: &config.sources,
        flag: &config.flag,
        show_sources: config.show_sources,
    }.render().map(Html)
        .with_context(|| format!("Failed to generate template"))
        .map_err(|e| index_error(e, level))
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum FormResults {
    #[serde(rename = "calculate")]
    Calculate {
        #[serde(rename = "level")]
        requested_level: String,
        expression: String,
    },
    #[serde(rename = "unlock")]
    Unlock {
        password: String,
    }
}

#[axum::debug_handler]
async fn post_index(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut session: WritableSession,
    Form(results): Form<FormResults>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    let config = CONFIGURATION.get().expect("Failed to load configuration");

    match results {
        FormResults::Calculate { requested_level, expression } => {
            let level: u32 = session.get("level").unwrap_or(0);

            let requested_level = requested_level.parse::<u32>()
                .with_context(|| format!("Error parsing requested level"))
                .map_err(|e| index_error(e, level))?;

            if requested_level > level {
                return Err(user_error("You must unlock a level before using it", level, StatusCode::FORBIDDEN))
            }

            let run_result = run_calculator(requested_level, expression, addr).await
                .with_context(|| format!("Calculation failed"))
                .map_err(|e| index_error(e, requested_level))?;

            IndexTemplate {
                level,
                run_result: Some(RunResult {
                    level: requested_level,
                    output: &run_result,
                }),
                notification: None,
                sources: &config.sources,
                flag: &config.flag,
                show_sources: config.show_sources,
            }.render().map(Html)
                .with_context(|| format!("Failed to generate template"))
                .map_err(|e| index_error(e, level))
        },
        FormResults::Unlock { password } => {
            let base_level: u32 = session.get("level").unwrap_or(0);

            let level = config.passwords.levels.get(&*password).copied();

            if let Some(level) = level {
                info!(level=level, "User entered correct password");

                if level > base_level {
                    session.insert("level", level)
                        .with_context(|| format!("Failed to set login cookie"))
                        .map_err(|e| index_error(e, level))?;

                    IndexTemplate {
                        level,
                        run_result: None,
                        notification: Some(Notification {
                            success: true,
                            message: &format!("Unlocked level {}", level),
                        }),
                        sources: &config.sources,
                        flag: &config.flag,
                        show_sources: config.show_sources,
                    }.render().map(Html)
                        .with_context(|| format!("Failed to generate template"))
                        .map_err(|e| index_error(e, level))
                } else {
                    IndexTemplate {
                        level,
                        run_result: None,
                        notification: Some(Notification {
                            success: true,
                            message: &format!("Level {} already unlocked", level),
                        }),
                        sources: &config.sources,
                        flag: &config.flag,
                        show_sources: config.show_sources,
                    }.render().map(Html)
                        .with_context(|| format!("Failed to generate template"))
                        .map_err(|e| index_error(e, level))
                }
            } else {
                IndexTemplate {
                    level: base_level,
                    run_result: None,
                    notification: Some(Notification {
                        success: false,
                        message: "Incorrect password",
                    }),
                    sources: &config.sources,
                    flag: &config.flag,
                    show_sources: config.show_sources,
                    }.render().map(Html)
                    .with_context(|| format!("Failed to generate template"))
                    .map_err(|e| index_error(e, base_level))
            }
        }
    }
}


fn index_error(
    err: anyhow::Error,
    level: u32,
) -> (StatusCode, Html<String>) {
    let config = CONFIGURATION.get().expect("Failed to load configuration");

    error!("Internal error: {:?}", err);

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(IndexTemplate {
            level,
            run_result: None,
            notification: Some(Notification {
                success: false,
                message: &format!("Internal error: {:?}", err),
            }),
            sources: &config.sources,
            flag: &config.flag,
            show_sources: config.show_sources,
        }.render()
            .unwrap_or_else(|_| format!("Something went wrong: {:?}", err))),
    );
}

fn user_error(
    text: &str,
    level: u32,
    code: StatusCode,
) -> (StatusCode, Html<String>) {
    let config = CONFIGURATION.get().expect("Failed to load configuration");

    return (
        code,
        Html(IndexTemplate {
            level,
            run_result: None,
            notification: Some(Notification {
                success: false,
                message: &text,
            }),
            sources: &config.sources,
            flag: &config.flag,
            show_sources: config.show_sources,
        }.render()
            .unwrap_or_else(|_| format!("Something went wrong: {:?}", text))),
    );
}


const LENGTH_LIMIT: u64 = 8192;

async fn run_calculator(mut requested_level: u32, expression: String, addr: SocketAddr) -> anyhow::Result<String> {
    use tokio::io::{AsyncWriteExt, AsyncReadExt, AsyncBufReadExt};

    info!(addr=debug(addr), requested_level=requested_level, expression=expression, "Started running command");

    let config = CONFIGURATION.get().expect("Failed to load configuration");
    let password = config.passwords.problem_passwords.get(requested_level as usize)
        .ok_or_else(|| anyhow::anyhow!("Failed to load password for level {}", requested_level))?;

    let source = config.sources.sources.get(requested_level as usize)
        .ok_or_else(|| anyhow::anyhow!("Failed to load source for level {}", requested_level))?;


    if requested_level >= 3 {
        requested_level = 3;
    }
    // let command = run_isolated(requested_level, format!("../problems/problem{}.py", requested_level), source, password).await?;
    let mut command = run_isolated_alt(requested_level, &format!("../problems/problem{}.py", requested_level), source, password).await
        .with_context(|| anyhow::anyhow!("Starting isolated runner failed"))?;

    let mut child = command.spawn()
        .with_context(|| anyhow::anyhow!("Starting isolated runner failed"))?;

    let mut stdin = child.stdin.take().ok_or_else(|| anyhow::anyhow!("Missing child stdin"))?;
    stdin.write_all(expression.as_bytes()).await
        .with_context(|| anyhow::anyhow!("Writing to stdin failed"))?;
    drop(stdin);

    let stdout = child.stdout.take().ok_or_else(|| anyhow::anyhow!("Missing child stdout"))?;
    let stderr = child.stderr.take().ok_or_else(|| anyhow::anyhow!("Missing child stderr"))?;

    let mut stdout_reader = tokio::io::BufReader::new(stdout.take(LENGTH_LIMIT)).lines();
    let mut stderr_reader = tokio::io::BufReader::new(stderr.take(LENGTH_LIMIT)).lines();

    let mut output = String::new();

    let future = async {
        loop {
            tokio::select! {
                result = child.wait() => {
                    match result {
                        Ok(_exit_code) => (),
                        _ => (),
                    }
                    break
                }
                result = stdout_reader.next_line() => {
                    match result {
                        Ok(Some(line)) => {
                            output += &line;
                            output += "\n";
                        }
                        Ok(None) => break,
                        Err(_) => break,
                    }
                }
                result = stderr_reader.next_line() => {
                    match result {
                        Ok(Some(line)) => {
                            output += &line;
                            output += "\n";
                        }
                        Ok(None) => break,
                        Err(_) => break,
                    }
                }
            };
        }
    };

    if let Err(_) = tokio::time::timeout(std::time::Duration::from_millis(config.timeout_ms), future).await {
        info!("Command timed out, killing child.");
        if let Err(e) = child.kill().await {
            error!("Failed to kill child process: {:?}", e);
        }
    }

    info!(addr=debug(addr), requested_level=requested_level, expression=expression, "Finished running command");

    Ok(output)
}


// This needs to run a python file in a semi-isolated manner;
// it needs to create a private file 'password.txt' containing the
// password before running the script.
// It also must ensure that when the runner is killed, the python
// script is also killed.

// If the password.txt file isn't private, there is a race condition
// in the startup of problems 2 and 3 that would allow people to
// read the file through brute force attempts rather than actually
// solving the problem.

// Additionally, scripts must be prevented from reading the code, binary
// or memory of the webserver.

// One theoretical solution to some of the issues is by creating a
// user account for each of the 4 problems and switching to those users
// before running each; this works well for 0 and 1, but leaves
// the race condition for 2 and 3.

// Adding infinite users partially solves that, but is much more problematic
// to implement.

async fn run_isolated(
    _problem_number: u32,
    problem_file: &str,
    _problem_source: &str,
    password: &str,
) -> anyhow::Result<tokio::process::Command> {

    // bwrap --ro-bind /usr /usr --symlink usr/lib64 /lib64 --proc /proc --dev /dev --unshare-all --die-with-parent bash
    // python3 
    // let mut child = tokio::process::Command::new("python3")
    //     .args([format!("../problems/problem{}.py", requested_level)])
    // let mut child = tokio::process::Command::new("bwrap")
    //     .args(["--ro-bind", "/usr", "/usr", "--symlink", "usr/lib64", "/lib64", "--proc", "/proc", "--dev", "/dev", "--unshare-all", "--die-with-parent",
    //         // "--ro-bind", &format!("../problems/problem{}.py", requested_level), "/proc/self/net/password.txt",
    //         // "--symlink", "/proc/self/net/password.txt", "/password.txt",
    //         // "--ro-bind", &format!("../problems/password{}.txt", requested_level), "/password.txt",
    //         "--ro-bind", &format!("../problems/password{}.txt", requested_level), "/password.txt",
    //         "--ro-bind", &format!("../problems/problem{}.py", requested_level), "/problem.py",
    //         "python3", "problem.py"])

    // --clearenv is probably better than env -i, but it isn't supported on ubuntu 20.04's bwrap
    let mut command = tokio::process::Command::new("bash");
    command.args([
            "-c", &(String::new() +
                "env -i bwrap --ro-bind /usr /usr --symlink usr/lib64 /lib64 --proc /proc --dev /dev --unshare-all --die-with-parent " +
                "--file 53 /password.txt " + 
                &format!("--ro-bind '{}' /problem.py ", problem_file) +
                "-- python3 problem.py " +
                &format!("53< <(echo -n '{}') ", password)
            )
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true);

    Ok(command)
}

async fn run_isolated_alt_setup() -> anyhow::Result<()> {
    for i in 0..4 {
        let dir = format!("/tmp/working/{}", i);
        tokio::fs::create_dir_all(&dir).await
            .with_context(|| anyhow::anyhow!("Error creating directory chain {:?}", dir))?;

        let source = format!("../problems/problem{}.py", i);
        let problem_file = format!("{}/problem.py", dir);
        tokio::fs::copy(&source, &problem_file).await
            .with_context(|| anyhow::anyhow!("Error copying problem file from {:?} to {:?}", source, problem_file))?;

        tokio::process::Command::new("sh")
            .args([
                "-c", &format!(r#" chmod a-rwx,ug+rwx,+t '{dir}'; chmod a-rwx,u+rw,g+r '{problem_file}'; chown server:p{i} '{dir}' '{problem_file}' "#)
            ])
            .status().await
            .with_context(|| anyhow::anyhow!("Error changing permissions"))?;
    }
    Ok(())
}

async fn run_isolated_alt(
    problem_number: u32,
    _problem_file: &str,
    _problem_source: &str,
    password: &str,
) -> anyhow::Result<tokio::process::Command> {
    use tokio::io::AsyncWriteExt;

    let dir = format!("/tmp/working/{}", problem_number);
    tokio::fs::create_dir_all(&dir).await
        .with_context(|| anyhow::anyhow!("Error creating directory chain {:?}", dir))?;

    // let password_path = format!("{}/password.txt", dir);
    // let mut file = tokio::fs::OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .truncate(true)
    //     .mode(0o640)
    //     .open(&password_path)
    //     .await
    //     .with_context(|| anyhow::anyhow!("Error creating password file {:?}", password_path))?;

    let mut create = tokio::process::Command::new("sudo")
        .args([
            "-T", "1", "-u", &format!("p{}", problem_number), "sh", "-c", r#"rm -rf password.txt; cat > password.txt"#
        ])
        .current_dir(&dir)
        .stdin(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .with_context(|| anyhow::anyhow!("Error changing permissions"))?;

    let mut stdin = create.stdin.take().ok_or_else(|| anyhow::anyhow!("Missing child stdin"))?;
    stdin.write_all(password.as_bytes()).await
        .with_context(|| anyhow::anyhow!("Writing to stdin failed"))?;
    drop(stdin);

    create.wait().await
        .with_context(|| anyhow::anyhow!("Error creating password: Waiting failed"))?;

    // file.write_all(password.as_bytes()).await
    //     .with_context(|| anyhow::anyhow!("Error writing password file"))?;

    // drop(file);

    let mut command = tokio::process::Command::new("sudo");
    command.args([
            "-u", &format!("p{}", problem_number), "python3", "-I", "problem.py"
        ])
        .current_dir(&dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true);

    Ok(command)
}
