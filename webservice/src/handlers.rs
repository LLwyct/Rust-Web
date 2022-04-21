use actix_web::{web, HttpResponse};
use super::state::AppState;

pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    return HttpResponse::Ok().json(&response);
}


use super::models::Course;
use chrono::Utc;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>
) -> HttpResponse {
    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();
    let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    return HttpResponse::Ok().json("Course added");
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize)>
) -> HttpResponse {
    let teacher_id: usize = params.0;
    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id)
        .collect::<Vec<Course>>();
    if filtered_courses.len() > 0 {
        return HttpResponse::Ok().json(filtered_courses);
    } else {
        return HttpResponse::Ok().json("No course found for teacher".to_string());
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>
) -> HttpResponse {
    let (teacher_id, course_id) = params.0;
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|course| course.teacher_id == teacher_id && course.id == Some(course_id))
        .ok_or("Course not found");
    if let Ok(course) = selected_course {
        return HttpResponse::Ok().json(course);
    } else {
        return HttpResponse::Ok().json("No course found for teacher".to_string());
    }
}