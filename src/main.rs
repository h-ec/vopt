use std::time::Duration;

use colored::Colorize;
use powershell_script::PsScriptBuilder;

fn main() 
{
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();
    let ras_output = ps.run(r#"rasdial "VPN" "vpn" "vpn""#).unwrap();

    let prompt_base_as_string: String = format!("{}{}{}", "[".bold().bright_black(), "VOPT".bright_magenta(), "]".bold().bright_black());
    let prompt_base: &str = prompt_base_as_string.as_str();
    println!("{} {}", prompt_base, "Connecting to \"VPN\" using \"rasdial\"");
    if ras_output.success() {
        println!("{} {}", prompt_base, "Connected to \"VPN\"");
        println!("{} {}", prompt_base, "Please consider changing the dns to \"8.8.8.8\" as a primary & \"8.8.4.4\" as an alternate");
        std::thread::sleep(Duration::from_secs(5));
        println!("{} {}", prompt_base, "Changing the priority of \"audiodg.exe\" to \"High (128)\"");
        let pri_output = ps.run(r#"Get-WmiObject Win32_process -filter 'name = "audiodg.exe"' | foreach-object { $_.SetPriority(128) }"#).unwrap();
        if pri_output.success() {
            println!("{} {}", prompt_base, "Changed the priority of \"audiodg.exe\" to \"High (128)\"");
            println!("{} {}", prompt_base, "Changing the affinity of \"audiodg.exe\" to \"4 (CPU2)\"");
            let aff_output = ps.run(r#"$Process = Get-Process audiodg; $Process.ProcessorAffinity=4"#).unwrap();
            if aff_output.success() {
                println!("{} {}", prompt_base, "Changed the affinity of \"audiodg.exe\" to \"4 (CPU2)\"");
            } else {
                println!("{} {}", prompt_base, "Failed to change the affinity of \"audiodg.exe\" to \"4 (CPU2)\", Consider changing it yourself from the \"Task Manager\"");
                std::thread::sleep(Duration::from_secs(5));
            }
        } else {
            println!("{} {}", prompt_base, "Failed to change the priority of \"audiodg.exe\", Please consider changing it yourself from the \"Task Manger\" => \"Details\" => \"audiodg.exe\" => \"RIGHT_CLICK_ON_AUDIODG.EXE\" => \"SET_PRIORITY\" => \"High\"");
            std::thread::sleep(Duration::from_secs(5));
        }
        println!("{} {}", prompt_base, "Adding routes, rerouting the vpn and masking it");
        let ipv4_output = ps.run(r#"((ipconfig | findstr IPv4)[1]).Split()[-1]"#).unwrap();
        let route_str: String = format!("route add 188.0.0.0 mask 255.0.0.0 {}", ipv4_output.stdout().unwrap().contains("."));
        let vpngv_output = ps.run(route_str.as_str()).unwrap();
        if vpngv_output.success() {
            println!("{} {}", prompt_base, "Added routes, rerouting the vpn and masking it");
            println!("{} {}", prompt_base, "Happy Gaming in \"VALORANT\" (:D)".bold().bright_green());
            std::thread::sleep(Duration::from_secs(2));
        } else {
            println!("{} {}", prompt_base, "Failed to add routes, rerouting the vpn and masking it, Please consider running \"VOPT\" in \"UAC (Administrator Mode)\"");
        }
    } else {
        println!("{} {}", prompt_base, "Failed to connect to \"VPN\"");
        println!("{} {}", prompt_base, "Make sure you followed the video tutorial which is in the \"README.md\"");
        std::thread::sleep(Duration::from_secs(5));
    }
}