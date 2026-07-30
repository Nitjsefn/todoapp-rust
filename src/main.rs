use std::{collections::LinkedList, error::Error};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use chrono::{NaiveDateTime, Local};

fn main() -> Result<(), Box<dyn Error>>
{
    println!("Hello, world!");

    let tasks_path = Path::new("tasks.txt");

    let mut tasks_file = match File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(tasks_path)
        {
            Err(err) => panic!("{}", err),
            Ok(file) => file
        };

        let mut unparsed_tasks = String::new();
        tasks_file.read_to_string(&mut unparsed_tasks)?;
        println!("{}", unparsed_tasks);

        parse_string_to_tasks(&unparsed_tasks);
        println!("{}", unparsed_tasks);
        return Ok(());
}

struct Task
{
    creation_date: NaiveDateTime,
    expiration_date: NaiveDateTime,
    text: String,
}

fn parse_string_to_tasks(s: &String) -> Result<LinkedList<Task>, Box<dyn Error>>
{
    let mut tasks: LinkedList<Task> = LinkedList::new();

    return Ok(tasks);
}