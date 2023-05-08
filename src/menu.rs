use console::{Term, Key, Style};

use crate::project::Project;

const MIN_PATH_PADDING: usize = 10;
const PROJECT_TYPE_PADDING: usize = 8;
const LAST_MOD_PADDING: usize = 10;
const SIZE_PADDING: usize = 18;
const MIN_CHARS: usize = MIN_PATH_PADDING + PROJECT_TYPE_PADDING + LAST_MOD_PADDING + SIZE_PADDING;

pub fn project_menu(projects: Vec<Project>, verbose: bool) {
    let mut max_path_len = 0;

    for project in &projects {
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
        let label = create_label(&project, max_path_len);
        let action = MenuAction::Delete(project);
        let menu_item = MenuItem::new(&label, action);
        menu_items.push(menu_item);
    }

    let mut menu = Menu::new(menu_items, verbose);
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
    Delete(Project)
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
    selected_item: usize,
    selected_page: usize,
    items_per_page: usize,
    num_pages: usize,
    page_start: usize,
    page_end: usize,
    verbose: bool,
    message: Option<String>,
    max_path_len: usize
}

impl Menu {
    pub fn new(items: Vec<MenuItem>, verbose: bool) -> Self {
        let mut items_per_page: i32 =
        if verbose {
            Term::stdout().size().0 as i32 - 9
        } else {
            Term::stdout().size().0 as i32 - 6
        };
        if items_per_page < 1 { items_per_page = 1 }
        let items_per_page = items_per_page as usize;
        let num_pages = ( (items.len() - 1)  / items_per_page ) + 1;
        let max_path_len = items[0].label.len() - MIN_CHARS;

        let mut menu = Self {
            title: None,
            items,
            selected_item: 0,
            selected_page: 0,
            items_per_page,
            num_pages,
            page_start: 0,
            page_end: 0,
            verbose,
            message: None,
            max_path_len,
        };
        menu.set_page(0);
        menu
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
        loop {
            let key = stdout.read_key().unwrap();

            match key {
                Key::ArrowUp => {
                    if self.selected_item != self.page_start { self.selected_item -= 1 }
                }
                Key::ArrowDown => {
                   if self.selected_item + 1 < self.page_end { self.selected_item += 1 }
                }
                Key::ArrowLeft => {
                    if self.selected_page != 0 {
                        self.set_page(self.selected_page - 1);
                    }
                }
                Key::ArrowRight => {
                    if self.selected_page < self.num_pages - 1 {
                        self.set_page(self.selected_page + 1);
                    }
                }
                Key::Escape | Key::Char('q') => {
                    self.exit(stdout);
                    break;
                }
                Key::Enter => {
                    self.run_action(self.selected_item);
                }
                _ => {}
            }
            
            self.draw(stdout);
        }
    }

    fn set_page(&mut self, page: usize) {
        self.selected_page = page;
        self.page_start = self.selected_page * self.items_per_page;
        self.selected_item = self.page_start;
        if self.items.len() > self.page_start + self.items_per_page {
            self.page_end = self.page_start + self.items_per_page
        } else {
            self.page_end = self.items.len()
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

        for (i, option) in self.items[self.page_start..self.page_end].iter().enumerate() {
            let selected_idx = self.page_start + i;
            if selected_idx == self.selected_item {
                let style = Style::new().bold();
                stdout.write_line(&format!("> {}", style.apply_to(&option.label))).unwrap();
            } else {
                stdout.write_line(&format!("  {}", option.label)).unwrap();
            }
        }
        stdout.write_line(&format!("Page {} of {}", self.selected_page + 1, self.num_pages)).unwrap();

        if let Some(message) = &self.message {
            let style = Style::new().red();
            stdout.write_line(&format!("\n{}", style.apply_to(message))).unwrap();
        }

        stdout.flush().unwrap();
    }

    fn exit(&self, stdout: &Term) {
        stdout.clear_screen().unwrap();
        stdout.show_cursor().unwrap();
        stdout.flush().unwrap();
    }

    fn run_action(&mut self, action_idx: usize) {
        let action = &mut self.items[action_idx].action;
        match action {
            MenuAction::Delete(project) => {
                let message = project.delete();
                if self.verbose { self.message = message; }
                self.items[action_idx].label = create_label(&project, self.max_path_len);
            }
        }
    }
}