use std::cell::RefCell;
use std::cmp;
use std::io;
use std::rc::Rc;
use std::str::FromStr;

struct Directory {
    name: String,
    entries: Vec<ListEntry>,
}

impl Directory {
    fn total_size(&self) -> usize {
        let mut total = 0;
        for e in &self.entries {
            match e {
                ListEntry::File(f) => total += f.size,
                ListEntry::Dir(d) => total += d.borrow().total_size(),
            }
        }
        dbg!(&self.name);
        dbg!(total);
        total
    }

    fn total_size_filt(&self, lim: usize) -> usize {
        let mut total = 0;
        for e in &self.entries {
            if let ListEntry::Dir(d) = e {
                let db = d.borrow();
                let size = db.total_size();
                if size < lim {
                    total += size;
                }
                total += db.total_size_filt(lim);
            }
        }
        total
    }

    fn smallest_larger(&self, lim: usize) -> Option<usize> {
        let mut min = None;
        for e in &self.entries {
            if let ListEntry::Dir(d) = e {
                let db = d.borrow();
                if let Some(s) = db.smallest_larger(lim) {
                    match min {
                        None => min = Some(s),
                        Some(m) => min = Some(cmp::min(m, s)),
                    }
                }
                let s = db.total_size();
                if s > lim {
                    match min {
                        None => min = Some(s),
                        Some(m) => min = Some(cmp::min(m, s)),
                    }
                }
            }
        }
        min
    }
}

struct File {
    name: String,
    size: usize,
}

enum ListEntry {
    Dir(Rc<RefCell<Directory>>),
    File(File),
}

impl FromStr for ListEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_once(" ").ok_or(())?;
        if s1 == "dir" {
            Ok(ListEntry::Dir(Rc::new(RefCell::new(Directory {
                name: String::from(s2),
                entries: Vec::new(),
            }))))
        } else {
            let size: usize = s1.parse().map_err(|_| ())?;
            Ok(ListEntry::File(File {
                name: String::from(s2),
                size: size,
            }))
        }
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Directory {
        name: String::from("/"),
        entries: Vec::new(),
    }));
    let mut pwd = root.clone();
    let mut parents = Vec::new();
    let lines = io::stdin().lines();
    for it in lines {
        let l = it.unwrap();
        if let Some(r) = l.trim().strip_prefix("$ ") {
            if let Some(dir) = r.strip_prefix("cd ") {
                if dir == ".." {
                    pwd = parents.pop().unwrap();
                } else if dir == "/" {
                    parents.clear();
                    pwd = root.clone();
                } else {
                    let mut newpwd = None;
                    for e in &pwd.borrow().entries {
                        if let ListEntry::Dir(d) = e {
                            if d.borrow().name == dir {
                                parents.push(pwd.clone());
                                newpwd = Some(d.clone());
                                break;
                            }
                        }
                    }
                    pwd = newpwd.expect("changed into directory {dir} that wasn't in listing");
                }
            }
            // ignore ls and assume everything without a $ is part of a listing
        } else {
            let e = l.trim().parse().unwrap();
            pwd.borrow_mut().entries.push(e);
        }
    }
    let total = root.borrow().total_size();
    let unused = 70000000 - total;
    let to_be_freed = 30000000 - unused;
    let smallest = root.borrow().smallest_larger(to_be_freed).unwrap();
    println!("{smallest}");
}
