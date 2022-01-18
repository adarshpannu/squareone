
pub struct StrSplit<'hs> {
    remainder: &'hs str,
    delim: &'hs str,
    eof: bool
}

impl<'hs> StrSplit<'hs> {
    fn new<'a>(haystack: &'a str, delim: &'a str) -> StrSplit<'a> {
        StrSplit { remainder: haystack, delim, eof: false }
    }

    fn next0<'a>(&'a mut self) -> Option<&'hs str> {
        if self.remainder.len() == 0 {
            if self.eof { 
                None
            } else {
                let retval = self.remainder;
                self.eof = true;
                Some(retval)
            }
        } else if let Some(ix) = self.remainder.find(&self.delim) {
            let retval = &self.remainder[..ix];
            self.remainder = &self.remainder[(ix+self.delim.len())..];
            Some(retval)
        } else {
            let retval = self.remainder;
            self.remainder = "";
            Some(retval)
        }
    }

}

impl<'hs> Iterator for StrSplit<'hs> {
    type Item = &'hs str;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.remainder.len() == 0 {
            if self.eof { 
                None
            } else {
                let retval = self.remainder;
                self.eof = true;
                Some(retval)
            }
        } else if let Some(ix) = self.remainder.find(&self.delim) {
            let retval = &self.remainder[..ix];
            self.remainder = &self.remainder[(ix+self.delim.len())..];
            Some(retval)
        } else {
            let retval = self.remainder;
            self.remainder = "";
            Some(retval)
        }
    }
}

#[test]
fn test() {
    let haystack = "a/";
    let delim = "/";

    //assert_eq!(StrSplit::new(haystack, " "), vec!["a", "b", "c"].into_iter())
    for w in StrSplit::new(haystack, " ") {
        dbg!(w);
    }

    let haystack = &"a b c".to_string()[..];
    let mut ss = StrSplit::new(haystack, " ");
    let part = ss.next0();
    drop(ss);
    dbg!(&part);
}

fn test2() {
    let s = "abc".to_string();
    let s2 = s.as_str();
    
}
