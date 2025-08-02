pub mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    let mut max_store: Option<(&String, &Store)> = None;
    let mut max_size = 0;
    
    for floor in mall.floors.values() {
        for (store_name, store) in &floor.stores {
            if store.square_meters > max_size {
                max_size = store.square_meters;
                max_store = Some((store_name, store));
            }
        }
    }
    
    max_store.unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut employees = Vec::new();
    let mut max_salary = 0.0;
    
    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (emp_name, employee) in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                }
                employees.push((emp_name, employee));
            }
        }
    }
    
    let mut result = Vec::new();
    for (name, employee) in employees {
        if employee.salary == max_salary {
            result.push((name, employee));
        }
    }
    
    result
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = 0;
    
    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            count += store.employees.len();
        }
    }
    
    count + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, available_sec: HashMap<String, Guard>) {
    let mut total_size = 0;
    
    for floor in mall.floors.values() {
        total_size += floor.size_limit;
    }
    
    let total_areas = total_size / 200;
    let unguarded_areas = total_areas as usize - mall.guards.len();
    
    let mut hired_count = 0;
    for (name, guard) in available_sec {
        if hired_count >= unguarded_areas {
            break;
        }
        mall.hire_guard(name, guard);
        hired_count += 1;
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let shift_hours = employee.working_hours.1 - employee.working_hours.0;
                if shift_hours >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.cut(employee.salary * 0.1);
                }
            }
        }
    }
}