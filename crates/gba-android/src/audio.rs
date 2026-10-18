//! PCM resampling and desktop-style normal-speed fast-forward backlog.
use crate::normal_audio::NormalAudio;
use std::path::Path;
#[derive(Default)]
pub struct Audio {
    phase:u32, previous:[i16;2], key:(usize,u32),
    queue:Option<NormalAudio>, staged:Vec<i16>, cadence:u64,
}
impl Audio {
    pub fn begin(&mut self,speed:usize,mode:u32,directory:&Path)->Result<(),String>{
        let key=(speed,mode);
        if key!=self.key {
            self.queue=None;self.phase=0;self.cadence=0;self.previous=[0;2];self.key=key;
            if speed>1 && mode==2 {
                std::fs::create_dir_all(directory).map_err(|e|e.to_string())?;
                self.queue=Some(NormalAudio::new_in(directory).map_err(|e|e.to_string())?);
            }
        }
        self.staged.clear();
        Ok(())
    }
    pub fn feed(&mut self,pcm:&[i16]){
        let (speed,mode)=self.key;
        if mode==0{return;}
        let denominator=65536*if mode==1{speed as u32}else{1};
        for pair in pcm.chunks_exact(2){
            self.phase+=48000;
            if self.phase>=denominator {
                self.phase-=denominator;
                let fraction=(48000-self.phase) as i64;
                for channel in 0..2 {
                    self.staged.push((self.previous[channel] as i64+
                        (pair[channel] as i64-self.previous[channel] as i64)*fraction/48000) as i16);
                }
            }
            self.previous.copy_from_slice(pair);
        }
    }
    pub fn finish(&mut self,out:&mut Vec<i16>)->Result<(),String>{
        out.clear();
        if let Some(queue)=&mut self.queue {
            queue.push(&self.staged).map_err(|e|e.to_string())?;
            self.cadence+=48000*280896;
            let pairs=self.cadence/16777216;
            self.cadence%=16777216;
            for _ in 0..pairs {
                if let Some((left,right))=queue.next_pair().map_err(|e|e.to_string())? {
                    out.extend([left,right]);
                } else {break;}
            }
        }else{out.extend_from_slice(&self.staged);}
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accelerated_audio_has_one_host_frame_of_samples_at_every_speed(){
        for speed in [1,2,4,8] {
            let mut audio=Audio::default();let mut out=Vec::new();let mut total=0;
            for _ in 0..60{
                audio.begin(speed,1,&std::env::temp_dir()).unwrap();
                audio.feed(&vec![1000;1097*2*speed]);
                audio.finish(&mut out).unwrap();
                assert!(out.len()<=1610);total+=out.len();
                assert!(out.iter().all(|&sample|sample==1000 || sample==0));
            }
            assert!((total as i32-96416).abs()<10);
        }
    }
    #[test]
    fn normal_audio_keeps_pitch_and_resets_backlog_on_return_to_normal(){
        let mut audio=Audio::default();let mut out=Vec::new();
        audio.begin(8,2,&std::env::temp_dir()).unwrap();
        audio.feed(&vec![1234;1097*2*8]);audio.finish(&mut out).unwrap();
        assert!((1606..=1608).contains(&out.len()));
        assert_eq!(out[100],1234);
        audio.begin(1,2,&std::env::temp_dir()).unwrap();
        audio.feed(&vec![2345;1097*2]);audio.finish(&mut out).unwrap();
        assert_eq!(out[100],2345);
        audio.begin(8,0,&std::env::temp_dir()).unwrap();
        audio.feed(&vec![2345;1097*2*8]);audio.finish(&mut out).unwrap();
        assert!(out.is_empty());
    }
}
