use std::slice;
use std::str::Split;
use std::{collections::LinkedList, error::Error};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use chrono::{DateTime, NaiveDateTime, Local};

fn main()
{
    println!("Hello, world!");

}

struct Task
{
    creation_date: NaiveDateTime,
    expiration_date: NaiveDateTime,
    text: String,
}

fn parse_string_to_tasks(s: &String) -> Result<Vec<Task>, Box<dyn Error>>
{
    let mut tasks: Vec<Task> = Vec::new();
    let mut split = s.split('\n');
    let mut current_line_opt = split.next();
    while(current_line_opt != None)
    {
        let current_line = current_line_opt.unwrap();
        let splitted: Vec<&str> = current_line.split(";").collect();
        let parse_err = "Cannot convert miliSec to NaiveDateTime";
        let current_dt = DateTime::from_timestamp_millis(splitted[0].parse()?).ok_or(parse_err)?;
        let expiration_dt = DateTime::from_timestamp_millis(splitted[1].parse()?).ok_or(parse_err)?;
        let mut text = String::new();
        for i in 2..splitted.len()
        {
            text = text + splitted[i];
        }
        let task = Task
        {
            creation_date: current_dt.naive_utc(),
            expiration_date: expiration_dt.naive_utc(),
            text: text
        };
        tasks.push(task);
        current_line_opt = split.next();
    }
    return Ok(tasks);
}

fn read_current_tasks() -> Result<Vec<Task>, Box<dyn Error>>
{
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
    let mut tasks = parse_string_to_tasks(&unparsed_tasks)?;

    return Ok(tasks);
}