use console::{Term, Key};

use crate::project::Project;

pub fn project_menu(projects: &Vec<Project>) {
    const MIN_PADDING: usize = 10;
    const PROJECT_TYPE_PADDING: usize = 8;
    const LAST_MOD_PADDING: usize = 10;
    let mut max_path_len = 0;
    let mut max_size_len = 0;

    for project in projects {
        let path_name = project.path.to_str().unwrap().to_string();
        if path_name.len() > max_path_len {
            max_path_len = path_name.len();
        }
        if project.rm_size_str.len() > max_size_len {
            max_size_len = project.rm_size_str.len();
        }
    }
    max_size_len += 2;

    println!("{}{}{}{}\n",
        format!("{:<width$}", "Path", width=(max_path_len + MIN_PADDING)),
        format!("{:<width$}", "Type", width=PROJECT_TYPE_PADDING),
        format!("{:>width$}", "Last Mod.", width=LAST_MOD_PADDING),
        format!("{:>width$}", "Size", width=max_size_len),
    );
    print!("{}{}{}{}\n",
        format!("{:<width$}", "----", width=(max_path_len + MIN_PADDING)),
        format!("{:<width$}", "----", width=PROJECT_TYPE_PADDING),
        format!("{:>width$}", "----", width=LAST_MOD_PADDING),
        format!("{:>width$}", "----", width=max_size_len), 
    );

    let mut menu_items: Vec<MenuItem> = vec![];
    for project in projects {
        let menu_item = MenuItem::new(
            &format!("{}{}{}{}",
                format!("{:<width$}", project.path.display(), width=(max_path_len + MIN_PADDING)),
                format!("{:<width$}", project.project_type.to_string(), width=PROJECT_TYPE_PADDING),
                format!("{:>width$}", project.last_modified, width=LAST_MOD_PADDING),
                format!("{:>width$}", project.rm_size_str, width=max_size_len),
            )
        );
        menu_items.push(menu_item);
    }
    let menu = Menu::new(menu_items);
    menu.show();
}


pub struct MenuItem {
    pub label: String
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_owned()
        }
    }
}

pub struct Menu {
    items: Vec<MenuItem>,
    selected_item: usize
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            items,
            selected_item: 0
        }
    }

    pub fn show(mut self) {
        let stdout = Term::buffered_stdout();
        stdout.hide_cursor().unwrap();
    
        stdout.clear_screen().unwrap();
        self.draw(&stdout);
        self.run_navigation(&stdout);
    }

    fn run_navigation(&mut self, stdout: &Term) {
        let num_options = self.items.len() - 1;
        loop {
            let key = stdout.read_key();
            if key.is_err() { println!("Error reading keystroke"); return; }
            let key = key.unwrap();

            match key {
                Key::ArrowUp => {
                    if self.selected_item != 0 { self.selected_item -= 1 }
                }
                Key::ArrowDown => {
                   if self.selected_item != num_options { self.selected_item += 1 }
                }
                Key::Escape => {
                    stdout.show_cursor().unwrap();
                    return;
                }
                Key::Enter => {
                    // run action here
                    stdout.show_cursor().unwrap();
                    return;
                }
                Key::Char(c) => println!("char {}", c),
                _ => {}
            }

            self.draw(stdout);
        }
    }

    fn draw(&self, stdout: &Term) {
        stdout.clear_screen().unwrap();

        // draw title here

        for (i, option) in self.items.iter().enumerate() {
            if i == self.selected_item {
                stdout.write_line(&format!("> {}", option.label)).unwrap();
            } else {
                stdout.write_line(&format!("  {}", option.label)).unwrap();
            }
        }

        stdout.flush().unwrap();
    }
}   