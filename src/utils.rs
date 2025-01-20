use colored::Colorize;

pub fn banner() {
    println!("{}", r"                                   
__________                   _________.__           __   
\______   \_______   ____   /   _____/|__|  ____  _/  |_ 
 |     ___/\_  __ \ /  _ \  \_____  \ |  | /    \ \   __\
 |    |     |  | \/(  <_> ) /        \|  ||   |  \ |  |  
 |____|     |__|    \____/ /_______  /|__||___|  / |__|  
                                   \/          \/        
".blue());
}

pub fn copyright() {
    println!("{}","\nOpenSource Project\n".blue())
}