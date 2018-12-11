//Day 4: Repose Record
extern crate chrono;
extern crate regex;
extern crate time;

use std::collections::HashMap;
use day4::chrono::prelude::*;
use day4::regex::Regex;
use day4::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Awake,
    Asleep,
    StartedShift { id: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuardEvent {
    dt: NaiveDateTime,
    event: EventType,
}

#[aoc_generator(day4)]
pub fn input_schedule(input: &str) -> Vec<GuardEvent> {
    let mut collected: Vec<&str> = input.lines().collect();
    collected.sort(); //Ensure that the guard patterns are in order

    let schedule = Regex::new(r"^\[(.+)\] (.+)$").unwrap();
    let guard = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();

    // Parse the ordered list into guard events
    collected
        .iter()
        .map(|event| {
            let s_cap: Vec<&str> = schedule
                .captures(event)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().as_str())
                .collect();
            let dt = NaiveDateTime::parse_from_str(s_cap[1], "%Y-%m-%d %H:%M").unwrap();
            let event = {
                match s_cap[2] {
                    "falls asleep" => EventType::Asleep,
                    "wakes up" => EventType::Awake,
                    _ => {
                        let guard_id: usize = guard.captures(s_cap[2]).unwrap()[1].parse().unwrap();
                        EventType::StartedShift { id: guard_id }
                    }
                }
            };
            GuardEvent { dt, event }
        })
        .collect()
}

// Given the ordered list of guard events, reduce into a hashmap that is referenced by guard id
//
fn order_schedule(schedule: &[GuardEvent]) -> HashMap<usize, Vec<GuardEvent>> {
    let mut current_guard: Option<usize> = None;
    let mut mapped_schedule: HashMap<usize, Vec<GuardEvent>> = HashMap::new();

    for record in schedule {
        if let EventType::StartedShift { id } = record.event {
            current_guard = Some(id);
        }

        if let Some(id) = current_guard {
            let guard_entry = mapped_schedule.entry(id).or_insert(Vec::new());
            guard_entry.push(record.clone());
        }
    }
    mapped_schedule
}

// Find the sleepiest guard given the schedule, and the time spent asleep
//
fn sleepiest_guard(schedule: &HashMap<usize, Vec<GuardEvent>>) -> (usize, Duration) {
    let mut sleepiest: Option<usize> = None;
    let mut max = Duration::zero();

    // Go through each guard record and calculate the time spent asleep
    for (guard, records) in schedule.iter() {
        let mut sleep_start: Option<NaiveDateTime> = None;
        let mut asleep_for = Duration::zero();

        for r in records {
            match r.event {
                EventType::Asleep => sleep_start = Some(r.dt),
                EventType::Awake => {
                    if let Some(start) = sleep_start {
                        asleep_for = asleep_for + r.dt.signed_duration_since(start);
                        sleep_start = None;
                    }
                }
                _ => continue,  // The guard has started the next day
            }
        }

        if asleep_for > max {
            sleepiest = Some(*guard);
            max = asleep_for;
        }
    }

    (sleepiest.unwrap(), max)
}

// Given a guard's schedule, find the minute of the day where he is asleep the most
//
fn sleepiest_minute(schedule: &[GuardEvent]) -> usize {
    let mut sleep_start: Option<usize> = None;
    let mut counter = HashMap::new();

    for record in schedule {
        match record.event {
            EventType::Asleep => sleep_start = Some(record.dt.minute() as usize),
            EventType::Awake => {
                if let Some(start) = sleep_start {
                    for i in start..record.dt.minute() as usize {
                        let minute = counter.entry(i).or_insert(0);
                        *minute += 1;
                    }
                    sleep_start = None;
                }
            },
            _ => continue,
        }
    }

    counter.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val).unwrap()
}

#[aoc(day4, part1)]
pub fn part1(input: &[GuardEvent]) -> usize {
    let schedule = order_schedule(input);
    let (guard, _) = sleepiest_guard(&schedule);

    guard * sleepiest_minute(&schedule.get(&guard).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "[1518-11-01 00:00] Guard #10 begins shift\n\
                    [1518-11-01 00:05] falls asleep\n\
                    [1518-11-01 00:25] wakes up\n\
                    [1518-11-01 00:30] falls asleep\n\
                    [1518-11-01 00:55] wakes up\n\
                    [1518-11-01 23:58] Guard #99 begins shift\n\
                    [1518-11-02 00:40] falls asleep\n\
                    [1518-11-02 00:50] wakes up\n\
                    [1518-11-03 00:05] Guard #10 begins shift\n\
                    [1518-11-03 00:24] falls asleep\n\
                    [1518-11-03 00:29] wakes up\n\
                    [1518-11-04 00:02] Guard #99 begins shift\n\
                    [1518-11-04 00:36] falls asleep\n\
                    [1518-11-04 00:46] wakes up\n\
                    [1518-11-05 00:03] Guard #99 begins shift\n\
                    [1518-11-05 00:45] falls asleep\n\
                    [1518-11-05 00:55] wakes up";

    #[test]
    fn grok_input() {
        let expected = vec![
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 0, 0),
                event: EventType::StartedShift { id: 10 },
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 5, 0),
                event: EventType::Asleep,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 25, 0),
                event: EventType::Awake,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 30, 0),
                event: EventType::Asleep,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 55, 0),
                event: EventType::Awake,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 01).and_hms(23, 58, 0),
                event: EventType::StartedShift { id: 99 },
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 02).and_hms(0, 40, 0),
                event: EventType::Asleep,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 02).and_hms(0, 50, 0),
                event: EventType::Awake,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 03).and_hms(0, 5, 0),
                event: EventType::StartedShift { id: 10 },
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 03).and_hms(0, 24, 0),
                event: EventType::Asleep,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 03).and_hms(0, 29, 0),
                event: EventType::Awake,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 04).and_hms(0, 2, 0),
                event: EventType::StartedShift { id: 99 },
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 04).and_hms(0, 36, 0),
                event: EventType::Asleep,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 04).and_hms(0, 46, 0),
                event: EventType::Awake,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 05).and_hms(0, 3, 0),
                event: EventType::StartedShift { id: 99 },
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 05).and_hms(0, 45, 0),
                event: EventType::Asleep,
            },
            GuardEvent {
                dt: NaiveDate::from_ymd(1518, 11, 05).and_hms(0, 55, 0),
                event: EventType::Awake,
            },
        ];

        assert_eq!(
            input_schedule(TEST_STR),
            expected
        );
    }

    #[test]
    fn sleepy(){
        let schedule = order_schedule(&input_schedule(TEST_STR));
        let (guard, time_asleep) = sleepiest_guard(&schedule);
        let minute = sleepiest_minute(&schedule.get(&guard).unwrap());

        assert_eq!((guard, time_asleep), (10, Duration::minutes(50)));
        assert_eq!(minute, 24);
    }

    #[test]
    fn sample1(){
        assert_eq!(
            part1(&input_schedule(TEST_STR)),
            240
        )
    }
}