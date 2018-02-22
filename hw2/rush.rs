use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io::stdin;
//use std::io;
//use std::io::{self, Write};
use std::io::{self,stdout};
use std::io::prelude::*;
use std::process;
use std::process::Command;
//use libc;
//use std::convert::AsRef;

/*pub enum Command {
	cd,
	exit,
	history,
	jobs,
	kill,
	pwd,
}*/


fn main(){
	
	let mut directory = env::current_dir().unwrap();
    let mut cur_directory = directory.to_str().unwrap().to_string();
    let mut history: Vec<String> = Vec::new();
	let mut n = 0;

	print!("$ ");
	io::stdout().flush().unwrap();
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
   		match line {
       		Ok(line) => {
       			//let line = lines.unwrap();
       			history.push(line.clone()); 
				n+=1;
				let args: Vec<&str> = line.split_whitespace().collect();
				let cmd = args[0];
   					
   				match cmd{
		
					"cd" => {
						let argument = args[1];
		    			cur_directory = cd_exec(&cur_directory, &argument);
		 	   			let path = Path::new(&cur_directory);
		 	    		env::set_current_dir(&path);
					},	
					"exit" => {
						process::exit(1);
					},
					"pwd" => println!("{}", cur_directory),

					"history" =>{
						for i in 0..n - 1{
						print!{"    {}  {}",i + 1,history[i]};
						io::stdout().flush().unwrap();
						}
					},
					"kill" => {
						//libc::kill(args[1], SIGKILL as libc::c_int).ok().expect("Error: Kill Failed");

					},
      		 		_ => {
      		 			let mut arg = args.clone();
      		 			arg.remove(0);
						//if let Some(first) = arg.first_mut() {
    					//	*first = " ";
						//}
      		 			//let the_process = 
      		 			/*match args.len(){
      		 			 1 => {process::Command::new(cmd)
       		 			                  .status()
       		 			                   .expect("failed to execute process");},
       		 			 2 => {process::Command::new(cmd)
       		 			                  .arg(args[1])
       		 			                  .status()
       		 			                  .expect("failed to execute process");},
       		 			 3 => {process::Command::new(cmd)
       		 			                  .arg(args[1])
       		 			                  .arg(args[2])
       		 			                  .status()
       		 			                   .expect("failed to execute process");},
       		 			 4 => {process::Command::new(cmd)
       		 			                  .arg(args[1])
       		 			                  .arg(args[2])
       		 			                  .arg(args[3])
       		 			                  .status()
       		 			                   .expect("failed to execute process");},
       		 			 5 => {process::Command::new(cmd)
       		 			                  .arg(args[1])
       		 			                  .arg(args[2])
       		 			                  .arg(args[3])
       		 			                  .arg(args[4])
       		 			                  .status()
       		 			                   .expect("failed to execute process");},
       		 			 _ => {process::Command::new(cmd)
       		 			                  .arg(args[1])
       		 			                  .arg(args[2])
       		 			                  .arg(args[3])
       		 			                  .arg(args[4])
       		 			                  .arg(args[5])
       		 			                  .status()
       		 			                   .expect("failed to execute process");},
       		 			 };*/
       		 			// the_process.join();

       		 			process::Command::new(cmd)
       		 							  .args(arg)
       		 			                  .status()
       		 			                   .expect("failed to execute process");
       		 			
      		 		},
					//	Command :: jobs => ,
					
				}
			},
        	Err(_) => break,
    	}
    	print!("$ ");
		io::stdout().flush().unwrap();
	}
}

fn cd_exec(directory: &str, arg: &str) -> String{
	let mut new_path = PathBuf::from(directory);

	if arg == ".." {
		return new_path.parent().unwrap().to_str().unwrap().to_string();
	}else if arg =="."{
		return directory.to_string();
	}else{
		new_path.push(arg);
		return new_path.to_str().unwrap().to_string();
	}
}