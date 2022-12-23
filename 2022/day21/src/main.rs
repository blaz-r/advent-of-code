use std::{fs, collections::HashMap};

#[derive(Clone, Debug)]
enum Oper {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String)
}

#[derive(Clone, Debug)]
struct Expr {
    expr: Oper,
    val: i64,
    human: bool
}


fn solve_expression(expr: &String, expr_map: &mut HashMap<String, Expr>) -> i64 {

    let val;
    let mut human = false;
    match &expr_map[expr].clone().expr {
        Oper::Num(n) => {val = *n},
        Oper::Add(e1, e2) => {
            val = solve_expression(e1, expr_map) + solve_expression(e2, expr_map);
            human = expr_map[e1].human | expr_map[e2].human;
        },
        Oper::Sub(e1, e2) => {
            val = solve_expression(e1, expr_map) - solve_expression(e2, expr_map);
            human = expr_map[e1].human | expr_map[e2].human;
        },
        Oper::Mul(e1, e2) => {
            val = solve_expression(e1, expr_map) * solve_expression(e2, expr_map);
            human = expr_map[e1].human | expr_map[e2].human;
        },
        Oper::Div(e1, e2) => {
            val = solve_expression(e1, expr_map) / solve_expression(e2, expr_map);
            human = expr_map[e1].human | expr_map[e2].human;
        },
    }

    let curr_expr = expr_map.get_mut(expr).unwrap();
    curr_expr.val = val;
    if expr == "humn" {
        curr_expr.human = true;
    }
    else {
        curr_expr.human = human;
    }
    return val;
}


fn make_equal(expr: String, expr_map: &HashMap<String, Expr>, val: i64) -> i64 {
    if expr == "humn" {
        return val;
    }

    let equal: i64;
    let next: String;
    match expr_map[&expr].clone().expr {
        Oper::Add(e1, e2) => {
            let (l, r) = (expr_map[&e1].clone(), expr_map[&e2].clone());
            if l.human {
                equal = val - r.val;
                next = e1;
            }
            else {
                equal = val - l.val;
                next = e2;
            }
        },
        Oper::Sub(e1, e2) => {
            let (l, r) = (expr_map[&e1].clone(), expr_map[&e2].clone());
            if l.human {
                equal = val + r.val;
                next = e1;
            }
            else {
                equal = l.val - val;
                next = e2;
            }
        },
        Oper::Mul(e1, e2) => {
            let (l, r) = (expr_map[&e1].clone(), expr_map[&e2].clone());
            if l.human {
                equal = val / r.val;
                next = e1;
            }
            else {
                equal = val / l.val;
                next = e2;
            }
        },
        Oper::Div(e1, e2) => {
            let (l, r) = (expr_map[&e1].clone(), expr_map[&e2].clone());
            if l.human {
                equal = val * r.val;
                next = e1;
            }
            else {
                equal = l.val / val;
                next = e2;
            }
        }
        Oper::Num(_) => panic!("Shouldn't reach"),
    }

    make_equal(next, &expr_map, equal)
}


fn day21(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file.");
    let mut expr_map: HashMap<String, Expr> = HashMap::new();

    let (mut l, mut r) = (String::new(), String::new());
    for line in data.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let name = split[0].to_owned();
        let value: Vec<&str> = split[1].split(" ").collect();
        let oper: Oper;
        if value.len() == 3 {
            let (v1, v2) = (value[0].to_owned(), value[2].to_owned());
            if name == "root" {
                (l, r) = (v1.clone(), v2.clone());
            }
            oper = match value[1] {
                "+" => Oper::Add(v1, v2),
                "-" => Oper::Sub(v1, v2),
                "*" => Oper::Mul(v1, v2),
                "/" => Oper::Div(v1, v2),
                _ => panic!("Shouldn't reach")
            };
        }
        else {
            oper = Oper::Num(value[0].parse().unwrap());
        }
        let expr = Expr {
            expr: oper,
            val: 0,
            human: false
        };
        expr_map.insert(name, expr);
    }

    println!("Part 1: {}", solve_expression(&String::from("root"), &mut expr_map));

    let (start, val) = if expr_map[&l].human { (l.clone(), expr_map[&r].val) } else { (r.clone(), expr_map[&l].val) };

    println!("Part 2: {}", make_equal(start, &expr_map, val));
}


fn main() {
    day21("input/input.txt");
}
