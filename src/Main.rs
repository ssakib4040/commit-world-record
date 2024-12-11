use std::process::Command;

fn main() {
    // Define the number of commits you want to create
    let total_commits = 1_000_000;

    println!("Creating {} empty commits...", total_commits);

    for i in 1..=total_commits {
        // Execute git command to create an empty commit
        let output = Command::new("git")
            .args(["commit", "--allow-empty", "-m", &format!("Commit number {}", i)])
            .output();

        // Check for errors
        match output {
            Ok(_) => {
                if i % 1000 == 0 {
                    println!("{} commits created...", i);
                }
            }
            Err(e) => {
                eprintln!("Error creating commit number {}: {:?}", i, e);
                break;
            }
        }
    }

    println!("Completed {} empty commits.", total_commits);
}
