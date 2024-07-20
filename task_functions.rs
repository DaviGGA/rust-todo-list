use std::{io};
use crate::Task;
use crate::task_module::Display;

pub fn get_user_option() -> usize {
    let mut input = String::new();
    get_input(&mut input);
    
    input.trim().parse::<usize>().unwrap()
}

pub fn add_task(tasks: &mut Vec<Task>) {

    let mut title = String::new();
    let mut description = String::new();

    println!("Título:");
    get_input(&mut title);

    println!("Description:");
    get_input(&mut description);

    println!("\nTask {} de desc. {} foi criada", title.trim(), description.trim());

    tasks.push(
        Task::new(String::from(title.trim()), 
        String::from(description.trim()))
    );
}

pub fn show_tasks(tasks: &Vec<Task>) {
    if tasks.len() == 0 {
        println!("Nenhuma tarefa adicionada ainda.")
    }

    for (index, task) in tasks.iter().enumerate() {
        println!("{}.", index + 1);
        task.display();
    }
}

pub fn complete_task(tasks: &mut Vec<Task>) {
    println!("Qual task você completou? ");
    
    let task_option = get_user_option();

    let completed_task = tasks.get_mut(task_option - 1);
    
    match completed_task {
        Some(task) => {
            task.complete_task();
            println!("\nTask {} foi completada!", task.title);
        },
        None => {
            println!("\nTask inválida.")
        }
    }
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    println!("Qual task você quer deletar?");
    
    let task_option = get_user_option();
    
    if task_option >= tasks.len() {
        println!("\nTask inválida.");
        return
    }

    let removed_task = tasks.remove(task_option - 1);


    println!("\nTask {} foi removida com sucesso!", removed_task.title);
}

pub fn update_task(tasks: &mut Vec<Task>) {
    println!("Qual task você quer atualizar?");

    let task_option = get_user_option();

    if task_option >= tasks.len() {
        println!("\nTask inválida.");
        return
    }
    
    let task = &mut tasks[task_option - 1];
 
    let mut title = String::new();
    let mut description = String::new();

    println!("Título:");
    get_input(&mut title);

    println!("Description:");
    get_input(&mut description);

    let old_title = &task.title.clone();

    task.title = title;
    task.description = description;

    println!("\n Task {} foi atualizada!", old_title);
}

pub fn get_input(buff: &mut String) {
    io::stdin().read_line(buff).expect("Failed to read");
}