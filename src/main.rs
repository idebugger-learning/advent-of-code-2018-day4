use std::collections::HashMap;

use chrono::{NaiveDate, Timelike};
use duty_record::{DutyRecord, Event};
use itertools::Itertools;
use parser::parse_input;

mod duty_record;
mod parser;

#[derive(Debug, Copy, Clone)]
enum GuardState {
    Sleep,
    Awake,
}

#[derive(Debug)]
struct GuardSchedule {
    hour: [GuardState; 60],
    guard: u32,
}
type CalendarSchedule = HashMap<NaiveDate, GuardSchedule>;

fn main() {
    let raw_input = include_str!("./data/input.txt");

    let (rest_input, mut duty_records) = parse_input(raw_input).expect("Failed to parse input");
    duty_records.sort_unstable_by_key(|record| record.datetime);

    println!("Rest input length: {}", rest_input.len());
    println!("Sorted duty records: {:#?}", duty_records);

    let calendar_schedule = convert_duty_records_to_schedule(duty_records);

    pretty_print_calendar(&calendar_schedule);
}

fn convert_duty_records_to_schedule(records: Vec<DutyRecord>) -> CalendarSchedule {
    let grouped_records = records.into_iter().group_by(|record| record.extract_date());
    grouped_records
        .into_iter()
        .map(|(key, group)| (key, group.collect::<Vec<_>>()))
        .map(|(key, records)| {
            let mut hour = [GuardState::Awake; 60];
            let mut last_event_minute = 0;
            let mut guard_id = 0;
            for record in records {
                if let Event::BeginsShift(id) = record.event {
                    guard_id = id;
                    continue;
                }

                let minute = record.datetime.minute() as usize;
                let prev_state = match record.event {
                    Event::FallsAsleep => GuardState::Awake,
                    Event::WakesUp => GuardState::Sleep,
                    _ => unreachable!(),
                };
                for i in last_event_minute..minute {
                    hour[i] = prev_state;
                }
                last_event_minute = minute;
            }
            let schedule = GuardSchedule {
                hour,
                guard: guard_id,
            };
            (key, schedule)
        })
        .collect()
}

fn pretty_print_calendar(calendar: &CalendarSchedule) {
    println!("Date   ID     Minute");

    print!("{}", " ".repeat(12));
    for i in 0..=5 {
        print!("{}", i.to_string().repeat(10))
    }
    println!();

    print!("{}", " ".repeat(12));
    for _ in 0..=5 {
        for j in 0..=9 {
            print!("{}", j);
        }
    }
    println!();

    for (date, schedule) in calendar.into_iter().sorted_by_key(|(date, _)| date.clone()) {
        let print_date = date.format("%d-%m").to_string();
        print!("{}  #{:<4}  ", print_date, schedule.guard);
        for state in schedule.hour {
            match state {
                GuardState::Awake => {
                    print!(".")
                }
                GuardState::Sleep => {
                    print!("#")
                }
            }
        }
        println!();
    }
}
