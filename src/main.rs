use std::collections::HashMap;

use chrono::{NaiveDate, Timelike};
use duty_record::{DutyRecord, Event};
use itertools::Itertools;
use parser::parse_input;

mod duty_record;
mod parser;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    // let raw_input = include_str!("./data/input_example.txt");
    let raw_input = include_str!("./data/input.txt");

    let (rest_input, mut duty_records) = parse_input(raw_input).expect("Failed to parse input");
    duty_records.sort_unstable_by_key(|record| record.datetime);

    println!("Rest input length: {}", rest_input.len());
    println!("Sorted duty records: {:#?}", duty_records);

    let calendar_schedule = convert_duty_records_to_schedule(duty_records);

    pretty_print_calendar(&calendar_schedule);

    println!("Running strategy 1...");
    let guard = strategy_one(&calendar_schedule);
    println!("Strategy 1 result: {}", guard);

    println!("Running strategy 2...");
    let guard = strategy_two(&calendar_schedule);
    println!("Strategy 2 result: {}", guard);
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

    print!("{}", " ".repeat(14));
    for i in 0..=5 {
        print!("{}", i.to_string().repeat(10))
    }
    println!();

    print!("{}", " ".repeat(14));
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

fn strategy_one(calendar: &CalendarSchedule) -> u32 {
    let mut pockets_map: HashMap<u32, [u32; 60]> = HashMap::new();
    for (_, schedule) in calendar {
        let entry = pockets_map.entry(schedule.guard).or_insert([0u32; 60]);
        for i in 0..60 {
            if schedule.hour[i] == GuardState::Sleep {
                entry[i] += 1;
            }
        }
    }
    let max_minutes_asleep = pockets_map
        .into_iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<u32>())
        .expect("Failed to find a guard with max minutes asleep");
    println!(
        "=> Guard #{} spent the most minutes asleep",
        max_minutes_asleep.0
    );
    let most_asleep_minute = max_minutes_asleep
        .1
        .into_iter()
        .enumerate()
        .max_by_key(|(_, minute_count)| *minute_count)
        .expect("Failed to find a most asleep minute");
    println!(
        "=> Guard #{} sleep mostly at {} minute",
        max_minutes_asleep.0, most_asleep_minute.0
    );
    max_minutes_asleep.0 * most_asleep_minute.0 as u32
}

fn strategy_two(calendar: &CalendarSchedule) -> u32 {
    let mut pockets_map: HashMap<u32, [u32; 60]> = HashMap::new();
    for (_, schedule) in calendar {
        let entry = pockets_map.entry(schedule.guard).or_insert([0u32; 60]);
        for i in 0..60 {
            if schedule.hour[i] == GuardState::Sleep {
                entry[i] += 1;
            }
        }
    }
    let (guard_id, (minute, freq)) = pockets_map
        .into_iter()
        .map(|(guard, hour)| {
            (
                guard,
                hour.into_iter()
                    .enumerate()
                    .max_by_key(|(_, hour_value)| hour_value.clone())
                    .expect("Failed to find max for guard"),
            )
        })
        .max_by_key(|(_, (_, max_freq))| max_freq.clone())
        .expect("Failed to find most frequent asleep guard");
    println!(
        "=> Guard #{} was asleep at {} minute {} times",
        guard_id, minute, freq
    );
    return guard_id * minute as u32;
}
