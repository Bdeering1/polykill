use menu_rs::{MenuOption, Menu};

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

    let mut menu_items: Vec<MenuOption> = vec![];
    for project in projects {
        let menu_item = MenuOption::new(
            &format!("{}{}{}{}",
                format!("{:<width$}", project.path.display(), width=(max_path_len + MIN_PADDING)),
                format!("{:<width$}", project.project_type.to_string(), width=PROJECT_TYPE_PADDING),
                format!("{:>width$}", project.last_modified, width=LAST_MOD_PADDING),
                format!("{:>width$}", project.rm_size_str, width=max_size_len),
            ),
            noop
        );
        menu_items.push(menu_item);
    }
    let menu = Menu::new(menu_items);
    menu.show();
}

fn noop() {}