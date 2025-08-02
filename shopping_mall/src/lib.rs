use std::collections::HashMap;

mod mall;
pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .map(|(name, store)| (name.clone(), store.clone()))
        .expect("No stores found")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut max_salary = 0.0;
    let mut highest_paid = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, emp) in &store.employees {
                if emp.salary > max_salary {
                    max_salary = emp.salary;
                    highest_paid.clear();
                    highest_paid.push((name.clone(), emp.clone()));
                } else if (emp.salary - max_salary).abs() < std::f64::EPSILON {
                    highest_paid.push((name.clone(), emp.clone()));
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.floors
        .values()
        .map(|floor| floor.stores.values().map(|store| store.employees.len()).sum::<usize>())
        .sum()
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

    extra_guards.into_iter().take(needed).for_each(|(name, guard)| {
        mall.hire_guard(name, guard);
    });
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
