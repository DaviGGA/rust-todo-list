mod task_module;
mod task_functions;

use task_module::Task;

use task_functions::{
    add_task,
    complete_task,
    delete_task,
    update_task,
    show_tasks,
    get_user_option
};

fn main() {
    let mut tasks: Vec<Task> = vec![];
    menu(&mut tasks);
}


fn menu(tasks: &mut Vec<Task>) {

    loop {
        println!("\n==== TO-DO LIST ====");     
        show_tasks(&tasks);
        
        println!("\n1. Adicionar tarefa");
        println!("2. Completar tarefa");
        println!("3. Deletar tarefa");
        println!("4. Atualizar tarefa");
        println!("5. Sair");
        
        let option = get_user_option();
        
        match option {
            1 => add_task(tasks),
            2 => complete_task(tasks),
            3 => delete_task(tasks),
            4 => update_task(tasks),
            5 => break,
            _ => {
                println!("\n Opção inválida.")
            }
        }        
    }

}


