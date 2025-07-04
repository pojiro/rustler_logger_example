use std::sync::mpsc;
use std::sync::LazyLock;
use std::sync::RwLock;

type LogTuple = (rustler::Atom, String);
type Sender = std::sync::mpsc::Sender<LogTuple>;
type Receiver = std::sync::mpsc::Receiver<LogTuple>;

static SENDER: LazyLock<RwLock<Option<Sender>>> = LazyLock::new(|| RwLock::new(None));

mod atoms {
    rustler::atoms! {
        debug,
        info,
        warning,
        error
    }
}

#[rustler::nif]
fn logger_init(pid: rustler::LocalPid) -> rustler::NifResult<rustler::Atom> {
    let (tx, rx): (Sender, Receiver) = mpsc::channel();
    let mut sender = SENDER.write().unwrap();
    *sender = Some(tx);

    std::thread::spawn(move || {
        let owned_env = rustler::OwnedEnv::new();
        loop {
            if owned_env.run(|env| pid.is_alive(env)) {
                owned_env.run(|env: rustler::Env| match rx.recv() {
                    Ok(message) => {
                        let _ = env.send(&pid, message);
                    }
                    Err(error) => {
                        println!("{}", error)
                    }
                });
            } else {
                println!("logger process is dead");
                break;
            }
        }
    });

    Ok(rustler::types::atom::ok())
}

#[allow(dead_code)]
pub fn logger_debug<T: AsRef<str>>(message: T) {
    logger_impl(atoms::debug(), message.as_ref());
}

fn logger_impl(level: rustler::Atom, message: &str) {
    let sender = SENDER.read().unwrap();
    if let Some(tx) = sender.as_ref() {
        match tx.send((level, message.to_string())) {
            Ok(_) => {}
            Err(error) => println!("{}", error),
        }
    }
}
