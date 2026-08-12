use std::slice::{self, EscapeAscii};
use std::str::Split;
use std::string;
use std::thread::sleep;
use std::time::Duration;
use std::{collections::LinkedList, error::Error};
use std::fs::File;
use std::io::{Read, Write, stdout};
use std::path::Path;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use crossterm::event::read;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

mod escape_ansi;
mod input_mode;
mod rect;

use input_mode::InputMode;
use rect::Rect;

const timePattern: &str = "%Y-%m-%d %H:%M";
const editDelimeter: char = ';';
const file_name: &str = "tasks.txt";
const task_marker_checked: &str = "x";
const task_marker_unchecked: &str = " ";


fn main() -> Result<(), Box<dyn Error>>
{
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut inputMode = InputMode::NORMAL;
    let mut selectedTask: usize = 0;
    let mut new_task_text = String::new();

    let mut tasks = match read_current_tasks()
    {
        Err(err) => panic!("Cannot read current tasks.: {}", err),
        Ok(tasks) => tasks
    };

    println!("{}", escape_ansi::SMCUP);
    write!(stdout, "{}", escape_ansi::CURHOME);
    stdout.flush();
    enable_raw_mode();

    let mut input: [u8; 1] = [0; 1];
    loop
    {
        match inputMode
        {
            InputMode::NORMAL => display_tasks(&tasks, selectedTask, &stdout),
            InputMode::INSERT => display_edit(&new_task_text, &stdout)
        }
        stdout.flush();

        //input = [0; 4];
        stdin.read(&mut input);
        
        if(inputMode == InputMode::NORMAL)
        {
            if(input[0] == b'q')
            {
                break;
            }
            match input[0]
            {
                b'j' => if(tasks.len() != 0 && selectedTask < tasks.len() - 1) {selectedTask += 1}
                b'k' => if(tasks.len() != 0 && selectedTask > 0) {selectedTask -= 1}
                b'x' => if(tasks.len() > 0) {tasks[selectedTask].finished = true}
                b'X' => if(tasks.len() > 0) {tasks[selectedTask].finished = false}
                b'a' => inputMode = InputMode::INSERT,
                _ => ()
            }
        }
        else if(inputMode == InputMode::INSERT)
        {
            match input[0]
            {
                8 => {new_task_text.remove(new_task_text.len() - 1); ()}, // Backspace
                b'\r' => {inputMode = InputMode::NORMAL; tasks.push(create_new_task(&new_task_text)); new_task_text.clear()},
                _ => {new_task_text += str::from_utf8(&input).unwrap()}
            }
        }
    }
    //println!("{}", str::from_utf8(&mut input)?);
    //let dur = Duration::new(2, 0);
    //sleep(dur);
    disable_raw_mode();
    print!("{}", escape_ansi::RSTCLR);
    print!("{}", escape_ansi::RMCUP);
    print!("{}", escape_ansi::CNORM);
    stdout.flush();

    save_current_tasks(&tasks);

    return Ok(());
}

struct Task
{
    creation_date: NaiveDateTime,
    expiration_date: NaiveDateTime,
    text: String,
    finished: bool,
}

fn parse_string_to_tasks(s: &String) -> Result<Vec<Task>, Box<dyn Error>>
{
    let mut tasks: Vec<Task> = Vec::new();
    let mut split = s.split('\n');
    let mut current_line_opt = split.next();
    while(current_line_opt != None)
    {
        let current_line = current_line_opt.unwrap();
        if(current_line.len() == 0)
        {
            break;
        }
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
            text: text,
            finished: false,
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
    let mut tasks: Vec<Task>;
    if(unparsed_tasks.len() > 0)
    {
        tasks = parse_string_to_tasks(&unparsed_tasks)?;
    }
    else
    {
        tasks = Vec::new();
    }

    return Ok(tasks);
}

fn handle_input_escape(input: &[u8])
{
    for v in input
    {
        print!("{}", v);
    }
    println!("");
}

fn display_tasks(tasks: &Vec<Task>, selected_idx: usize, mut stdout: &std::io::Stdout)
{
    write!(stdout, "{}{}{}", escape_ansi::CLEAR, escape_ansi::CURHOME, escape_ansi::CIVIS);
    for (i, task) in tasks.iter().enumerate()
    {
        let dt_now = Local::now();
        let offset = dt_now.offset().to_owned();
        let expiration_dt = DateTime::<Local>::from_naive_utc_and_offset(task.expiration_date, offset);
        let expiration_str = format!("{}", expiration_dt.format(timePattern));
        if(i == selected_idx)
        {
            write!(stdout, "{}{}", escape_ansi::rgb_background(255, 255, 255), escape_ansi::rgb_foreground(0, 0, 0));
        }
        let mut task_marking: &str = task_marker_unchecked;
        if(task.finished == true)
        {
            task_marking = task_marker_checked;
        }
        
        write!(stdout, "[{}] {} | {}\n", task_marking, task.text, expiration_str);
        if(i == selected_idx)
        {
            write!(stdout, "{}", escape_ansi::RSTCLR);
        }
    }
}

fn display_edit(text: &String, mut stdout: &std::io::Stdout)
{
    write!(stdout, "{}{}{}{}", escape_ansi::CLEAR, escape_ansi::CURHOME, escape_ansi::CNORM, text);
}

fn create_new_task(text: &String) -> Task
{
    let (content, exp_date) = extract_date_from_text(&text);
    let task = Task
    {
        creation_date: Local::now().naive_utc(),
        expiration_date: exp_date,
        text: content,
        finished: false
    };
    return task;
}

fn extract_date_from_text(text: &String) -> (String, NaiveDateTime)
{
    let mut strings = match text.rsplit_once(editDelimeter)
    {
        None => (text.to_owned(), ""),
        Some((s1, s2)) => (s1.to_owned(), s2)
    };
    let dateTime = match NaiveDateTime::parse_from_str(strings.1, timePattern)
    {
        Err(_) => {strings.0 += strings.1; DateTime::from_timestamp_millis(Local::now().naive_utc().and_utc().timestamp_millis() + (24*60*60*1000)).unwrap().naive_utc()},
        Ok(dt) => Local.from_local_datetime(&dt).unwrap().naive_utc()
    };
    return (strings.0, dateTime);
}

fn save_current_tasks(tasks: &Vec<Task>)
{
    let mut file = match File::create(file_name)
    {
        Err(err) => panic!("Cannot create file to save tasks"),
        Ok(file) => file
    };

    let csv = tasks_to_string(tasks);
    file.write(csv.as_bytes());
}

fn tasks_to_string(tasks: &Vec<Task>) -> String
{
    let mut s = String::new();
    for task in tasks
    {
        if(task.finished == true)
        {
            continue;
        }

        s += &format!
        (
            "{};{};{}\n",
            task.creation_date.and_utc().timestamp_millis(),
            task.expiration_date.and_utc().timestamp_millis(),
            task.text
        );
    }
    return s;
}