use chrono::NaiveDateTime;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, digit1};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

use crate::duty_record::{DutyRecord, Event};

pub fn parse_input(input: &str) -> IResult<&str, Vec<DutyRecord>> {
    separated_list1(tag("\n"), parse_row)(input)
}

fn parse_row(input: &str) -> IResult<&str, DutyRecord> {
    let (input, datetime) = delimited(char('['), parse_date, char(']'))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, event) = parse_event(input)?;

    Ok((input, DutyRecord { datetime, event }))
}

fn parse_date(input: &str) -> IResult<&str, NaiveDateTime> {
    let (input, raw_datetime) = take(16usize)(input)?;
    let datetime = NaiveDateTime::parse_from_str(raw_datetime, "%Y-%m-%d %H:%M").unwrap();

    Ok((input, datetime))
}

fn parse_event(input: &str) -> IResult<&str, Event> {
    alt((
        parse_begins_shift_event,
        parse_falls_asleep_event,
        parse_wakes_up_event,
    ))(input)
}

fn parse_begins_shift_event(input: &str) -> IResult<&str, Event> {
    let (input, _) = tag("Guard #")(input)?;
    let (input, raw_digit) = digit1(input)?;
    let (input, _) = tag(" begins shift")(input)?;

    let digit = raw_digit.parse().unwrap();

    return Ok((input, Event::BeginsShift(digit)));
}

fn parse_falls_asleep_event(input: &str) -> IResult<&str, Event> {
    let (input, _) = tag("falls asleep")(input)?;

    return Ok((input, Event::FallsAsleep));
}

fn parse_wakes_up_event(input: &str) -> IResult<&str, Event> {
    let (input, _) = tag("wakes up")(input)?;

    return Ok((input, Event::WakesUp));
}
