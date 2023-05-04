use std::path::PathBuf;
use console::{Term, Key, Style};

use crate::project::{Project, delete};

const MIN_PATH_PADDING: usize = 10;
const PROJECT_TYPE_PADDING: usize = 8;
const LAST_MOD_PADDING: usize = 10;
const SIZE_PADDING: usize = 18;

pub fn project_menu(projects: &Vec<Project>) {
    let mut max_path_len = 0;

    for project in projects {
        let path_name = project.path.to_str().unwrap().to_string();
        if path_name.len() > max_path_len {
            max_path_len = path_name.len();
        }
    }

    let menu_title = format!("  {}{}{}{}\n  {}{}{}{}",
        format!("{:<width$}", "Path", width=(max_path_len + MIN_PATH_PADDING)),
        format!("{:<width$}", "Type", width=PROJECT_TYPE_PADDING),
        format!("{:>width$}", "Last Mod.", width=LAST_MOD_PADDING),
        format!("{:>width$}", "Disk Savings", width=SIZE_PADDING),
        format!("{:<width$}", "----", width=(max_path_len + MIN_PATH_PADDING)),
        format!("{:<width$}", "----", width=PROJECT_TYPE_PADDING),
        format!("{:>width$}", "----", width=LAST_MOD_PADDING),
        format!("{:>width$}", "----", width=SIZE_PADDING), 
    );

    let mut menu_items: Vec<MenuItem> = vec![];
    for project in projects {
        let label = create_label(project, max_path_len);
        let action = MenuAction::Delete(project.rm_dirs.to_owned());
        let menu_item = MenuItem::new(&label, action);
        menu_items.push(menu_item);
    }

    let mut menu = Menu::new(menu_items);
    menu.title(&menu_title);
    menu.show();
}

fn create_label(project: &Project, max_path_len: usize) -> String {
    format!("{}{}{}{}",
        format!("{:<width$}", project.path.display(), width=(max_path_len + MIN_PATH_PADDING)),
        format!("{:<width$}", project.project_type.to_string(), width=PROJECT_TYPE_PADDING),
        format!("{:>width$}", project.last_modified, width=LAST_MOD_PADDING),
        format!("{:>width$}", project.rm_size_str, width=SIZE_PADDING))
}


pub enum MenuAction {
    Delete(Vec<PathBuf>)
}

pub struct MenuItem {
    pub label: String,
    pub action: MenuAction
}

impl MenuItem {
    pub fn new(label: &str, action: MenuAction) -> Self {
        Self {
            label: label.to_owned(),
            action
        }
    }
}

pub struct Menu {
    title: Option<String>,
    items: Vec<MenuItem>,
    selected_item: usize
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            title: None,
            items,
            selected_item: 0
        }
    }

    pub fn title(&mut self, title: &str) {
        self.title = Some(title.to_owned());
    }

    pub fn show(&mut self) {
        let stdout = Term::buffered_stdout();

        stdout.hide_cursor().unwrap();
        stdout.clear_screen().unwrap();

        self.draw(&stdout);
        self.run_navigation(&stdout);
    }

    fn run_navigation(&mut self, stdout: &Term) {
        let num_options = self.items.len();
        loop {
            let key = stdout.read_key().unwrap();

            match key {
                Key::ArrowUp => {
                    if self.selected_item != 0 { self.selected_item -= 1 }
                }
                Key::ArrowDown => {
                   if self.selected_item < num_options - 1 { self.selected_item += 1 }
                }
                Key::Escape | Key::Char('q') => {
                    self.exit(stdout);
                    break;
                }
                Key::Enter => {
                    self.exit(stdout);
                    self.run_action(&self.items[self.selected_item].action);
                    break;
                }
                _ => {}
            }
            
            self.draw(stdout);
        }
    }

    fn draw(&self, stdout: &Term) {
        stdout.clear_screen().unwrap();

        if let Some(title) = &self.title {
            let controls_style = Style::new().dim();
            stdout.write_line(&format!("{}", controls_style.apply_to("  ↓ ↑ to select project, enter to delete artifacts\n"))).unwrap();
            let title_style = Style::new().bold();
            stdout.write_line(&format!("{}", title_style.apply_to(title))).unwrap();
        }

        for (i, option) in self.items.iter().enumerate() {
            if i == self.selected_item {
                let style = Style::new().bold();
                stdout.write_line(&format!("> {}", style.apply_to(&option.label))).unwrap();
            } else {
                stdout.write_line(&format!("  {}", option.label)).unwrap();
            }
        }

        stdout.flush().unwrap();
    }

    fn exit(&self, stdout: &Term) {
        stdout.clear_screen().unwrap();
        stdout.show_cursor().unwrap();
        stdout.flush().unwrap();
    }

    fn run_action(&self, action: &MenuAction) {
        match action {
            MenuAction::Delete(dirs) => {
                delete(dirs);
            }
        }
    }
}