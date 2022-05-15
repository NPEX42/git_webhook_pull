use std::env;
use std::path::Path;
use std::io::{self, Write};
use std::time::*;
use chrono::prelude::*;
pub fn main() {
	use std::{thread};
	loop {
		fetch();
		thread::sleep(Duration::from_secs(10))
	}
}

fn fetch() {
    use git2::{Repository, Cred, RemoteCallbacks};
	let mut callbacks = RemoteCallbacks::new();
  callbacks.credentials(|_url, username_from_url, _allowed_types| {
    Cred::ssh_key(
      username_from_url.unwrap(),
      None,
      Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
      None,
    )
  });

	callbacks.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "[{:?}] Resolving deltas {}/{}\r",
		Utc::now(),
                stats.indexed_deltas(),
                stats.total_deltas(),
            );
        } else if stats.total_objects() > 0 {
            print!(
                "[{}] Received {}/{} objects ({}) in {} bytes\r",
		Utc::now(),
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes(),            
		);
        }
	println!();
        io::stdout().flush().unwrap();
	
        true
    });

   	let mut fetch_opts = git2::FetchOptions::new();
	fetch_opts.remote_callbacks(callbacks);

    match Repository::open("/home/site_user/website") {
        Ok(repo) => {

            match repo.find_remote("origin") {
                Ok(mut remote) => {
                  match remote.fetch(&["prod"], Some(&mut fetch_opts), None) {
		  	Ok(_) => println!("[{}] Fetched", Utc::now()),
			Err(e) => println!("[{}] Failed To Fetch: {}", Utc::now(), e)
		  }
                },
                Err(e) => {println!("[{}] Failed To Fetch Repo: {}", Utc::now(), e)}
            }
        },
        Err(e) => println!("[{}] Failed To Fetch Repo: {}", Utc::now(), e),
    }
}
