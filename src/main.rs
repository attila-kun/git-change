use git2::Repository;

fn main() {

    let repo = match Repository::open("/home/attila/dev/projects/TypeScript") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let reflog = match repo.reflog("HEAD") {
        Ok(reflog) => reflog,
        Err(e) => panic!("could not open reflog: {}", e)
    };

    println!("reflog length: {}", reflog.len());

    for entry in reflog.iter() {
        match entry.message() {
            Some(msg) => println!("entry {}", msg),
            None => ()
        }        
    }

    let branches = match(repo.branches(None)) {
        Ok(branches) => branches,
        Err(e) => panic!("could not open branches: {}", e)
    };

    for branch in branches {
        
        match branch {
            Ok((branch, branch_type)) => {
                let name = branch.name();

                match name {
                    Ok(Some(branch_name)) => println!("branch name: {}", branch_name),
                    Ok(None) => println!("could not read branch name"),
                    Err(e) => panic!("could not read branch name: {}", e)
                }
            },
            Err(e) => panic!("error while iterating through branches")
        }            
    }
}
