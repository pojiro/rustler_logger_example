use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::LazyLock;
use std::sync::RwLock;

struct Resource {
    tx: Sender<String>,
}

impl Resource {
    fn new(tx: Sender<String>) -> Self {
        Resource { tx }
    }
}

static RESOURCE: LazyLock<RwLock<Option<Resource>>> = LazyLock::new(|| RwLock::new(None));

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    let _ = logger_debug("debug".to_string());
    a + b
}

#[rustler::nif]
fn init(pid: rustler::LocalPid) -> rustler::NifResult<rustler::Atom> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut resource = RESOURCE.write().unwrap();
    *resource = Some(Resource::new(tx));

    std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(message) => {
                rustler::OwnedEnv::new().run(|env: rustler::Env| {
                    let _ = env.send(&pid, message);
                });
            }
            Err(_error) => {}
        }
    });

    Ok(rustler::types::atom::ok())
}

fn logger_debug(message: String) -> rustler::NifResult<rustler::Atom> {
    let resource = RESOURCE.read().unwrap();
    if let Some(resource) = resource.as_ref() {
        resource
            .tx
            .send(message)
            .map_err(|error| rustler::Error::Term(Box::new(error.to_string())))?;
        Ok(rustler::types::atom::ok())
    } else {
        Ok(rustler::types::atom::ok())
    }
}

rustler::init!("Elixir.RustlerLoggerExample.Nif");
