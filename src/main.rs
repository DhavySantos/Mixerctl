use std::process::Command;

fn main() {
    let volume = std::env::args().nth(1).unwrap();

    let pid_command = Command::new("sh").arg("-c").arg("xdotool getwindowfocus getwindowpid").output().unwrap();
    let pid_string = String::from_utf8_lossy(&pid_command.stdout); 
    
    let pid = pid_string.replace("\n", "");

    let sink_command = Command::new("sh").arg("-c").arg("pactl list sink-inputs").output().unwrap();
    let sink_string = String::from_utf8_lossy(&sink_command.stdout);
    

    let sinks: Vec<&str> = 
        sink_string.split("Sink Input #")
            .filter(|f| !f.is_empty())
            .collect();
    
    for sink in sinks {
        let value = format!("application.process.id = \"{pid}\"");
        if sink.contains(&value) {
            let index = sink.lines().nth(0).unwrap();
            let command = format!("pactl set-sink-input-volume {index} {volume}");
            Command::new("sh").arg("-c").arg(command).output().unwrap();
            break;
        };
    }
}
