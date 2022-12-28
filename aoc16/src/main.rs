use std::collections::VecDeque;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

struct Line {
    valve: String,
    flow: usize,
    neighbours: Vec<String>,
}

#[derive(Debug)]
struct LineParseError {
    descr: &'static str,
    input: String,
}

impl Display for LineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (input: {})", self.descr, self.input)
    }
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (valve, rest) = s
            .strip_prefix("Valve ")
            .ok_or(LineParseError {
                descr: "no valve prefix",
                input: String::from(s),
            })?
            .split_once(" ")
            .ok_or(LineParseError {
                descr: "no space after valve",
                input: String::from(s),
            })?;

        let rate_part = rest.strip_prefix("has flow rate=").ok_or(LineParseError {
            descr: "no flow rate",
            input: String::from(s),
        })?;

        let (rate_str, list) = rate_part
            .split_once("; tunnels lead to valves ")
            .or_else(|| rate_part.split_once("; tunnel leads to valve "))
            .ok_or(LineParseError {
                descr: "no tunnels lead to valves",
                input: String::from(s),
            })?;
        let flow = rate_str.parse().map_err(|_| LineParseError {
            descr: "couldn't parse rate",
            input: String::from(s),
        })?;
        let mut neighbours = Vec::new();
        for part in list.split(", ") {
            neighbours.push(String::from(part));
        }
        Ok(Line {
            valve: String::from(valve),
            flow,
            neighbours,
        })
    }
}

struct Graph {
    names: Vec<String>,
    flows: Vec<usize>,
    neighbours: Vec<Vec<usize>>,
    start: usize,
}

impl Graph {
    fn from_lines(lines: &[Line]) -> Graph {
        let names: Vec<String> = lines.iter().map(|l| String::from(&l.valve)).collect();
        let flows = lines.iter().map(|l| l.flow).collect();
        let mut neighbours = Vec::new();
        for l in lines {
            let this_neighbours = l
                .neighbours
                .iter()
                .map(|l| names.iter().position(|s| s == l).unwrap())
                .collect();
            neighbours.push(this_neighbours);
        }
        let start = names.iter().position(|s| s == "AA").unwrap();
        Graph {
            names,
            flows,
            neighbours,
            start,
        }
    }
}

const MAX_TIME: usize = 26;

fn dists_from(pos: usize, neighbours: &[Vec<usize>]) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; neighbours.len()];
    let mut dist = vec![usize::MAX; neighbours.len()];
    dist[pos] = 0;
    queue.push_back(pos);
    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        visited[p] = true;
        for n in &neighbours[p] {
            if !visited[*n] {
                queue.push_back(*n);
                if dist[*n] > dist[p] {
                    dist[*n] = dist[p] + 1;
                }
            }
        }
    }
    dist
}

#[derive(Clone, Copy)]
struct State {
    time: Option<usize>,
    pos: usize,
    value: usize,
}

fn try_next(old: State, next: usize, dists: &[Vec<usize>], flows: &[usize]) -> State {
    match old.time {
        None => return old,
        Some(mut t) => {
            let dist = dists[old.pos][next];
            if dist + 1 > t {
                return State {
                    time: None,
                    pos: old.pos,
                    value: old.value,
                };
            }
            t -= dist + 1;
            State {
                time: Some(t),
                pos: next,
                value: old.value + flows[next] * t,
            }
        }
    }
}

fn max_flow_step_parallel(
    g: &Graph,
    valves: &[usize],
    used: &mut [bool],
    state1: State,
    state2: State,
    dists: &[Vec<usize>],
) -> usize {
    let mut max = 0;

    // only consider branches where we're not at the end yet
    if state1.time.is_none() && state2.time.is_none() {
        return state1.value + state2.value;
    }

    for (i, v) in valves.iter().enumerate() {
        if used[i] {
            continue;
        }

        used[i] = true;
        if state1.time.is_some() {
            let mut new_states = [state1, state2];
            new_states[0] = try_next(new_states[0], *v, dists, &g.flows);

            let total_value = new_states[0].value + new_states[1].value;
            if total_value > max {
                max = total_value;
            }

            let value =
                max_flow_step_parallel(g, &valves, used, new_states[0], new_states[1], dists);
            if value > max {
                max = value;
            }
        }
        if state2.time.is_some() {
            let mut new_states = [state1, state2];
            new_states[1] = try_next(new_states[1], *v, dists, &g.flows);

            let total_value = new_states[0].value + new_states[1].value;
            if total_value > max {
                max = total_value;
            }

            let value =
                max_flow_step_parallel(g, &valves, used, new_states[0], new_states[1], dists);
            if value > max {
                max = value;
            }
        }
        used[i] = false;
    }
    max
}

fn max_flow_step(
    g: &Graph,
    valves: &[usize],
    used: &mut [bool],
    state: State,
    dists: &[Vec<usize>],
) -> (usize, usize) {
    let mut max = 0;
    let mut viable = 0;

    if state.time.is_none() {
        return (state.value, 0);
    }

    for (i, v) in valves.iter().enumerate() {
        if used[i] {
            continue;
        }

        used[i] = true;
        let new_state = try_next(state, *v, dists, &g.flows);
        if new_state.value > max {
            max = new_state.value;
        }
        viable += 1;
        let (value, add_viable) = max_flow_step(g, &valves, used, new_state, dists);
        if value > max {
            max = value;
        }
        viable += add_viable;

        used[i] = false;
    }
    (max, viable)
}

fn max_flow(g: &Graph) -> usize {
    let mut all_dists = Vec::new();
    for p in 0..g.names.len() {
        all_dists.push(dists_from(p, &g.neighbours));
    }

    let real_valves: Vec<usize> = g
        .flows
        .iter()
        .enumerate()
        .filter(|(_, f)| **f > 0)
        .map(|(i, _)| i)
        .collect();

    let state1 = State {
        time: Some(MAX_TIME),
        pos: g.start,
        value: 0,
    };
    let state2 = state1;
    let mut used = vec![false; real_valves.len()];
    //max_flow_step_parallel(g, &real_valves, &mut used, state1, state2, &all_dists)
    let (max, viable) = max_flow_step(g, &real_valves, &mut used, state1, &all_dists);
    println!("viable: {viable}");
    max
}

fn main() {
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if bytes == 0 {
            break;
        }

        lines.push(line.trim().parse::<Line>().expect("couldn't parse line"));
    }

    let g = Graph::from_lines(&lines);
    let flow = max_flow(&g);
    println!("{flow}");
}
