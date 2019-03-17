use git2::Repository;

fn get_reflog() -> git2::Reflog {

    let repo = match Repository::open("/home/attila/dev/projects/TypeScript") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    match repo.reflog("HEAD") {
        Ok(reflog) => reflog,
        Err(e) => panic!("could not open reflog: {}", e)
    }
}

fn main() {

    let reflog = get_reflog();
    println!("reflog length: {}", reflog.len());

    for entry in reflog.iter() {
        match entry.message() {
            Some(msg) => println!("entry {}", msg),
            None => ()
        }        
    }
}
