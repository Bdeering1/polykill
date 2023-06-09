use console::{Key, Style, Term, pad_str, Alignment};

use crate::project::{Project, ProjectType};

const MIN_PATH_PADDING: usize = 10;
const PROJECT_TYPE_PADDING: usize = 12;
const LAST_MOD_PADDING: usize = 10;
const SIZE_PADDING: usize = 16;

pub fn project_menu(projects: Vec<Project>, verbose: bool) {
    let max_path_len = (&projects).iter().fold(0, |max, project| {
        let path_len = project.path.to_str().unwrap().to_string().len();
        if path_len > max { path_len } else { max }
    });

    let menu_title = format!(
        "  {}{}{}{}\n  {}{}{}{}",
        format_args!("{:<width$}", "Path", width=(max_path_len + MIN_PATH_PADDING)),
        format_args!("{:<width$}", "Type", width=PROJECT_TYPE_PADDING),
        format_args!("{:>width$}", "Last Mod.", width=LAST_MOD_PADDING),
        format_args!("{:>width$}", "Disk Savings", width=SIZE_PADDING),
        format_args!("{:<width$}", "----", width=(max_path_len + MIN_PATH_PADDING)),
        format_args!("{:<width$}", "----", width=PROJECT_TYPE_PADDING),
        format_args!("{:>width$}", "----", width=LAST_MOD_PADDING),
        format_args!("{:>width$}", "----", width=SIZE_PADDING),
    );

    let mut menu_items: Vec<MenuItem> = vec![];
    for project in projects {
        let label = create_label(&project, max_path_len);
        let action = MenuAction::Delete(project);
        let menu_item = MenuItem::new(&label, action);
        menu_items.push(menu_item);
    }

    let mut menu = Menu::new(menu_items, max_path_len, verbose);
    menu.title(&menu_title);
    menu.show();
}

fn create_label(project: &Project, max_path_len: usize) -> String {
    let project_type = if let ProjectType::Misc = project.project_type {
        format!("Misc ({})", project.rm_dirs[0].file_name().unwrap().to_str().unwrap())
    } else {
        project.project_type.to_string()
    };
    let last_modified = if let Some(days) = project.last_modified {
        format!("{} days", days)
    } else {
        String::from("unknown")
    };

    let type_color = match project.project_type {
        ProjectType::Cargo => 214,
        ProjectType::Node => 28,
        ProjectType::Dotnet => 171,
        ProjectType::Mix => 98,
        ProjectType::Gradle => 42,
        ProjectType::Composer => 117,
        ProjectType::Misc => 147,
    };
    let last_mod_color = match project.last_modified {
        Some(days) if days > 180 => 1,
        Some(days) if days > 30 => 3,
        Some(_days) => 2,
        None => 1,
    };
    let rm_size_color = match project.rm_size {
        size if size > 1000_000_000 => 1,
        size if size > 100_000_000 => 3,
        _ => 2,
    };

    format!(
        "{}{}{}{}",
        pad_right(project.path.display().to_string(), max_path_len + MIN_PATH_PADDING),
        pad_right(apply_color256(project_type, type_color), PROJECT_TYPE_PADDING),
        pad_left(apply_color256(last_modified, last_mod_color), LAST_MOD_PADDING),
        pad_left(apply_color256(project.rm_size_str.to_owned(), rm_size_color), SIZE_PADDING),
    )
}

fn pad_left(s: String, width: usize) -> String {
    pad_str(&s, width, Alignment::Right, None).to_string()
}

fn pad_right(s: String, width: usize) -> String {
    pad_str(&s, width, Alignment::Left, None).to_string()
}

fn apply_color256(input: String, color: u32) -> String {
    format!("\x1b[38;5;{}m{}\x1b[39m", color, input)
}

pub enum MenuAction {
    Delete(Project)
}

pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
}

impl MenuItem {
    pub fn new(label: &str, action: MenuAction) -> Self {
        Self {
            label: label.to_owned(),
            action,
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
    max_path_len: usize,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>, max_path_len: usize, verbose: bool) -> Self {
        let mut items_per_page: i32 =
            if verbose {
                Term::stdout().size().0 as i32 - 9
            } else {
                Term::stdout().size().0 as i32 - 6
            };
        if items_per_page < 1 { items_per_page = 1 }
        let items_per_page = items_per_page as usize;
        let num_pages = ((items.len() - 1) / items_per_page) + 1;

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
                Key::ArrowUp | Key::Char('k') => {
                    if self.selected_item != self.page_start {
                        self.selected_item -= 1;
                    } else if self.selected_page != 0 {
                        self.set_page(self.selected_page - 1);
                        self.selected_item = self.page_end;
                    }
                }
                Key::ArrowDown | Key::Char('j') => {
                    if self.selected_item < self.page_end {
                        self.selected_item += 1
                    } else if self.selected_page < self.num_pages - 1 {
                        self.set_page(self.selected_page + 1);
                    }
                }
                Key::ArrowLeft | Key::Char('h') => {
                    if self.selected_page != 0 {
                        self.set_page(self.selected_page - 1);
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if self.selected_page < self.num_pages - 1 {
                        self.set_page(self.selected_page + 1);
                    }
                }
                Key::Escape | Key::Char('q') => {
                    self.exit(stdout);
                    break;
                }
                Key::Enter | Key::Del => {
                    self.set_working(stdout);
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
            self.page_end = self.page_start + self.items_per_page - 1
        } else {
            self.page_end = self.items.len() - 1
        }
    }

    fn set_working(&mut self, stdout: &Term) {
        let MenuAction::Delete(project) = &mut self.items[self.selected_item].action;
        project.rm_size_str = String::from("working...");
        self.items[self.selected_item].label = create_label(project, self.max_path_len);
        self.draw(stdout);
    }

    fn draw(&self, stdout: &Term) {
        stdout.clear_screen().unwrap();

        if let Some(title) = &self.title {
            let controls_style = Style::new().dim();
            stdout.write_line(&format!("{}", controls_style.apply_to("  ↓,↑,←,→ to select project, enter to delete artifacts\n"))).unwrap();
            let title_style = Style::new().bold();
            stdout.write_line(&format!("{}", title_style.apply_to(title))).unwrap();
        }

        for (i, option) in self.items[self.page_start..=self.page_end].iter().enumerate() {
            if self.page_start + i == self.selected_item {
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
                self.items[action_idx].label = create_label(project, self.max_path_len);
            }
        }
    }
}
