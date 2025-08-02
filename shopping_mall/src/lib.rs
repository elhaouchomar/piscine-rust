
use std::collections::HashMap;
mod mall;
pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let hash: HashMap<String, Employee> = HashMap::new();

    let mut big_store = Store::new(hash, 0);
    let mut name_store = String::new();

    for (_, floor) in &mall.floors {
        for (nam_store, content_store) in &floor.stores.clone() {
            if content_store.square_meters > big_store.square_meters {
                big_store = content_store.clone();
                name_store = nam_store.clone();
            }
        }
    }
    (name_store, big_store)
}


pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut highest_paid: Vec<(String, Employee)> = Vec::new();

    let mut max_salary: i32 = 0;

    for (_, floor) in &mall.floors {
        for (_, store) in floor.stores.clone() {
            for (name_emp, emp) in store.employees {
                if emp.salary > max_salary as f64 {
                    max_salary = emp.salary as i32;
                    highest_paid = vec![(name_emp.clone(), emp.clone())]
                }
            }
        }
    }
    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut nbr_emp = mall.guards.len();
    
    for (_, floor) in &mall.floors.clone() {
        for (_, store) in &floor.stores.clone() {
            nbr_emp += store.employees.len();
        }
    }
    nbr_emp
}

pub fn check_for_securities(mall: &mut Mall, extra_guards: HashMap<String, Guard>) {
    let total_area: u64 = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    let required_guards = (total_area + 199) / 200;
    let current_guards = mall.guards.len();

    if current_guards >= required_guards as usize {
        return;
    }

    let needed = required_guards as usize - current_guards;

    for (name, guard) in extra_guards.into_iter().take(needed) {
        mall.hire_guard(name, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours = employee.working_hours.1.saturating_sub(employee.working_hours.0);
                if hours >= 10 {
                    employee.raise(employee.salary * 0.10);
                } else {
                    employee.cut(employee.salary * 0.10);
                }
            }
        }
    }
}