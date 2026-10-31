//! Parse debugger commands without dispatching handlers or interpreting a shell.
use crate::debug_control::Action;

const MAX_BYTES: usize = 4096;
const MAX_TOKENS: usize = 64;
const MAX_ID_BYTES: usize = 512;

#[derive(Debug)]
pub(crate) enum Command {
    Help(Option<String>),
    Control(Action),
    Select { slot: menu::ContentSlot, id: String },
    JobsStatus,
}

/// Whitespace separates tokens; single/double quotes group text. Backslash
/// escapes only the active quote or another backslash inside quoted text.
/// There is deliberately no expansion, substitution, pipeline or shell.
pub(crate) fn tokenize(input:&str)->Result<Vec<String>,String> {
    if input.len()>MAX_BYTES {return Err("Command exceeds 4096 bytes".into());}
    if input.chars().any(|c|c.is_control() && c!='\t') {return Err("Command must be one line without control characters".into());}
    let mut tokens=Vec::new();let mut token=String::new();let mut quote=None;let mut started=false;
    let mut chars=input.chars().peekable();
    while let Some(ch)=chars.next() {
        if let Some(q)=quote {
            if ch==q {quote=None;} else if ch=='\\' {
                match chars.next() {Some(next) if next==q || next=='\\'=>token.push(next),_=>return Err("Only quoted quote/backslash escapes are supported".into())}
            } else {token.push(ch);}
        } else if ch=='\'' || ch=='"' {quote=Some(ch);started=true;}
        else if ch.is_whitespace() {
            if started {tokens.push(std::mem::take(&mut token));started=false;if tokens.len()>MAX_TOKENS{return Err("Too many command tokens".into());}}
        } else {token.push(ch);started=true;}
    }
    if quote.is_some(){return Err("Unterminated quote".into());}
    if started {tokens.push(token);}
    if tokens.len()>MAX_TOKENS{return Err("Too many command tokens".into());}
    if tokens.is_empty(){return Err("Empty command".into());}
    Ok(tokens)
}

fn id(value:&str,max:usize)->Result<String,String>{
    if value.is_empty() || value.len()>max || value.chars().any(char::is_control) {return Err("Invalid identifier".into());}
    Ok(value.to_owned())
}
fn decimal(v:&str)->Result<(),String>{
    if v.is_empty() || !v.bytes().all(|c|c.is_ascii_digit()) || (v.len()>1 && v.starts_with('0')) {return Err("Use unambiguous decimal digits without leading zeroes".into());}Ok(())
}
fn hex(v:&str)->Result<(),String>{
    let digits=v.strip_prefix("0x").ok_or("Use 0x-prefixed hexadecimal values")?;
    if digits.is_empty() || !digits.bytes().all(|c|c.is_ascii_hexdigit()){return Err("Invalid 0x-prefixed hexadecimal value".into());}Ok(())
}
fn numeric_syntax(t:&[String])->Result<(),String>{
    match t[0].as_str(){
        "step"=>{if let Some(v)=t.get(1){decimal(v)?;}},
        "inspect"=>{if let Some(v)=t.get(1){hex(v)?;}if let Some(v)=t.get(2){decimal(v)?;}},
        "break"=>{if let Some(v)=t.get(1){hex(v)?;}},
        "watch"=>{if let Some(v)=t.get(2){hex(v)?;}if let Some(v)=t.get(3){hex(v)?;}},
        _=>{}
    }
    for pair in t.windows(2){match pair[0].as_str(){
        "--keys"|"--start"|"--end"|"--context"|"--value"=>hex(&pair[1])?,
        "--steps"|"--width"|"--events"|"--frames"=>decimal(&pair[1])?,_=>{}
    }}
    Ok(())
}

/// Parse to existing typed operations. This function performs no filesystem,
/// menu, session or machine action; dispatch is a separate frontend boundary.
pub(crate) fn parse(input:&str)->Result<Command,String>{
    let t=tokenize(input)?;
    match t.as_slice(){
        [v] if v=="help"=>return Ok(Command::Help(None)),
        [v,name] if v=="help"=>{
            if crate::debug_operations::get(name).is_none(){return Err("Unknown operation ID".into());}
            return Ok(Command::Help(Some(name.clone())));
        },
        [family,verb,value] if verb=="select" && (family=="profile"||family=="catalog")=>{
            return Ok(Command::Select{slot:if family=="profile"{menu::ContentSlot::Replay}else{menu::ContentSlot::Catalog},id:id(value,MAX_ID_BYTES)?});
        },
        [family,verb] if family=="jobs" && verb=="status"=>return Ok(Command::JobsStatus),
        [family,verb,value] if family=="jobs" && verb=="cancel"=>{
            let args=vec!["cancel".into(),"<current-session>".into(),"--request".into(),id(value,100)?];
            return crate::debug_control::parse(&args).map(|(_,a)|Command::Control(a));
        },
        _=>{}
    }
    if t[0]=="issue-report" && t.iter().any(|v|v=="--profile") {return Err("Select a profile by ID first; paths are not accepted".into());}
    numeric_syntax(&t)?;
    let mut args=Vec::with_capacity(t.len()+1);args.push(t[0].clone());args.push("<current-session>".into());args.extend(t[1..].iter().cloned());
    crate::debug_control::parse(&args).map(|(_,a)|Command::Control(a))
}


