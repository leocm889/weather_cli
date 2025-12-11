//use colored::*;

//use crate::{
//    forecast::{add_weather, retrive_weathers_sorted, search_menu},
//    utils::read_input,
//};
//

#[derive(Debug, Copy, Clone)]
pub enum MenuItem {
    Home,
    Weathers,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Weathers => 1,
        }
    }
}

//pub fn display_menu(file_path: &str) {
//    println!("{}", "🎉 Welcome to the Weather Forecast!".bold().blue());
//
//    loop {
//        println!("{}", "=== Weather Forecast ===".bold().cyan());
//        println!("{}", "1. ➕ Add a Weather".green());
//        println!("{}", "2. 📋 View Weathers".yellow());
//        println!("{}", "3. 🔍 Search Weathers".magenta());
//        println!("{}", "4. 🚪Exit Program".white().bold());
//        println!("{}", "👉 Enter choice:".bold());
//
//        let choice = read_input::<u32>();
//
//        match choice {
//            1 => add_weather(file_path),
//            2 => retrive_weathers_sorted(file_path),
//            3 => search_menu(file_path),
//            4 => {
//                println!("{}", "👋 Goodbye!".bold().green());
//                break;
//            }
//            _ => {
//                println!("{}", "❌ Invalid choice, try again.".red().bold());
//                continue;
//            }
//        }
//    }
//}
