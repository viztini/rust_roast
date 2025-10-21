use sysinfo::{CpuExt, System, SystemExt};
use colored::*;
use rand::seq::SliceRandom;
use std::process::Command;

// Function to get GPU name (Linux specific, attempts lspci parsing)
fn get_gpu_name() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci -v | grep -i vga | grep -i 'controller|display'")
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().next() {
                // Attempt to extract a more readable name
                if let Some(start) = line.find(": ") {
                    let name_part = &line[start + 2..];
                    if let Some(end) = name_part.find(" (") {
                        return name_part[..end].trim().to_string();
                    }
                    return name_part.trim().to_string();
                }
                return line.trim().to_string();
            }
            "Unknown GPU (lspci failed)".to_string()
        }
        Err(_) => "Unknown GPU (lspci command error)".to_string(),
    }
}

// Function to detect if it's a laptop (checks for battery)
fn is_laptop() -> bool {
    // This is a common path for battery information on Linux
    std::path::Path::new("/sys/class/power_supply/BAT0").exists() ||
    std::path::Path::new("/sys/class/power_supply/BAT1").exists() // Check BAT0 or BAT1
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "\n--- System Specs ---".cyan().bold());

    // --- CPU Info ---
    let cpu_count = sys.cpus().len();
    let cpu_brand = sys.cpus()[0].brand();
    let cpu_frequency = sys.cpus()[0].frequency(); // in MHz
    println!("  {}: {} ({} cores @ {} MHz)", "CPU".green().bold(), cpu_brand.white(), cpu_count.to_string().white(), cpu_frequency.to_string().white());

    // --- RAM Info ---
    let total_ram_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_ram_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    println!("  {}: {{:.2}} GB total, {{:.2}} GB used", "RAM".green().bold(), total_ram_gb.to_string().white(), used_ram_gb.to_string().white());

    // --- GPU Info ---
    let gpu_name = get_gpu_name();
    println!("  {}: {}", "GPU".green().bold(), gpu_name.white());

    // --- OS Info ---
    let os_name = sys.name().unwrap_or_else(|| "Unknown OS".to_string());
    let os_version = sys.os_version().unwrap_or_else(|| "Unknown Version".to_string());
    println!("  {}: {} {}", "OS".green().bold(), os_name.white(), os_version.white());

    // --- Form Factor ---
    let form_factor = if is_laptop() { "Laptop" } else { "Desktop" };
    println!("  {}: {}", "Form Factor".green().bold(), form_factor.white());

    println!("{}", "\n--- The Roast ---".red().bold());

    let mut roasts = Vec::new();
    let mut rng = rand::thread_rng();

    // --- CPU Roasts ---
    let cpu_roasts_low = vec![
        "Your CPU is so slow, it thinks \"loading screen\" is a feature, not a bug.",
        "Did you find your CPU in a cereal box?",
        "Your processor is still running on dial-up speed.",
        "Is your CPU powered by a hamster on a wheel?",
        "I've seen calculators with more processing power.",
        "Your CPU is so old, it remembers when \"megahertz\" was impressive.",
        "Does your CPU come with a built-in coffee break every time you open a tab?",
        "It's not a CPU, it's a paperweight that occasionally computes.",
        "Your CPU is the reason \"patience is a virtue\" was invented.",
        "I bet your CPU still thinks Windows XP is cutting edge.",
        "Your CPU is so bad, it makes a snail look like a cheetah.",
        "Are you sure that's a CPU and not a potato with wires?",
        "Your CPU's performance is a cry for help.",
        "It's not throttling, it's just taking a very long nap.",
        "Your CPU is the digital equivalent of waiting for paint to dry.",
        "I heard your CPU is still trying to render the first frame of Pong.",
        "Does your CPU need a nap after opening Notepad?",
        "Your CPU is so outdated, it probably runs on steam.",
        "The only thing fast about your CPU is how quickly it disappoints.",
        "Your CPU is a master of procrastination.",
        "It's not a bug, it's a feature... of your slow CPU.",
        "Your CPU is the reason we have progress bars.",
        "I've seen faster calculations on an abacus.",
        "Your CPU is a monument to \"almost there\".",
        "The only thing your CPU is good at is generating heat.",
    ];
    let cpu_roasts_mid = vec![
        "Your CPU is trying its best, bless its little silicon heart. Almost there!",
        "It's not the fastest, but at least it's not actively trying to sabotage you.",
        "Your CPU is the definition of \"gets the job done... eventually.\"",
        "A solid B- for effort, your CPU.",
        "Your CPU is like a reliable old car: it runs, but don't ask it to win any races.",
        "It's not a powerhouse, but it's not a complete embarrassment either.",
        "Your CPU is the middle child of processors: always overlooked.",
        "It's got enough cores to count your fingers, and maybe a few toes.",
        "Your CPU is the reason \"good enough\" exists.",
        "It's not breaking any records, but it's not breaking your bank either.",
        "Your CPU is the beige of computing: functional, but unexciting.",
        "It's got enough power for spreadsheets and existential dread.",
        "Your CPU is the equivalent of a participation trophy.",
        "It's not fast, it's not slow, it's just... there.",
        "Your CPU is the background music of your computing life: always present, rarely noticed.",
        "It's got enough oomph for basic tasks and questioning your life choices.",
        "Your CPU is the definition of \"average.\"",
        "It's not a beast, but it's not a total slouch either.",
        "Your CPU is the reason you have time to make a sandwich while waiting.",
        "It's got enough power to run your OS and a mild sense of regret.",
        "Your CPU is the unsung hero of \"just barely.\"",
        "It's not a Ferrari, but it's not a tricycle either.",
        "Your CPU is the definition of \"meh.\"",
        "It's got enough power to run your favorite retro games.",
        "Your CPU is the reason you appreciate fast computers.",
    ];
    let cpu_roasts_high = vec![
        "Your CPU is decent, but are you really pushing it, or just browsing memes?",
        "You've got the power, but do you have the skills to use it?",
        "Your CPU is so fast, it finishes tasks before you even think of them.",
        "Did you buy that CPU just to flex on your friends?",
        "Your processor is a beast, but are you taming it or just letting it nap?",
        "I bet your CPU has its own fan club.",
        "Your CPU is so powerful, it probably runs on pure ambition.",
        "It's not a CPU, it's a supercomputer in disguise.",
        "Your CPU is the reason \"lag\" is just a myth to you.",
        "I heard your CPU can render the entire universe in 8K.",
        "Your CPU is so good, it makes other CPUs cry.",
        "Are you sure that's a CPU and not a quantum computer?",
        "Your CPU's performance is a threat to national security.",
        "It's not throttling, it's just taking a very short power nap.",
        "Your CPU is the digital equivalent of a rocket ship.",
        "I heard your CPU is still trying to render the first frame of the multiverse.",
        "Does your CPU need a challenge after compiling the Linux kernel?",
        "Your CPU is so advanced, it probably runs on dark matter.",
        "The only thing slow about your CPU is how long it takes to find a worthy task.",
        "Your CPU is a master of efficiency.",
        "It's not a bug, it's a feature... of your lightning-fast CPU.",
        "Your CPU is the reason we don't have progress bars.",
        "I've seen slower calculations on a supercomputer.",
        "Your CPU is a monument to \"overkill\".",
        "The only thing your CPU is good at is making other CPUs jealous.",
    ];

    if cpu_count < 4 || cpu_frequency < 2000 {
        roasts.push(cpu_roasts_low.choose(&mut rng).unwrap().to_string());
    } else if cpu_count < 8 || cpu_frequency < 3000 {
        roasts.push(cpu_roasts_mid.choose(&mut rng).unwrap().to_string());
    } else {
        roasts.push(cpu_roasts_high.choose(&mut rng).unwrap().to_string());
    }

    // --- RAM Roasts ---
    let ram_roasts_low = vec![
        "Your RAM is so low, you probably have to close your browser to open a text editor.",
        "Is your RAM powered by a single gerbil on a tiny treadmill?",
        "I've seen more memory in a sticky note.",
        "Your RAM is the reason \"out of memory\" is your favorite error message.",
        "Does your computer run on hopes and dreams, because it's not running on RAM.",
        "Your RAM is so small, it gets lost in a single tab.",
        "I bet your RAM still thinks 256MB is a lot.",
        "Your RAM is the digital equivalent of a goldfish's memory.",
        "It's not multitasking, it's just desperately trying to remember one thing at a time.",
        "Your RAM is the reason your computer sounds like a jet engine taking off.",
        "I've seen faster data retrieval from a stone tablet.",
        "Your RAM is a bottleneck so severe, it's practically a chokehold.",
        "Does your RAM need a nap after opening the task manager?",
        "Your RAM is the reason you're still using Internet Explorer.",
        "The only thing fast about your RAM is how quickly it fills up.",
        "Your RAM is a master of forgetting.",
        "It's not a bug, it's a feature... of your tiny RAM.",
        "Your RAM is the reason we have swap files.",
        "I've seen more efficient memory management in a toddler's brain.",
        "Your RAM is a monument to \"just barely not enough\".",
        "The only thing your RAM is good at is making you upgrade.",
        "Your RAM is so small, it can't even hold a single thought.",
        "I bet your RAM is still trying to load the first pixel of your desktop.",
        "Your RAM is the reason you have to restart your computer every hour.",
        "The only thing your RAM is good at is making you frustrated.",
    ];
    let ram_roasts_mid = vec![
        "Your RAM is like your memory of last week's tasks: barely enough to get by.",
        "It's got enough memory for a few tabs and a mild existential crisis.",
        "Your RAM is the definition of \"adequate.\"",
        "A solid C+ for effort, your RAM.",
        "Your RAM is like a small apartment: enough space, but you're always bumping into things.",
        "It's not a lot, but it's not nothing either.",
        "Your RAM is the middle child of memory: always overlooked.",
        "It's got enough memory to run your OS and a few background apps.",
        "Your RAM is the reason \"close some programs\" is your mantra.",
        "It's not breaking any speed records, but it's not breaking your budget either.",
        "Your RAM is the beige of memory: functional, but unexciting.",
        "It's got enough memory for basic tasks and a few open documents.",
        "Your RAM is the equivalent of a participation trophy in the memory Olympics.",
        "It's not fast, it's not slow, it's just... there.",
        "Your RAM is the background noise of your computing life: always present, rarely noticed.",
        "It's got enough memory for a few browser tabs and a mild sense of regret.",
        "Your RAM is the definition of \"average.\"",
        "It's not a beast, but it's not a total slouch either.",
        "Your RAM is the reason you have time to make a sandwich while waiting for apps to load.",
        "It's got enough memory to run your OS and a mild sense of disappointment.",
        "Your RAM is the unsung hero of \"just barely enough.\"",
        "It's not a superhighway, but it's not a dirt road either.",
        "Your RAM is the definition of \"meh.\"",
        "It's got enough memory to run your favorite retro games.",
        "Your RAM is the reason you appreciate fast memory.",
    ];
    let ram_roasts_high = vec![
        "Plenty of RAM, but are you using it for anything productive, or just 50 Chrome tabs?",
        "You've got the memory, but do you have the applications to fill it?",
        "Your RAM is so vast, it probably has its own zip code.",
        "Did you buy that RAM just to flex on your friends?",
        "Your memory is a beast, but are you taming it or just letting it idle?",
        "I bet your RAM has its own fan club.",
        "Your RAM is so powerful, it probably runs on pure ambition.",
        "It's not RAM, it's a data ocean.",
        "Your RAM is the reason \"out of memory\" is just a legend to you.",
        "I heard your RAM can store the entire internet.",
        "Your RAM is so good, it makes other RAM modules cry.",
        "Are you sure that's RAM and not a quantum storage device?",
        "Your RAM's capacity is a threat to national security.",
        "It's not filling up, it's just taking a very short data nap.",
        "Your RAM is the digital equivalent of a black hole for data.",
        "I heard your RAM is still trying to load the first byte of the multiverse.",
        "Does your RAM need a challenge after opening every program you own?",
        "Your RAM is so advanced, it probably runs on dark matter.",
        "The only thing slow about your RAM is how long it takes to find a worthy task.",
        "Your RAM is a master of retention.",
        "It's not a bug, it's a feature... of your massive RAM.",
        "Your RAM is the reason we don't have swap files.",
        "I've seen slower data storage on a supercomputer.",
        "Your RAM is a monument to \"overkill\".",
        "The only thing your RAM is good at is making other RAM modules jealous.",
    ];

    if total_ram_gb < 8.0 {
        roasts.push(ram_roasts_low.choose(&mut rng).unwrap().to_string());
    } else if total_ram_gb < 16.0 {
        roasts.push(ram_roasts_mid.choose(&mut rng).unwrap().to_string());
    } else {
        roasts.push(ram_roasts_high.choose(&mut rng).unwrap().to_string());
    }

    // --- GPU Roasts ---
    let gpu_roasts_integrated = vec![
        "Your GPU is so weak, it struggles to render a single pixel in 4K. Maybe try ASCII art?",
        "Is your GPU powered by a single AA battery?",
        "I've seen more graphical fidelity in a flipbook.",
        "Your GPU is the reason \"low settings\" is your default.",
        "Does your computer run on hopes and dreams, because it's not running on a dedicated GPU.",
        "Your GPU is so small, it gets lost in a single texture.",
        "I bet your GPU still thinks 640x480 is high resolution.",
        "Your GPU is the digital equivalent of a crayon drawing.",
        "It's not gaming, it's just desperately trying to display one frame at a time.",
        "Your GPU is the reason your computer sounds like a jet engine taking off when you open Solitaire.",
        "I've seen faster rendering from a cave painting.",
        "Your GPU is a bottleneck so severe, it's practically a chokehold on your pixels.",
        "Does your GPU need a nap after rendering a static webpage?",
        "Your GPU is the reason you're still playing games from the 90s.",
        "The only thing fast about your GPU is how quickly it disappoints.",
        "Your GPU is a master of pixelation.",
        "It's not a bug, it's a feature... of your integrated graphics.",
        "Your GPU is the reason we have \"minimum requirements\".",
        "I've seen more efficient graphics processing in a toaster.",
        "Your GPU is a monument to \"just barely not enough pixels\".",
        "The only thing your GPU is good at is making you upgrade.",
        "Your GPU is so weak, it can't even render a single thought.",
        "I bet your GPU is still trying to load the first pixel of your desktop background.",
        "Your GPU is the reason you have to restart your computer after watching a YouTube video.",
        "The only thing your GPU is good at is making you frustrated.",
    ];
    let gpu_roasts_low_end = vec![
        "Your GPU is trying its best, bless its little silicon heart. Almost there!",
        "It's not the fastest, but at least it's not actively trying to sabotage your framerate.",
        "Your GPU is the definition of \"gets the job done... eventually.\"",
        "A solid C- for effort, your GPU.",
        "Your GPU is like a reliable old car: it runs, but don't ask it to win any graphical races.",
        "It's not a powerhouse, but it's not a complete embarrassment either.",
        "Your GPU is the middle child of graphics cards: always overlooked.",
        "It's got enough VRAM to count your fingers, and maybe a few toes.",
        "Your GPU is the reason \"good enough\" exists for low settings.",
        "It's not breaking any records, but it's not breaking your bank either.",
        "Your GPU is the beige of graphics: functional, but unexciting.",
        "It's got enough power for spreadsheets and mild graphical regret.",
        "Your GPU is the equivalent of a participation trophy in the graphics Olympics.",
        "It's not fast, it's not slow, it's just... there.",
        "Your GPU is the background music of your gaming life: always present, rarely noticed.",
        "It's got enough oomph for basic tasks and questioning your graphical choices.",
        "Your GPU is the definition of \"average.\"",
        "It's not a beast, but it's not a total slouch either.",
        "Your GPU is the reason you have time to make a sandwich while waiting for textures to load.",
        "It's got enough power to run your OS and a mild sense of disappointment in your framerate.",
        "Your GPU is the unsung hero of \"just barely playable.\"",
        "It's not a Ferrari, but it's not a tricycle either.",
        "Your GPU is the definition of \"meh.\" for gaming.",
        "It's got enough power to run your favorite retro games in glorious pixelation.",
        "Your GPU is the reason you appreciate fast graphics cards.",
    ];
    let gpu_roasts_high_end = vec![
        "Your GPU is probably fine, but let's be honest, you're not playing Cyberpunk on max settings, are you?",
        "You've got the graphical power, but do you have the games to push it?",
        "Your GPU is so fast, it renders frames before you even think of them.",
        "Did you buy that GPU just to flex on your friends?",
        "Your graphics card is a beast, but are you taming it or just letting it idle?",
        "I bet your GPU has its own fan club.",
        "Your GPU is so powerful, it probably runs on pure ambition and RGB.",
        "It's not a GPU, it's a pixel-pushing supercomputer in disguise.",
        "Your GPU is the reason \"lag\" is just a myth to you in games.",
        "I heard your GPU can render the entire universe in 8K, with ray tracing.",
        "Your GPU is so good, it makes other GPUs cry in envy.",
        "Are you sure that's a GPU and not a quantum rendering device?",
        "Your GPU's performance is a threat to national security for its sheer power.",
        "It's not throttling, it's just taking a very short rendering nap.",
        "Your GPU is the digital equivalent of a rocket ship for graphics.",
        "I heard your GPU is still trying to render the first frame of the multiverse in real-time.",
        "Does your GPU need a challenge after rendering every game you own at max settings?",
        "Your GPU is so advanced, it probably runs on dark matter and unicorn tears.",
        "The only thing slow about your GPU is how long it takes to find a worthy graphical task.",
        "Your GPU is a master of visual fidelity.",
        "It's not a bug, it's a feature... of your lightning-fast GPU.",
        "Your GPU is the reason we don't have graphical limitations.",
        "I've seen slower rendering on a supercomputer.",
        "Your GPU is a monument to \"overkill\" in the best way possible.",
        "The only thing your GPU is good at is making other GPUs jealous with its performance.",
    ];

    if gpu_name.contains("Integrated") || gpu_name.contains("Intel") || gpu_name.contains("AMD Radeon Graphics") && !gpu_name.contains("RX") {
        roasts.push(gpu_roasts_integrated.choose(&mut rng).unwrap().to_string());
    } else if gpu_name.contains("NVIDIA GeForce") || gpu_name.contains("AMD Radeon") {
        // More nuanced check for dedicated GPUs
        if gpu_name.contains("GT") || gpu_name.contains("RX 5") || gpu_name.contains("RX 4") {
            roasts.push(gpu_roasts_low_end.choose(&mut rng).unwrap().to_string());
        } else {
            roasts.push(gpu_roasts_high_end.choose(&mut rng).unwrap().to_string());
        }
    } else {
        roasts.push(gpu_roasts_low_end.choose(&mut rng).unwrap().to_string()); // Default to low-end if unknown
    }

    // --- Form Factor Roasts ---
    let form_factor_roasts_laptop = vec![
        "Ah, a laptop user. Enjoy your portable space heater.",
        "Your laptop's battery life is shorter than your attention span.",
        "I bet your laptop fan sounds like a jet engine taking off.",
        "Portable power, portable problems.",
        "Your laptop is great for gaming... if the game is Solitaire.",
        "The only thing thinner than your laptop is your patience for its performance.",
        "Your laptop is so light, it probably floats away when you open too many tabs.",
        "I've seen more desk space on a postage stamp than you have with that laptop.",
        "Your laptop's keyboard probably has more crumbs than keys.",
        "The only thing your laptop is good at is burning your thighs.",
        "Your laptop is a master of thermal throttling.",
        "It's not a bug, it's a feature... of your laptop's overheating.",
        "Your laptop is the reason we have cooling pads.",
        "I've seen more efficient cooling in a desert.",
        "Your laptop is a monument to \"almost a desktop\".",
        "The only thing your laptop is good at is making you wish you had a desktop.",
        "Your laptop is so small, it can't even hold a single thought.",
        "I bet your laptop is still trying to load the first pixel of your desktop background.",
        "Your laptop is the reason you have to restart your computer every hour.",
        "The only thing your laptop is good at is making you frustrated.",
        "Your laptop is the digital equivalent of a hot potato.",
        "I heard your laptop can cook an egg on its keyboard.",
        "Your laptop is so quiet, you can hear the dust bunnies breeding inside.",
        "The only thing your laptop is good at is being a portable disappointment.",
        "Your laptop is the reason you carry a power bank everywhere.",
    ];
    let form_factor_roasts_desktop = vec![
        "A desktop user, I see. Enjoy being tethered to your desk, cave dweller.",
        "Your desktop is so big, it probably has its own zip code.",
        "I bet your desktop fan sounds like a wind tunnel.",
        "Immovable power, immovable problems.",
        "Your desktop is great for gaming... if the game is \"find the dust bunny\".",
        "The only thing wider than your desktop is your waistline from sitting all day.",
        "Your desktop is so heavy, it probably has its own gravitational pull.",
        "I've seen more portability in a refrigerator than in your desktop.",
        "Your desktop's cables probably have more knots than a sailor's convention.",
        "The only thing your desktop is good at is collecting dust.",
        "Your desktop is a master of cable management nightmares.",
        "It's not a bug, it's a feature... of your desktop's massive footprint.",
        "Your desktop is the reason we have bigger desks.",
        "I've seen more efficient space utilization in a landfill.",
        "Your desktop is a monument to \"overkill\".",
        "The only thing your desktop is good at is making you wish you had a laptop.",
        "Your desktop is so big, it can't even fit in a single thought.",
        "I bet your desktop is still trying to load the first pixel of your desktop background.",
        "Your desktop is the reason you have to restart your computer every hour.",
        "The only thing your desktop is good at is making you frustrated.",
        "Your desktop is the digital equivalent of a brick house.",
        "I heard your desktop can heat an entire room in winter.",
        "Your desktop is so loud, you can hear it from the next county.",
        "The only thing your desktop is good at is being a stationary disappointment.",
        "Your desktop is the reason you never leave your house.",
    ];

    if form_factor == "Laptop" {
        roasts.push(form_factor_roasts_laptop.choose(&mut rng).unwrap().to_string());
    } else {
        roasts.push(form_factor_roasts_desktop.choose(&mut rng).unwrap().to_string());
    }

    // --- General Roasts (if not enough specific roasts) ---
    if roasts.len() < 3 {
        let general_roasts = vec![
            "Overall, your system is a testament to \"it works, mostly.\" Don't worry, we've all been there.",
            "Your system is... adequate. Just like your social life.",
            "I've seen better specs on a toaster. Just kidding... mostly.",
            "Your computer is like a fine wine: it gets slower with age.",
            "The only thing fast about your system is how quickly it disappoints.",
            "Your system is a master of procrastination.",
            "It's not a bug, it's a feature... of your unique setup.",
            "Your system is the reason we have progress bars.",
            "I've seen faster calculations on an abacus.",
            "Your system is a monument to \"almost there\".",
            "The only thing your system is good at is generating heat.",
            "Your computer is like a bad relationship: constantly letting you down.",
            "I bet your system still thinks Windows 95 is cutting edge.",
            "Your system is the digital equivalent of waiting for paint to dry.",
            "It's not slow, it's just taking a very long nap.",
            "Your system is the reason you have time to make a sandwich while waiting.",
            "I heard your system is still trying to render the first frame of Pong.",
            "Does your system need a nap after opening Notepad?",
            "Your system is so outdated, it probably runs on steam.",
            "The only thing fast about your system is how quickly it disappoints.",
            "Your system is a master of procrastination.",
            "It's not a bug, it's a feature... of your slow system.",
            "Your system is the reason we have progress bars.",
            "I've seen faster calculations on an abacus.",
            "Your system is a monument to \"almost there\".",
        ];
        roasts.push(general_roasts.choose(&mut rng).unwrap().to_string());
    }

    for roast in roasts {
        println!("  {}", roast.yellow());
    }
    println!("");
}