/// CLI adapter uses the same typed local-control transport and reports unsupported
/// selection separately until ID selection is wired into the frontend.
pub(crate) fn cmd(args:&[String])->Result<(),String>{
    let result=(|| -> Result<serde_json::Value,String> {
        if args.len()!=2{return Err("Use debug-command SESSION 'COMMAND'".into());}
        match parse(&args[1])? {
            Command::Help(None)=>Ok(serde_json::json!({"schema":1,"kind":"ffta-debug-command-help","operations":crate::debug_operations::registry().operations})),
            Command::Help(Some(id))=>Ok(serde_json::json!({"schema":1,"kind":"ffta-debug-command-help","operation":crate::debug_operations::get(&id)})),
            Command::JobsStatus=>crate::debug_control::execute_action(std::path::Path::new(&args[0]),Action::Status,120_000),
            Command::Control(action)=>crate::debug_control::execute_action(std::path::Path::new(&args[0]),action,120_000),
            Command::Select{..}=>Err("Content selection by ID is not connected to the local channel yet".into()),
        }
    })();
    match result {
        Ok(value)=>{println!("{value}");if value.get("ok")==Some(&serde_json::json!(false)){Err(value["error"].as_str().unwrap_or("Command failed").into())}else{Ok(())}},
        Err(error)=>{println!("{}",serde_json::json!({"schema":1,"kind":"ffta-debug-command","ok":false,"outcome":"refused","error":error}));Err(error)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quotes_are_bounded_and_never_execute_or_expand() {
        assert_eq!(tokenize("issue-report --symptom \"é and \\\"quoted\\\"\"").unwrap(),vec!["issue-report","--symptom","é and \"quoted\""]);
        assert_eq!(tokenize("'$(not executed)' \\").unwrap(),vec!["$(not executed)","\\"]);
        for input in ["", "'unfinished", "one\ntwo", "\"bad\\n\""] {assert!(tokenize(input).is_err(),"{input:?}");}
        assert!(tokenize(&"x".repeat(MAX_BYTES+1)).is_err());assert!(tokenize(&"a ".repeat(MAX_TOKENS+1)).is_err());
    }
    #[test]
    fn shared_control_parser_and_typed_selection_are_unambiguous() {
        let cases=[("pause",vec!["pause","." ]),("step 1 --keys 0x3ff",vec!["step",".","1","--keys","0x3ff"]),("inspect 0x8000000 16",vec!["inspect",".","0x8000000","16"]),("break 0x8000000 thumb --steps 100",vec!["break",".","0x8000000","thumb","--steps","100"]),("watch read 0x2000000 0x2000010 --width 2",vec!["watch",".","read","0x2000000","0x2000010","--width","2"])];
        for (text,args) in cases {let Command::Control(a)=parse(text).unwrap() else{panic!("{text}")};let (_,b)=crate::debug_control::parse(&args.into_iter().map(str::to_owned).collect::<Vec<_>>()).unwrap();assert_eq!(serde_json::to_value(a).unwrap(),serde_json::to_value(b).unwrap());}
        let Command::Select{slot,id}=parse("profile select 'root:opaque id'").unwrap() else{panic!()};assert_eq!(slot,menu::ContentSlot::Replay);assert_eq!(id,"root:opaque id");
        assert!(matches!(parse("jobs status"),Ok(Command::JobsStatus)));assert!(matches!(parse("help control.step"),Ok(Command::Help(Some(_)))));
        for text in ["step 01","step 0","step 601","step 1 --keys 3ff","inspect 8000000 16","inspect 0x8000000 0","break 0x8000000 invalid","watch read 0x2000000 0x2000000","issue-report --profile /tmp/x","jobs cancel ''","profile select ''","unknown","help unknown"] {assert!(parse(text).is_err(),"{text}");}
    }
}
