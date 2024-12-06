extern crate git2;

use git2::{Repository, Signature};
use std::{env, process};

fn main() {
    // Get the current directory or provide a specific repo path
    let repo_path = env::current_dir().unwrap();

    // Open the repository
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Error opening repository: {}", e);
            process::exit(1);
        }
    };

    // Define the author and committer
    let signature = Signature::now("Sadman Sakib", "ssakib4040@gmail.com").unwrap();

    // Define the max number of commits
    let max_commits = 1_000_000;
    let mut count = 0;

    // Get HEAD reference to check if repo is empty or not
    let mut head = match repo.head() {
        Ok(head) => head,
        Err(_) => {
            eprintln!("Repository has no HEAD reference. Ensure there is at least one commit.");
            process::exit(1);
        }
    };

    // Create the initial commit if repository has no commits
    if head.shorthand().is_none() {
        println!("No commits found, creating initial commit.");
        let tree = match repo.find_tree(repo.head().unwrap().target().unwrap()) {
            Ok(tree) => tree,
            Err(_) => {
                eprintln!("Error finding tree.");
                process::exit(1);
            }
        };

        match repo.commit(
            Some("refs/heads/master"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        ) {
            Ok(_) => println!("Created initial commit."),
            Err(e) => {
                eprintln!("Error creating initial commit: {}", e);
                process::exit(1);
            }
        };
    }

    // Start creating 1 million empty commits
    while count < max_commits {
        // Get the current commit (parent commit)
        head = match repo.head() {
            Ok(head) => head,
            Err(_) => {
                eprintln!("Error finding HEAD reference.");
                process::exit(1);
            }
        };

        // Get the current commit object (parent)
        let parent_commit = match head.peel_to_commit() {
            Ok(commit) => commit,
            Err(_) => {
                eprintln!("Error retrieving parent commit.");
                process::exit(1);
            }
        };

        // Get the current tree from the parent commit
        let tree = match parent_commit.tree() {
            Ok(tree) => tree,
            Err(_) => {
                eprintln!("Error retrieving tree from parent commit.");
                process::exit(1);
            }
        };

        // Create an empty commit on top of the current commit
        match repo.commit(
            Some("refs/heads/master"), // Reference name
            &signature,                 // Author
            &signature,                 // Committer
            "Empty commit",             // Commit message
            &tree,                      // The tree object
            &[&parent_commit],          // Parent commit(s)
        ) {
            Ok(_) => {
                count += 1;
                if count % 100_000 == 0 {
                    println!("{} commits created.", count);
                }
            }
            Err(e) => {
                eprintln!("Error creating commit: {}", e);
                break;
            }
        }
    }

    println!("Finished creating {} commits!", count);
}
