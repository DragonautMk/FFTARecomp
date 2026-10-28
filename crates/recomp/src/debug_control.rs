//! Bounded frame stepping in an isolated interactive session; no guest fields.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub(crate) const MAX_STEP: u32 = 600;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "kebab-case", deny_unknown_fields)]
pub(crate) enum Action {
    Status,
    Pause,
    Resume,
    Step {
        frames: u32,
        keys: u16,
    },
    UiStart,
    UiReport,
    Graphics {
        compare_last: bool,
    },
    Inspect {
        address: u32,
        length: u32,
    },
    Find {
        settings: crate::debug_probe::Settings,
    },
}
impl Action {
    fn validate(&self) -> Result<(), String> {
        if let Self::Step { frames, keys } = self {
            if !(1..=MAX_STEP).contains(frames) || *keys > 0x3ff {
                return Err("Step needs 1..600 frames and a 000..3ff active-low key mask".into());
            }
        }
        match self {
            Self::Inspect { address, length }
                if *length == 0 || *length > 256 || address.checked_add(*length - 1).is_none() =>
            {
                return Err("Inspect needs1..256 bytes without wrap".into())
            }
            Self::Find { settings } => settings.validate()?,
            _ => {}
        }
        Ok(())
    }
}
pub(crate) struct Control {
    pub remaining: u32,
    pub keys: u16,
    pub reason: &'static str,
}
impl Default for Control {
    fn default() -> Self {
        Self {
            remaining: 0,
            keys: 0x3ff,
            reason: "running",
        }
    }
}
impl Control {
    pub fn apply(&mut self, action: &Action, paused: bool, busy: bool) -> Result<bool, String> {
        action.validate()?;
        if matches!(action, Action::Status) {
            return Ok(paused);
        }
        if busy {
            return Err("Finish or cancel the replay before controlling this session".into());
        }
        match action {
            Action::Pause => {
                self.remaining = 0;
                self.reason = "pause-command";
                Ok(true)
            }
            Action::Resume => {
                self.remaining = 0;
                self.reason = "running";
                Ok(false)
            }
            Action::Step { frames, keys } => {
                if !paused || self.remaining != 0 {
                    return Err("Pause first and wait for any pending step".into());
                }
                self.remaining = *frames;
                self.keys = *keys;
                self.reason = "stepping";
                Ok(true)
            }
            Action::UiStart | Action::UiReport | Action::Graphics { .. } | Action::Inspect { .. } | Action::Find { .. } => {
                Err("Inspection/search is owned by the isolated frontend".into())
            }
            Action::Status => unreachable!(),
        }
    }
    pub fn stop(&mut self, reason: &'static str) {
        self.remaining = 0;
        self.reason = reason;
    }
    pub fn completed_frame(&mut self, before: u64, after: u64) -> Result<(), String> {
        if self.remaining == 0 || after.checked_sub(before) != Some(1) {
            self.stop("incomplete-frame");
            return Err("Debug step did not complete exactly one guest frame".into());
        }
        self.remaining -= 1;
        if self.remaining == 0 {
            self.reason = "step-complete";
        }
        Ok(())
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Request {
    schema: u32,
    run: String,
    id: String,
    pub command: Action,
}
fn stamp() -> String {
    format!(
        "{}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        std::process::id()
    )
}
fn put(dir: &Path, name: &str, value: &Value) -> Result<(), String> {
    let mut file = tempfile::NamedTempFile::new_in(dir).map_err(|e| e.to_string())?;
    file.write_all(&serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    file.persist(dir.join(name))
        .map_err(|e| e.error.to_string())?;
    Ok(())
}
fn read(path: &Path) -> Result<Value, String> {
    serde_json::from_slice(&crate::diagnostics::read(path, 128 * 1024)?).map_err(|e| e.to_string())
}
fn directory(root: &Path) -> PathBuf {
    root.join("Data/config/debug-control")
}
pub(crate) struct Server {
    dir: PathBuf,
    run: String,
    pub pending: Option<Request>,
    outbox: Option<Value>,
}
impl Server {
    pub fn new(root: &Path) -> Result<Self, String> {
        let dir = directory(root);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let server = Self {
            dir,
            run: stamp(),
            pending: None,
            outbox: None,
        };
        put(
            &server.dir,
            "ready.json",
            &json!({"schema":1,"kind":"ffta-debug-control","run":server.run,"active":true,"pid":std::process::id()}),
        )?;
        Ok(server)
    }
    pub fn poll(&mut self) -> Result<Option<Request>, String> {
        self.flush()?;
        let path = self.dir.join("request.json");
        if !path.exists() {
            return Ok(None);
        }
        // The client publishes a complete file by atomic rename. Consume once.
        let bytes = crate::diagnostics::read(&path, 4096)?;
        fs::remove_file(&path).map_err(|e| e.to_string())?;
        let request: Request = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        if request.schema != 1 || request.run != self.run || request.id.len() > 100 {
            return Err("Stale or unsupported debug-control request".into());
        }
        Ok(Some(request))
    }
    pub fn reply(
        &mut self,
        request: &Request,
        status: Value,
        error: Option<String>,
    ) -> Result<(), String> {
        self.outbox = Some(
            json!({"schema":1,"run":self.run,"id":request.id,"ok":error.is_none(),"error":error,"status":status}),
        );
        self.flush()
    }
    fn flush(&mut self) -> Result<(), String> {
        if let Some(value) = &self.outbox {
            put(&self.dir, "result.json", value)?;
            self.outbox = None;
        }
        Ok(())
    }
    pub fn inspection(&self, value: &Value) -> Result<(), String> {
        put(&self.dir, "inspection.json", value)
    }
    pub fn finish(&mut self, status: Value, error: Option<String>) -> Result<(), String> {
        if let Some(request) = self.pending.take() {
            self.reply(&request, status, error)?;
        }
        Ok(())
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        if let Some(request) = self.pending.take() {
            let _ = self.reply(
                &request,
                Value::Null,
                Some("Session closed before step completion".into()),
            );
        }
        // A reader may briefly deny replacement on Windows. Retain exactly the
        // same terminal response while giving the client time to close its handle.
        for _ in 0..50 {
            if self.flush().is_ok() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let _ = put(
            &self.dir,
            "ready.json",
            &json!({"schema":1,"kind":"ffta-debug-control","run":self.run,"active":false}),
        );
    }
}

fn parse(args: &[String]) -> Result<(PathBuf, Action), String> {
    if args.len() < 2 {
        return Err(
            "Use debug-control <status|pause|resume|step|inspect|break|watch|graphics|ui-start|ui-report> SESSION [options]"
                .into(),
        );
    }
    let hex = |s: &str| {
        u32::from_str_radix(s.trim_start_matches("0x"), 16)
            .map_err(|_| "Expected a hexadecimal address/value".to_owned())
    };
    let action = match args[0].as_str() {
        "ui-start" if args.len()==2=>Action::UiStart,
        "ui-report" if args.len()==2=>Action::UiReport,
        "graphics" if args.len()==2 || (args.len()==3 && args[2]=="--compare-last")=>Action::Graphics{compare_last:args.len()==3},
        "inspect" if args.len() == 4 => Action::Inspect {
            address: hex(&args[2])?,
            length: args[3]
                .parse()
                .map_err(|_| "Length must be decimal1..256")?,
        },
        "break" if args.len() == 4 || args.len() == 6 => {
            let thumb = match args[3].as_str() {
                "arm" => false,
                "thumb" => true,
                _ => return Err("Choose arm or thumb".into()),
            };
            let steps = if args.len() == 6 && args[4] == "--steps" {
                args[5].parse().map_err(|_| "Invalid step limit")?
            } else if args.len() == 4 {
                100000
            } else {
                return Err("Use --steps N".into());
            };
            Action::Find {
                settings: crate::debug_probe::Settings {
                    condition: crate::debug_probe::Condition::Pc {
                        address: hex(&args[2])?,
                        thumb,
                    },
                    steps,
                },
            }
        }
        "watch" if args.len() >= 5 => {
            let write = match args[2].as_str() {
                "read" => false,
                "write" => true,
                _ => return Err("Choose read or write".into()),
            };
            let mut steps = 100000;
            let mut width = None;
            let mut value = None;
            let mut seen = std::collections::BTreeSet::new();
            let mut it = args[5..].iter();
            while let Some(flag) = it.next() {
                if !seen.insert(flag) {
                    return Err("Duplicate watch option".into());
                }
                let v = it.next().ok_or("Watch option needs value")?;
                match flag.as_str() {
                    "--steps" => steps = v.parse().map_err(|_| "Invalid step limit")?,
                    "--width" => width = Some(v.parse().map_err(|_| "Invalid width")?),
                    "--value" => value = Some(hex(v)?),
                    _ => return Err("Use --steps N --width1/2/4 --value HEX".into()),
                }
            }
            Action::Find {
                settings: crate::debug_probe::Settings {
                    condition: crate::debug_probe::Condition::Access {
                        watch: gba_core::debug_memory::Watch {
                            start: hex(&args[3])?,
                            end: hex(&args[4])?,
                            write,
                            width,
                            value,
                        },
                    },
                    steps,
                },
            }
        }
        "status" if args.len() == 2 => Action::Status,
        "pause" if args.len() == 2 => Action::Pause,
        "resume" if args.len() == 2 => Action::Resume,
        "step" if matches!(args.len(), 3 | 5) => Action::Step {
            frames: args[2].parse().map_err(|_| "N must be 1..600")?,
            keys: if args.len() == 5 && args[3] == "--keys" {
                u16::from_str_radix(args[4].trim_start_matches("0x"), 16)
                    .map_err(|_| "Invalid hexadecimal key mask")?
            } else if args.len() == 3 {
                0x3ff
            } else {
                return Err("Use --keys HEX".into());
            },
        },
        _ => return Err(
            "Use debug-control <status|pause|resume|step|inspect|break|watch|graphics|ui-start|ui-report> SESSION [options]"
                .into(),
        ),
    };
    action.validate()?;
    Ok((PathBuf::from(&args[1]), action))
}
pub(crate) fn cmd(args: &[String]) -> Result<(), String> {
    let (root, action) = parse(args)?;
    if !root.join("debug-session.json").is_file() {
        return Err("Choose an isolated debug-session folder".into());
    }
    let dir = directory(&root);
    // OS lock, automatically released on client death. One mailbox, one owner.
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(dir.join("client.lock"))
        .map_err(|e| e.to_string())?;
    lock.try_lock()
        .map_err(|_| "Another debug-control client is waiting for a result")?;
    let ready = read(&dir.join("ready.json"))?;
    if ready["schema"] != 1 || ready["kind"] != "ffta-debug-control" || ready["active"] != true {
        return Err("No active debug-control session".into());
    }
    let run = ready["run"]
        .as_str()
        .ok_or("Missing session run identity")?
        .to_owned();
    let request = Request {
        schema: 1,
        run: run.clone(),
        id: stamp(),
        command: action,
    };
    put(
        &dir,
        "request.json",
        &serde_json::to_value(&request).map_err(|e| e.to_string())?,
    )?;
    let start = Instant::now();
    loop {
        if let Ok(value) = read(&dir.join("result.json")) {
            if value["run"] == run && value["id"] == request.id {
                println!("{value}");
                return if value["ok"] == true {
                    Ok(())
                } else {
                    Err(value["error"].as_str().unwrap_or("Control failed").into())
                };
            }
        }
        if start.elapsed() > Duration::from_secs(120) {
            return Err("No acknowledgement within 120s; inspect the session before retrying. The command may still complete.".into());
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ui_commands_are_explicit_frontend_actions() {
        let args=|parts:&[&str]|parts.iter().map(|p|p.to_string()).collect::<Vec<_>>();
        assert!(matches!(parse(&args(&["ui-start","case"])).unwrap().1,Action::UiStart));
        assert!(matches!(parse(&args(&["ui-report","case"])).unwrap().1,Action::UiReport));
        assert!(parse(&args(&["ui-start","case","extra"])).is_err());
        assert!(Control::default().apply(&Action::UiReport,true,false).is_err());
    }
    #[test]
    fn exact_budget_and_invalid_commands_are_transactional() {
        let mut c = Control::default();
        assert!(c
            .apply(
                &Action::Step {
                    frames: 2,
                    keys: 0x3ff
                },
                false,
                false
            )
            .is_err());
        assert!(c
            .apply(
                &Action::Step {
                    frames: 0,
                    keys: 0x3ff
                },
                true,
                false
            )
            .is_err());
        assert!(c
            .apply(
                &Action::Step {
                    frames: 601,
                    keys: 0x3ff
                },
                true,
                false
            )
            .is_err());
        assert!(c
            .apply(
                &Action::Step {
                    frames: 1,
                    keys: 0x400
                },
                true,
                false
            )
            .is_err());
        assert!(c.apply(&Action::Resume, true, true).is_err());
        assert!(c
            .apply(
                &Action::Step {
                    frames: 2,
                    keys: 0x3fe
                },
                true,
                false
            )
            .unwrap());
        assert!(c
            .apply(
                &Action::Step {
                    frames: 1,
                    keys: 0x3ff
                },
                true,
                false
            )
            .is_err());
        c.completed_frame(42, 43).unwrap();
        assert_eq!(c.remaining, 1);
        c.completed_frame(43, 44).unwrap();
        assert_eq!(c.remaining, 0);
        assert_eq!(c.reason, "step-complete");
        assert!(!c.apply(&Action::Resume, true, false).unwrap());
        assert!(c.apply(&Action::Pause, false, false).unwrap());
        assert_eq!(c.remaining, 0);
    }
    #[test]
    fn guard_does_not_count_an_incomplete_or_multiple_frame() {
        for delta in [0, 2] {
            let mut c = Control::default();
            c.apply(
                &Action::Step {
                    frames: 2,
                    keys: 0x3ff,
                },
                true,
                false,
            )
            .unwrap();
            assert!(c.completed_frame(4, 4 + delta).is_err());
            assert_eq!(c.remaining, 0);
            assert_eq!(c.reason, "incomplete-frame");
        }
    }
    #[test]
    fn mailbox_is_run_bound_consumed_once_and_retained_in_constant_space() {
        let t = tempfile::tempdir().unwrap();
        let mut s = Server::new(t.path()).unwrap();
        let request = Request {
            schema: 1,
            run: s.run.clone(),
            id: "one".into(),
            command: Action::Status,
        };
        put(
            &s.dir,
            "request.json",
            &serde_json::to_value(&request).unwrap(),
        )
        .unwrap();
        let r = s.poll().unwrap().unwrap();
        assert!(s.poll().unwrap().is_none());
        s.reply(&r, json!({"frame":42}), None).unwrap();
        assert_eq!(read(&s.dir.join("result.json")).unwrap()["id"], "one");
        let mut stale = request;
        stale.run = "old".into();
        put(
            &s.dir,
            "request.json",
            &serde_json::to_value(stale).unwrap(),
        )
        .unwrap();
        assert!(s.poll().is_err());
        let dir = s.dir.clone();
        drop(s);
        assert_eq!(read(&dir.join("ready.json")).unwrap()["active"], false);
    }
    #[test]
    fn failed_response_publication_retries_only_the_same_result() {
        let t = tempfile::tempdir().unwrap();
        let mut s = Server::new(t.path()).unwrap();
        let r = Request {
            schema: 1,
            run: s.run.clone(),
            id: "once".into(),
            command: Action::Pause,
        };
        put(&s.dir, "request.json", &serde_json::to_value(&r).unwrap()).unwrap();
        let request = s.poll().unwrap().unwrap();
        // A directory at the destination deterministically blocks atomic replace.
        fs::create_dir(s.dir.join("result.json")).unwrap();
        assert!(s.reply(&request, json!({"counter":1}), None).is_err());
        assert!(s.outbox.is_some());
        assert!(!s.dir.join("request.json").exists());
        fs::remove_dir(s.dir.join("result.json")).unwrap();
        assert!(s.poll().unwrap().is_none());
        assert!(s.outbox.is_none());
        let result = read(&s.dir.join("result.json")).unwrap();
        assert_eq!(result["id"], "once");
        assert_eq!(result["status"]["counter"], 1);
    }

    #[test]
    fn diagnostic_parser_keeps_modes_ranges_values_and_bounds_explicit() {
        let a = |s: &str| s.split_whitespace().map(str::to_owned).collect::<Vec<_>>();
        assert!(matches!(parse(&a("graphics session --compare-last")).unwrap().1,Action::Graphics{compare_last:true}));
        assert!(parse(&a("graphics session --bad")).is_err());
        assert!(matches!(
            parse(&a("inspect session 08000000 256")).unwrap().1,
            Action::Inspect {
                address: 0x08000000,
                length: 256
            }
        ));
        assert!(matches!(
            parse(&a("break session 08000000 arm --steps 10"))
                .unwrap()
                .1,
            Action::Find { .. }
        ));
        assert!(matches!(
            parse(&a(
                "watch session write 02000000 02000004 --width 4 --value ff --steps 12"
            ))
            .unwrap()
            .1,
            Action::Find { .. }
        ));
        for text in [
            "inspect session 04000000 257",
            "break session 08000002 arm",
            "break session 08000000 bad",
            "break session 08000000 arm --steps 0",
            "watch session read 02000004 02000000",
            "watch session read 02000000 02000004 --width 3",
            "watch session read 02000000 02000004 --width 1 --value 100",
            "watch session read 02000000 02000004 --width 1 --width 2",
        ] {
            assert!(parse(&a(text)).is_err(), "{text}");
        }
    }

    #[test]
    fn parser_bounds_and_key_masks() {
        let args = |s: &str| s.split_whitespace().map(str::to_owned).collect::<Vec<_>>();
        assert!(matches!(
            parse(&args("step session 3 --keys 3fe")).unwrap().1,
            Action::Step {
                frames: 3,
                keys: 0x3fe
            }
        ));
        for s in [
            "step session 0",
            "step session 601",
            "step session 1 --keys 400",
            "resume session extra",
            "pause",
            "step session 1 --bad 3ff",
        ] {
            assert!(parse(&args(s)).is_err(), "{s}");
        }
    }
}
