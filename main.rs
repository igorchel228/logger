use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

impl LogEntry {
    fn from_line(line: &str) -> Option<LogEntry> {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() == 3 {
            Some(LogEntry {
                timestamp: parts[0].trim().to_string(),
                level: parts[1].trim().to_string(),
                message: parts[2].trim().to_string(),
            })
        } else {
            None
        }
    }

    fn to_line(&self) -> String {
        format!("{}|{}|{}", self.timestamp, self.level, self.message)
    }
}

struct LogAnalyzer {
    entries: Vec<LogEntry>,
}

impl LogAnalyzer {
    fn new() -> LogAnalyzer {
        LogAnalyzer {
            entries: Vec::new(),
        }
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        if path.exists() {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Some(entry) = LogEntry::from_line(&line) {
                        self.entries.push(entry);
                    }
                }
            }
        }
        Ok(())
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for entry in &self.entries {
            writeln!(file, "{}", entry.to_line())?;
        }
        Ok(())
    }

    fn add_entry(&mut self, level: String, message: String) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.entries.push(LogEntry {
            timestamp,
            level,
            message,
        });
    }

    fn filter_by_level(&self, level: &str) -> Vec<LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level.eq_ignore_ascii_case(level))
            .cloned()
            .collect()
    }

    fn search(&self, query: &str) -> Vec<LogEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.message.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    fn get_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for entry in &self.entries {
            *stats.entry(entry.level.clone()).or_insert(0) += 1;
        }
        stats
    }

    fn count_total(&self) -> usize {
        self.entries.len()
    }

    fn get_recent(&self, count: usize) -> Vec<LogEntry> {
        let start = if self.entries.len() > count {
            self.entries.len() - count
        } else {
            0
        };
        self.entries[start..].to_vec()
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut analyzer = LogAnalyzer::new();
    let filename = "logs.txt";
    
    if let Err(e) = analyzer.load_from_file(filename) {
        println!("Could not load log file: {}", e);
    }

    loop {
        println!("\n=== Log Analyzer ===");
        println!("1. Add log entry");
        println!("2. View all logs");
        println!("3. Filter by level");
        println!("4. Search logs");
        println!("5. View statistics");
        println!("6. View recent logs");
        println!("7. Clear logs");
        println!("8. Save and exit");

        print!("\nEnter choice: ");
        io::stdout().flush().unwrap();
        let choice = read_line();

        match choice.as_str() {
            "1" => {
                print!("Level (INFO/WARNING/ERROR): ");
                io::stdout().flush().unwrap();
                let level = read_line().to_uppercase();

                print!("Message: ");
                io::stdout().flush().unwrap();
                let message = read_line();

                analyzer.add_entry(level, message);
                println!("Log entry added");
            }
            "2" => {
                println!("\nAll logs:");
                for entry in &analyzer.entries {
                    println!("[{}] {} - {}", entry.timestamp, entry.level, entry.message);
                }
            }
            "3" => {
                print!("Level: ");
                io::stdout().flush().unwrap();
                let level = read_line();

                let filtered = analyzer.filter_by_level(&level);
                println!("\nFiltered logs:");
                for entry in filtered {
                    println!("[{}] {} - {}", entry.timestamp, entry.level, entry.message);
                }
            }
            "4" => {
                print!("Search query: ");
                io::stdout().flush().unwrap();
                let query = read_line();

                let results = analyzer.search(&query);
                println!("\nSearch results:");
                for entry in results {
                    println!("[{}] {} - {}", entry.timestamp, entry.level, entry.message);
                }
            }
            "5" => {
                let stats = analyzer.get_statistics();
                println!("\nStatistics:");
                println!("Total entries: {}", analyzer.count_total());
                for (level, count) in stats {
                    println!("{}: {}", level, count);
                }
            }
            "6" => {
                print!("Number of recent logs: ");
                io::stdout().flush().unwrap();
                let count = read_line().parse::<usize>().unwrap_or(10);

                let recent = analyzer.get_recent(count);
                println!("\nRecent logs:");
                for entry in recent {
                    println!("[{}] {} - {}", entry.timestamp, entry.level, entry.message);
                }
            }
            "7" => {
                analyzer.clear();
                println!("Logs cleared");
            }
            "8" => {
                if let Err(e) = analyzer.save_to_file(filename) {
                    println!("Error saving: {}", e);
                } else {
                    println!("Logs saved");
                }
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
