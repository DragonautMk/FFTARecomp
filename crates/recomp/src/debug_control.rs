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
    Cancel { request_id: String },
    Pause,
    Resume,
    Step {
        frames: u32,
        keys: u16,
    },
    AudioStart { settings: crate::debug_audio::Settings },
    AudioReport,
    CodeStart { settings: crate::debug_code::Settings },
    CodeReport,
    GameplayStart { settings: crate::debug_gameplay::Settings },
    GameplayReport,
    RecentStart { settings: crate::debug_recent::Settings },
    RecentFreeze,
    RecentReport,
    IssueReport {profile: Option<PathBuf>, symptom: String},
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
            Self::Cancel {request_id} if request_id.is_empty() || request_id.len()>100 => return Err("Cancel needs the exact pending request ID (1..100bytes)".into()),
            Self::Inspect { address, length }
                if *length == 0 || *length > 256 || address.checked_add(*length - 1).is_none() =>
            {
                return Err("Inspect needs1..256 bytes without wrap".into())
            }
            Self::Find { settings } => settings.validate()?,
            Self::AudioStart { settings } => settings.validate()?,
            Self::CodeStart { settings } => settings.validate()?,
            Self::GameplayStart { settings } => settings.validate()?,
            Self::RecentStart { settings } => settings.validate()?,
            Self::IssueReport{profile,symptom}=>{
                crate::debug_issue::validate_symptom(symptom)?;
                if profile.as_ref().is_some_and(|p|p.to_string_lossy().len()>1024){return Err("Profile path exceeds1024bytes".into());}
                if serde_json::to_vec(self).map_err(|e|e.to_string())?.len()>3500{return Err("Issue request exceeds mailbox size; shorten path or symptom".into());}
            },
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
            Action::Pause | Action::Cancel {..} => {
                self.remaining = 0;
                self.reason = if matches!(action,Action::Cancel{..}){"control-cancelled"}else{"pause-command"};
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
            Action::RecentStart{..} | Action::RecentFreeze | Action::RecentReport | Action::GameplayStart{..} | Action::GameplayReport | Action::IssueReport{..} | Action::CodeStart{..} | Action::CodeReport | Action::AudioStart{..} | Action::AudioReport | Action::UiStart | Action::UiReport | Action::Graphics { .. } | Action::Inspect { .. } | Action::Find { .. } => {
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
    #[serde(default)]
    deadline_unix_ms: Option<u64>,
    #[serde(default)]
    session_manifest_sha256: Option<String>,
    #[serde(skip)]
    query: bool,
}
fn unix_ms()->u64{SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis().min(u128::from(u64::MAX)) as u64}
impl Request {
    pub fn expired(&self)->bool{self.deadline_unix_ms.is_some_and(|d|unix_ms()>=d)}
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
    session: Value,
    pub pending: Option<Request>,
    outbox: Option<Value>,
    query_outbox: Option<Value>,
}
impl Server {
    pub fn new(root: &Path) -> Result<Self, String> {
        let manifest=crate::diagnostics::read(&root.join("debug-session.json"),1024*1024)?;
        let declared:Value=serde_json::from_slice(&manifest).map_err(|e|e.to_string())?;
        let session=json!({"manifest_sha256":gba_core::ffta_rom::digest(&manifest),
            "label":declared["label"].as_str().unwrap_or("").chars().take(128).collect::<String>()});
        let dir = directory(root);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let server=Self{dir,run:stamp(),session,pending:None,outbox:None,query_outbox:None};
        put(&server.dir,"ready.json",&server.ready(true))?;
        Ok(server)
    }
    fn ready(&self,active:bool)->Value{
        json!({"schema":1,"kind":"ffta-debug-control","run":self.run,"active":active,
            "pid":std::process::id(),"session":self.session,
            "capabilities":["query-status","targeted-cancel","request-deadline"]})
    }
    pub fn pending_status(&self)->Value{
        self.pending.as_ref().map_or(Value::Null,|r|json!({"request_id":r.id,"command":r.command,"deadline_unix_ms":r.deadline_unix_ms}))
    }
    pub fn pending_matches(&self,id:&str)->bool{self.pending.as_ref().is_some_and(|r|r.id==id)}
    pub fn deadline_expired(&self)->bool{self.pending.as_ref().is_some_and(Request::expired)}
    pub fn poll(&mut self)->Result<Option<Request>,String>{self.poll_channel(false)}
    pub fn poll_query(&mut self)->Result<Option<Request>,String>{self.poll_channel(true)}
    fn poll_channel(&mut self,query:bool)->Result<Option<Request>,String>{
        self.flush_channel(query)?;
        let path=self.dir.join(if query{"query.json"}else{"request.json"});
        if !path.exists(){return Ok(None);}
        // Each client publishes once by atomic rename. Consume before validation.
        let bytes=crate::diagnostics::read(&path,4096)?;
        fs::remove_file(&path).map_err(|e|e.to_string())?;
        let mut request:Request=serde_json::from_slice(&bytes).map_err(|e|e.to_string())?;
        request.query=query;
        if request.schema!=1 || request.run!=self.run || request.id.is_empty() || request.id.len()>100
            || request.session_manifest_sha256.as_ref().is_some_and(|h|Some(h.as_str())!=self.session["manifest_sha256"].as_str()){
            return Err("Stale, mismatched session or unsupported debug-control request".into());
        }
        if query && !matches!(request.command,Action::Status|Action::Cancel{..}){
            return Err("Query mailbox only accepts status or targeted cancel".into());
        }
        if !query && matches!(request.command,Action::Cancel{..}){
            return Err("Targeted cancel requires the query mailbox".into());
        }
        Ok(Some(request))
    }
    pub fn reply(&mut self,request:&Request,status:Value,mut error:Option<String>)->Result<(),String>{
        if request.expired() && error.is_none(){error=Some("Command completed after deadline; inspect returned state before retrying".into());}
        let outcome=if request.expired(){"timeout"}else if error.as_deref().is_some_and(|e|e.starts_with("Session closed")){"closed"}else if error.as_deref().is_some_and(|e|e.contains("control-cancelled")){"cancelled"}else if error.is_some(){"refused"}else{"ok"};
        let value=json!({"schema":1,"kind":"ffta-debug-control-result","run":self.run,"id":request.id,
            "session":self.session,"outcome":outcome,"ok":error.is_none(),"error":error,"status":status});
        if request.query{self.query_outbox=Some(value)}else{self.outbox=Some(value)}
        self.flush_channel(request.query)
    }
    fn flush_channel(&mut self,query:bool)->Result<(),String>{
        let outbox=if query{&mut self.query_outbox}else{&mut self.outbox};
        if let Some(value)=outbox{
            put(&self.dir,if query{"query-result.json"}else{"result.json"},value)?;
            *outbox=None;
        }Ok(())
    }
    pub fn inspection(&self,value:&Value)->Result<(),String>{put(&self.dir,"inspection.json",value)}
    pub fn finish(&mut self,status:Value,error:Option<String>)->Result<(),String>{
        if let Some(request)=self.pending.take(){self.reply(&request,status,error)?;}Ok(())
    }
}
impl Drop for Server {
    fn drop(&mut self){
        if let Some(request)=self.pending.take(){let _=self.reply(&request,Value::Null,Some("Session closed before command completion".into()));}
        for _ in 0..3{
            let primary=self.flush_channel(false);let query=self.flush_channel(true);
            if primary.is_ok()&&query.is_ok(){break;}
            std::thread::sleep(Duration::from_millis(2));
        }
        let _=put(&self.dir,"ready.json",&self.ready(false));
    }
}

fn parse(args: &[String]) -> Result<(PathBuf, Action), String> {
    if args.len() < 2 {
        return Err(
            "Use debug-control <status|cancel|pause|resume|step|inspect|break|watch|graphics|ui-start|ui-report|audio-start|audio-report|code-start|code-report|gameplay-start|gameplay-report|recent-start|recent-freeze|recent-report|issue-report> SESSION [options]"
                .into(),
        );
    }
    let hex = |s: &str| {
        u32::from_str_radix(s.trim_start_matches("0x"), 16)
            .map_err(|_| "Expected a hexadecimal address/value".to_owned())
    };
    let action = match args[0].as_str() {
        "cancel" if args.len()==4 && args[2]=="--request"=>Action::Cancel{request_id:args[3].clone()},
        "issue-report"=>{
            let mut profile=None;let mut symptom=String::new();let mut seen=std::collections::BTreeSet::new();let mut it=args[2..].iter();
            while let Some(key)=it.next(){if !seen.insert(key){return Err("Duplicate issue option".into());}let value=it.next().ok_or("Issue option needs a value")?;
                match key.as_str(){"--profile"=>profile=Some(PathBuf::from(value)),"--symptom"=>symptom=value.clone(),_=>return Err("Use --profile FILE / --symptom TEXT".into())}
            }Action::IssueReport{profile,symptom}
        },
        "recent-start"=>{
            let mut settings=crate::debug_recent::Settings::default();let mut seen=std::collections::BTreeSet::new();let mut it=args[2..].iter();
            let boolean=|v:&str|match v{"on"=>Ok(true),"off"=>Ok(false),_=>Err("Choose on/off")};
            while let Some(key)=it.next(){if !seen.insert(key){return Err("Duplicate recent option".into());}let value=it.next().ok_or("Recent option needs a value")?;
                match key.as_str(){"--events"=>settings.events=value.parse().map_err(|_|"Invalid event limit")?,
                    "--reads"=>settings.reads=boolean(value)?,"--on-error"=>settings.on_error=boolean(value)?,
                    "--start"=>settings.start=hex(value)?,"--end"=>settings.end=hex(value)?,_=>return Err("Unknown recent option".into())}
            }settings.validate()?;Action::RecentStart{settings}
        },
        "recent-freeze" if args.len()==2=>Action::RecentFreeze,
        "recent-report" if args.len()==2=>Action::RecentReport,
        "gameplay-start"=>{
            let mut settings=crate::debug_gameplay::Settings::default();let mut seen=std::collections::BTreeSet::new();let mut it=args[2..].iter();
            while let Some(key)=it.next(){if !seen.insert(key){return Err("Duplicate gameplay option".into());}let value=it.next().ok_or("Gameplay option needs a value")?;
                match key.as_str(){"--frames"=>settings.frames=value.parse().map_err(|_|"Invalid gameplay frame count")?,"--events"=>settings.events=value.parse().map_err(|_|"Invalid event limit")?,
                    "--family"=>settings.family=value.clone(),"--context"=>settings.context=Some(hex(value)?),_=>return Err("Unknown gameplay trace option".into())}
            }settings.validate()?;Action::GameplayStart{settings}
        },
        "gameplay-report" if args.len()==2=>Action::GameplayReport,
        "code-start"=>{
            let mut settings=crate::debug_code::Settings::default();let mut seen=std::collections::BTreeSet::new();let mut it=args[2..].iter();
            while let Some(key)=it.next(){if !seen.insert(key){return Err("Duplicate code trace option".into());}let value=it.next().ok_or("Code option needs a value")?;
                match key.as_str(){"--frames"=>settings.frames=value.parse().map_err(|_|"Invalid code frame count")?,"--events"=>settings.events=value.parse().map_err(|_|"Invalid event limit")?,
                    "--start"=>settings.start=hex(value)?,"--end"=>settings.end=hex(value)?,"--kind"=>settings.kind=value.clone(),
                    "--mode"=>settings.thumb=match value.as_str(){"arm"=>Some(false),"thumb"=>Some(true),"all"=>None,_=>return Err("Choose arm/thumb/all".into())},_=>return Err("Unknown code trace option".into())}
            }settings.validate()?;Action::CodeStart{settings}
        },
        "code-report" if args.len()==2=>Action::CodeReport,
        "audio-start"=>{
            let mut settings=crate::debug_audio::Settings::default();let mut i=2;let mut seen=std::collections::BTreeSet::new();
            while i<args.len(){let key=&args[i];if !seen.insert(key)||i+1>=args.len(){return Err("Duplicate/missing audio option".into());}
                match key.as_str(){"--frames"=>settings.frames=args[i+1].parse().map_err(|_|"Invalid audio frame count")?,"--streams"=>settings.streams=args[i+1].split(',').map(str::to_owned).collect(),_=>return Err("Use --frames N / --streams LIST".into())}i+=2;
            }settings.validate()?;Action::AudioStart{settings}
        },
        "audio-report" if args.len()==2=>Action::AudioReport,
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
            "Use debug-control <status|cancel|pause|resume|step|inspect|break|watch|graphics|ui-start|ui-report|audio-start|audio-report|code-start|code-report|gameplay-start|gameplay-report|recent-start|recent-freeze|recent-report|issue-report> SESSION [options]"
                .into(),
        ),
    };
    action.validate()?;
    Ok((PathBuf::from(&args[1]), action))
}
fn options(args:&[String])->Result<(Vec<String>,u64),String>{
    let mut filtered=Vec::new();let mut timeout=None;let mut it=args.iter();
    while let Some(arg)=it.next(){
        if arg=="--timeout-ms"{
            if timeout.is_some(){return Err("Duplicate --timeout-ms".into());}
            timeout=Some(it.next().ok_or("--timeout-ms needs a value")?.parse::<u64>().map_err(|_|"Invalid timeout")?);
        }else{filtered.push(arg.clone());}
    }
    let timeout=timeout.unwrap_or(120_000);
    if !(20..=120_000).contains(&timeout){return Err("Timeout needs20..120000milliseconds".into());}
    Ok((filtered,timeout))
}
fn client_lock(dir:&Path,query:bool)->Result<std::fs::File,String>{
    let lock=OpenOptions::new().create(true).truncate(false).read(true).write(true)
        .open(dir.join(if query{"query.lock"}else{"client.lock"})).map_err(|e|e.to_string())?;
    lock.try_lock().map_err(|_|if query{"Another status/cancel query is waiting"}else{"Another debug-control client is waiting for a result"})?;
    Ok(lock)
}
fn exchange(dir:&Path,ready:&Value,action:Action,query:bool,timeout:u64)->Result<Value,String>{
    let run=ready["run"].as_str().ok_or("Missing session run identity")?.to_owned();
    let request=Request{schema:1,run:run.clone(),id:stamp(),command:action,
        deadline_unix_ms:Some(unix_ms().saturating_add(timeout)),
        session_manifest_sha256:Some(ready["session"]["manifest_sha256"].as_str().ok_or("Missing session identity")?.into()),query};
    put(dir,if query{"query.json"}else{"request.json"},&serde_json::to_value(&request).map_err(|e|e.to_string())?)?;
    let start=Instant::now();
    loop{
        if let Ok(value)=read(&dir.join(if query{"query-result.json"}else{"result.json"})){
            if value["schema"]==1 && value["run"]==run && value["id"]==request.id && value["session"]==ready["session"]{
                return Ok(value);
            }
        }
        if start.elapsed()>=Duration::from_millis(timeout){
            return Ok(json!({"schema":1,"kind":"ffta-debug-control-client","run":run,"id":request.id,"session":ready["session"],
                "ok":false,"outcome":"timeout","error":"No acknowledgement before deadline; use status to inspect the exact request. Expiry stops pending work at the next frontend poll, not inside a guest frame or blocked callback.",
                "deadline_unix_ms":request.deadline_unix_ms,"server_acknowledged":false}));
        }
        if let Ok(value)=read(&dir.join("ready.json")){
            if value["run"]!=run || value["active"]!=true{
                return Ok(json!({"schema":1,"kind":"ffta-debug-control-client","run":run,"id":request.id,"session":ready["session"],
                    "ok":false,"outcome":"closed","error":"Session closed or reopened before acknowledgement"}));
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
fn execute(args:&[String])->Result<Value,String>{
    let (args,timeout)=options(args)?;
    let (root,action)=parse(&args)?;
    let manifest=crate::diagnostics::read(&root.join("debug-session.json"),1024*1024)
        .map_err(|_|"Choose an isolated debug-session folder")?;
    let dir=directory(&root);
    let query=matches!(action,Action::Status|Action::Cancel{..});
    let _lock=client_lock(&dir,query)?;
    let ready=read(&dir.join("ready.json"))?;
    if ready["schema"]!=1 || ready["kind"]!="ffta-debug-control" || ready["active"]!=true{
        return Err("No active debug-control session".into());
    }
    if ready["session"]["manifest_sha256"]!=gba_core::ffta_rom::digest(&manifest){
        return Err("Session identity mismatch or old channel; create a copy with the current runtime".into());
    }
    if ready["capabilities"].as_array().is_none_or(|c|!c.contains(&json!("targeted-cancel"))){
        return Err("This channel does not support query/cancel/deadline; use the current runtime".into());
    }
    exchange(&dir,&ready,action,query,timeout)
}
pub(crate) fn cmd(args:&[String])->Result<(),String>{
    match execute(args){
        Ok(value)=>{
            println!("{value}");
            if value["ok"]==true{Ok(())}else{Err(value["error"].as_str().unwrap_or("Control failed").into())}
        },
        Err(error)=>{
            println!("{}",json!({"schema":1,"kind":"ffta-debug-control-client","ok":false,"outcome":"refused","error":error}));
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn synthetic_server(root:&Path)->Server{
        fs::write(root.join("debug-session.json"),br#"{"label":"SYNTHETIC protocol only"}"#).unwrap();
        Server::new(root).unwrap()
    }
    fn synthetic_request(s:&Server,id:&str,command:Action,query:bool)->Request{
        Request{schema:1,run:s.run.clone(),id:id.into(),command,deadline_unix_ms:Some(unix_ms()+1000),
            session_manifest_sha256:Some(s.session["manifest_sha256"].as_str().unwrap().into()),query}
    }
    #[test]
    fn pending_command_queries_and_targeted_cancel_have_separate_locks_results_and_ids(){
        let t=tempfile::tempdir().unwrap();let mut s=synthetic_server(t.path());
        let command=synthetic_request(&s,"step1",Action::Step{frames:600,keys:0x3ff},false);
        put(&s.dir,"request.json",&serde_json::to_value(&command).unwrap()).unwrap();
        let mut c=Control::default();c.apply(&command.command,true,false).unwrap();
        s.pending=Some(s.poll().unwrap().unwrap());
        let primary=client_lock(&s.dir,false).unwrap();
        assert!(client_lock(&s.dir,false).is_err());
        let query_lock=client_lock(&s.dir,true).unwrap();assert!(client_lock(&s.dir,true).is_err());
        let query=synthetic_request(&s,"status1",Action::Status,true);
        put(&s.dir,"query.json",&serde_json::to_value(&query).unwrap()).unwrap();
        let got=s.poll_query().unwrap().unwrap();
        s.reply(&got,json!({"pending_control":s.pending_status()}),None).unwrap();
        assert_eq!(read(&s.dir.join("query-result.json")).unwrap()["status"]["pending_control"]["request_id"],"step1");
        assert!(s.pending_matches("step1")&&!s.pending_matches("old")&&!s.dir.join("result.json").exists());
        let cancel=synthetic_request(&s,"cancel1",Action::Cancel{request_id:"step1".into()},true);
        put(&s.dir,"query.json",&serde_json::to_value(&cancel).unwrap()).unwrap();
        let got=s.poll_query().unwrap().unwrap();c.apply(&got.command,true,false).unwrap();
        assert_eq!(c.reason,"control-cancelled");assert_eq!(c.remaining,0);
        s.finish(json!({"remaining":0}),Some("Step stopped: control-cancelled".into())).unwrap();
        s.reply(&got,json!({"pending_control":s.pending_status()}),None).unwrap();
        assert_eq!(read(&s.dir.join("result.json")).unwrap()["id"],"step1");
        assert_eq!(read(&s.dir.join("result.json")).unwrap()["outcome"],"cancelled");
        assert_eq!(read(&s.dir.join("query-result.json")).unwrap()["id"],"cancel1");
        drop((primary,query_lock));
    }
    #[test]
    fn protocol_consumes_stale_session_and_cross_channel_requests_without_execution(){
        let t=tempfile::tempdir().unwrap();let mut s=synthetic_server(t.path());
        for (field,value) in [("run",json!("old")),("schema",json!(2)),("session_manifest_sha256",json!("other"))]{
            let mut request=serde_json::to_value(synthetic_request(&s,"stale",Action::Pause,false)).unwrap();
            request[field]=value;put(&s.dir,"request.json",&request).unwrap();
            assert!(s.poll().is_err());assert!(!s.dir.join("request.json").exists());
        }
        let request=synthetic_request(&s,"wrong",Action::Step{frames:1,keys:0x3ff},true);
        put(&s.dir,"query.json",&serde_json::to_value(request).unwrap()).unwrap();assert!(s.poll_query().is_err());
        let request=synthetic_request(&s,"cancel",Action::Cancel{request_id:"id".into()},false);
        put(&s.dir,"request.json",&serde_json::to_value(request).unwrap()).unwrap();assert!(s.poll().is_err());
        let run=s.run.clone();drop(s);let s=Server::new(t.path()).unwrap();assert_ne!(s.run,run);
        assert_eq!(s.ready(true)["session"]["manifest_sha256"],gba_core::ffta_rom::digest(&fs::read(t.path().join("debug-session.json")).unwrap()));
        let mut s=s;s.pending=Some(synthetic_request(&s,"closing",Action::Step{frames:1,keys:0x3ff},false));
        let dir=s.dir.clone();drop(s);
        let closed=read(&dir.join("result.json")).unwrap();
        assert_eq!(closed["outcome"],"closed");assert_eq!(closed["id"],"closing");assert!(closed["status"].is_null());

    }
    #[test]
    fn deadlines_client_timeout_and_late_ack_are_structured_and_bounded(){
        let t=tempfile::tempdir().unwrap();let mut s=synthetic_server(t.path());let ready=s.ready(true);
        let value=exchange(&s.dir,&ready,Action::Step{frames:600,keys:0x3ff},false,20).unwrap();
        assert_eq!(value["outcome"],"timeout");assert_eq!(value["server_acknowledged"],false);
        let request=s.poll().unwrap().unwrap();assert!(request.expired());
        s.pending=Some(request.clone());assert!(s.deadline_expired());
        s.finish(json!({"frame":7}),Some("Step stopped: control-timeout".into())).unwrap();
        let result=read(&s.dir.join("result.json")).unwrap();
        assert_eq!(result["id"],value["id"]);assert_eq!(result["outcome"],"timeout");assert_eq!(result["ok"],false);
        // Late synchronous completion is also honest about a missed deadline.
        s.reply(&request,json!({"completed":true}),None).unwrap();
        assert_eq!(read(&s.dir.join("result.json")).unwrap()["ok"],false);
        let a=|v:&[&str]|v.iter().map(|s|s.to_string()).collect::<Vec<_>>();
        for v in [vec!["status","case","--timeout-ms","0"],vec!["status","case","--timeout-ms","120001"],vec!["status","case","--timeout-ms","20","--timeout-ms","30"]]{assert!(options(&a(&v)).is_err());}
        assert_eq!(options(&a(&["status","case","--timeout-ms","20"])).unwrap().1,20);
        assert!(parse(&a(&["cancel","case"])).is_err());
        assert!(matches!(parse(&a(&["cancel","case","--request","exact"])).unwrap().1,Action::Cancel{..}));
    }
    #[test]
    fn query_publication_failure_keeps_primary_ack_and_retries_same_query(){
        let t=tempfile::tempdir().unwrap();let mut s=synthetic_server(t.path());
        let q=synthetic_request(&s,"query",Action::Status,true);
        fs::create_dir(s.dir.join("query-result.json")).unwrap();
        assert!(s.reply(&q,json!({"v":1}),None).is_err());
        let p=synthetic_request(&s,"primary",Action::Pause,false);
        s.reply(&p,json!({"v":2}),None).unwrap();
        assert_eq!(read(&s.dir.join("result.json")).unwrap()["id"],"primary");
        fs::remove_dir(s.dir.join("query-result.json")).unwrap();
        assert!(s.poll_query().unwrap().is_none());
        assert_eq!(read(&s.dir.join("query-result.json")).unwrap()["id"],"query");
    }
    #[test]
    fn recent_parser_bounds_and_frontend_ownership(){
        let args=|v:&[&str]|v.iter().map(|s|s.to_string()).collect::<Vec<_>>();
        assert!(matches!(parse(&args(&["recent-start","case","--events","16","--reads","on","--start","00000000","--end","01000000","--on-error","off"])).unwrap().1,Action::RecentStart{..}));
        assert!(matches!(parse(&args(&["recent-freeze","case"])).unwrap().1,Action::RecentFreeze));
        assert!(matches!(parse(&args(&["recent-report","case"])).unwrap().1,Action::RecentReport));
        for v in [vec!["recent-start","case","--events","0"],vec!["recent-start","case","--events","1025"],vec!["recent-start","case","--reads","yes"],vec!["recent-start","case","--start","01000000","--end","0"],vec!["recent-start","case","--on-error","on","--on-error","off"]]{assert!(parse(&args(&v)).is_err());}
        assert!(Control::default().apply(&Action::RecentFreeze,true,false).is_err());
    }
    #[test]
    fn gameplay_parser_bounds_filters_and_paused_ownership() {
        let args=|a:&[&str]|a.iter().map(|s|s.to_string()).collect::<Vec<_>>();
        assert!(matches!(parse(&args(&["gameplay-start","case","--frames","2","--family","script","--context","02000000"])).unwrap().1,Action::GameplayStart{..}));
        assert!(matches!(parse(&args(&["gameplay-report","case"])).unwrap().1,Action::GameplayReport));
        for a in [vec!["gameplay-start","case","--frames","61"],vec!["gameplay-start","case","--events","513"],vec!["gameplay-start","case","--family","other"],vec!["gameplay-start","case","--context","fffffffff"],vec!["gameplay-start","case","--family","unit","--family","unit"]] {assert!(parse(&args(&a)).is_err());}
        assert!(Control::default().apply(&Action::GameplayReport,true,false).is_err());
    }
    #[test]
    fn audio_commands_validate_window_and_stream_selection() {
        let args=|v:&[&str]|v.iter().map(|s|s.to_string()).collect::<Vec<_>>();
        assert!(matches!(parse(&args(&["audio-start","case","--frames","2","--streams","mixed,hle"])).unwrap().1,Action::AudioStart{..}));
        assert!(matches!(parse(&args(&["audio-report","case"])).unwrap().1,Action::AudioReport));
        for v in [vec!["audio-start","case","--frames","181"],vec!["audio-start","case","--streams","mixed,mixed"],vec!["audio-start","case","--frames","2","--frames","3"]]{assert!(parse(&args(&v)).is_err());}
    }

    #[test]
    fn issue_command_bounds_and_selected_profile() {
        let a=|v:&[&str]|v.iter().map(|s|s.to_string()).collect::<Vec<_>>();
        assert!(matches!(parse(&a(&["issue-report","session","--profile","case/replay.json","--symptom","Observed issue"])).unwrap().1,Action::IssueReport{..}));
        assert!(parse(&a(&["issue-report","case","--profile","one","--profile","two"])).is_err());
        assert!(Action::IssueReport{profile:None,symptom:"x".repeat(2049)}.validate().is_err());
        assert!(Action::IssueReport{profile:Some(PathBuf::from("\\".repeat(1000))),symptom:"\n".repeat(1500)}.validate().is_err());
    }

    #[test]
    fn code_commands_validate_bounded_filters() {
        let a=|v:&[&str]|v.iter().map(|s|s.to_string()).collect::<Vec<_>>();
        assert!(matches!(parse(&a(&["code-start","case","--frames","2","--start","08000000","--end","09000000","--mode","thumb","--kind","missing"])).unwrap().1,Action::CodeStart{..}));
        assert!(matches!(parse(&a(&["code-report","case"])).unwrap().1,Action::CodeReport));
        for v in [vec!["code-start","case","--events","513"],vec!["code-start","case","--kind","unknown"],vec!["code-start","case","--frames","0"],vec!["code-start","case","--mode","arm","--mode","thumb"]]{assert!(parse(&a(&v)).is_err());}
    }

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
        fs::write(t.path().join("debug-session.json"),br#"{"label":"SYNTHETIC"}"#).unwrap();
        let mut s = Server::new(t.path()).unwrap();
        let request = Request {
            schema: 1,
            run: s.run.clone(),
            id: "one".into(),
            command: Action::Status,
            deadline_unix_ms:None,session_manifest_sha256:None,query:false,
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
        fs::write(t.path().join("debug-session.json"),br#"{"label":"SYNTHETIC"}"#).unwrap();
        let mut s = Server::new(t.path()).unwrap();
        let r = Request {
            schema: 1,
            run: s.run.clone(),
            id: "once".into(),
            command: Action::Pause,
            deadline_unix_ms:None,session_manifest_sha256:None,query:false,
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
